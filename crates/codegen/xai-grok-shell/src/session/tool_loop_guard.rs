//! Client-side tool-call anti-loop protocols.
//!
//! Local models (Ollama, LM Studio, small OpenRouter models) often repeat the
//! same tool call with identical arguments. Server-side doom-loop detection
//! only covers xAI Responses thinking streams. This guard fingerprints tool
//! calls across a sliding window and emits escalating interventions.

use std::collections::VecDeque;

use serde_json::Value;

/// Default consecutive identical tool-call fingerprint threshold before nudge.
pub const DEFAULT_REPEAT_NUDGE_THRESHOLD: u32 = 2;
/// Default threshold before hard-stop followup (includes the nudge attempt).
pub const DEFAULT_REPEAT_HARD_STOP_THRESHOLD: u32 = 4;
/// Sliding window of recent fingerprints retained for near-repeat detection.
pub const DEFAULT_WINDOW: usize = 24;
/// Near-repeat: same tool name dominating the recent window.
pub const DEFAULT_SAME_TOOL_DOMINANCE: u32 = 6;

/// Intervention produced when a loop is detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopIntervention {
    /// Soft system reminder — model should pivot.
    Nudge(String),
    /// Hard stop — inject followup that ends the fruitless micro-loop.
    HardStop(String),
}

/// Sliding-window fingerprint tracker for tool calls.
#[derive(Debug, Clone)]
pub struct ToolLoopGuard {
    window: VecDeque<String>,
    window_cap: usize,
    /// Consecutive count for the current fingerprint streak.
    streak_fingerprint: Option<String>,
    streak_count: u32,
    nudge_threshold: u32,
    hard_stop_threshold: u32,
    same_tool_dominance: u32,
    /// Nudges already issued for the current streak (avoid spam).
    nudged_for_streak: bool,
    hard_stopped_for_streak: bool,
}

impl Default for ToolLoopGuard {
    fn default() -> Self {
        Self::new(
            DEFAULT_WINDOW,
            DEFAULT_REPEAT_NUDGE_THRESHOLD,
            DEFAULT_REPEAT_HARD_STOP_THRESHOLD,
            DEFAULT_SAME_TOOL_DOMINANCE,
        )
    }
}

impl ToolLoopGuard {
    pub fn new(
        window_cap: usize,
        nudge_threshold: u32,
        hard_stop_threshold: u32,
        same_tool_dominance: u32,
    ) -> Self {
        Self {
            window: VecDeque::with_capacity(window_cap.max(1)),
            window_cap: window_cap.max(1),
            streak_fingerprint: None,
            streak_count: 0,
            nudge_threshold: nudge_threshold.max(1),
            hard_stop_threshold: hard_stop_threshold.max(2),
            same_tool_dominance: same_tool_dominance.max(3),
            nudged_for_streak: false,
            hard_stopped_for_streak: false,
        }
    }

    /// Observe a tool call. Returns an intervention when a loop is detected.
    pub fn observe(&mut self, tool_name: &str, arguments_json: &str) -> Option<LoopIntervention> {
        let fp = fingerprint(tool_name, arguments_json);
        if self.streak_fingerprint.as_deref() == Some(fp.as_str()) {
            self.streak_count = self.streak_count.saturating_add(1);
        } else {
            self.streak_fingerprint = Some(fp.clone());
            self.streak_count = 1;
            self.nudged_for_streak = false;
            self.hard_stopped_for_streak = false;
        }

        self.window.push_back(fp.clone());
        while self.window.len() > self.window_cap {
            self.window.pop_front();
        }

        if self.streak_count >= self.hard_stop_threshold && !self.hard_stopped_for_streak {
            self.hard_stopped_for_streak = true;
            return Some(LoopIntervention::HardStop(format!(
                "ANTI-LOOP HARD STOP: tool `{tool_name}` was called {} times with the same \
                 arguments. Stop repeating this call. Change technique, parameters, target, \
                 or spawn a specialist subagent. Summarize what failed and the next differing action.",
                self.streak_count
            )));
        }

        if self.streak_count >= self.nudge_threshold && !self.nudged_for_streak {
            self.nudged_for_streak = true;
            return Some(LoopIntervention::Nudge(format!(
                "ANTI-LOOP NUDGE: you repeated `{tool_name}` with identical arguments \
                 {} times. Do not retry the same call. Pivot: change flags/inputs, try a \
                 different tool, or reframe the micro-goal.",
                self.streak_count
            )));
        }

        if let Some(msg) = self.same_tool_dominance_nudge(tool_name) {
            return Some(LoopIntervention::Nudge(msg));
        }

        None
    }

    fn same_tool_dominance_nudge(&self, tool_name: &str) -> Option<String> {
        let prefix = format!("{tool_name}|");
        let count = self
            .window
            .iter()
            .filter(|fp| fp.starts_with(&prefix))
            .count() as u32;
        if count >= self.same_tool_dominance && self.streak_count < self.nudge_threshold {
            Some(format!(
                "ANTI-LOOP NUDGE: `{tool_name}` dominates the recent tool window ({count} calls). \
                 Diversify — combine findings, switch phase, or delegate to a specialist."
            ))
        } else {
            None
        }
    }

    /// Reset streak state (e.g. after a successful phase change / user interrupt).
    pub fn reset_streak(&mut self) {
        self.streak_fingerprint = None;
        self.streak_count = 0;
        self.nudged_for_streak = false;
        self.hard_stopped_for_streak = false;
    }
}

/// Normalize tool name + args into a stable fingerprint.
pub fn fingerprint(tool_name: &str, arguments_json: &str) -> String {
    let normalized_args = normalize_args(arguments_json);
    format!("{tool_name}|{normalized_args}")
}

fn normalize_args(arguments_json: &str) -> String {
    let trimmed = arguments_json.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    match serde_json::from_str::<Value>(trimmed) {
        Ok(Value::Object(map)) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut out = String::new();
            for k in keys {
                if ignore_arg_key(k) {
                    continue;
                }
                let v = canonicalize(&map[k]);
                out.push_str(k);
                out.push('=');
                out.push_str(&v);
                out.push(';');
            }
            out
        }
        Ok(other) => canonicalize(&other),
        Err(_) => trimmed.to_string(),
    }
}

fn ignore_arg_key(key: &str) -> bool {
    matches!(
        key,
        "task_id" | "tool_use_id" | "timeout" | "timeout_ms" | "timeout_secs"
    )
}

fn canonicalize(value: &Value) -> String {
    match value {
        Value::String(s) => {
            let t = s.trim();
            // Collapse insignificant whitespace in shell-ish commands.
            if t.contains('\n') || t.contains("  ") {
                t.split_whitespace().collect::<Vec<_>>().join(" ")
            } else {
                t.to_string()
            }
        }
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".into(),
        Value::Array(arr) => {
            let parts: Vec<String> = arr.iter().map(canonicalize).collect();
            format!("[{}]", parts.join(","))
        }
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let parts: Vec<String> = keys
                .into_iter()
                .map(|k| format!("{k}={}", canonicalize(&map[k])))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_args_trigger_nudge_then_hard_stop() {
        let mut guard = ToolLoopGuard::new(16, 2, 4, 10);
        let args = r#"{"command":"nmap -sV 10.0.0.1"}"#;
        assert!(guard.observe("run_terminal_command", args).is_none());
        match guard.observe("run_terminal_command", args) {
            Some(LoopIntervention::Nudge(msg)) => assert!(msg.contains("ANTI-LOOP NUDGE")),
            other => panic!("expected nudge, got {other:?}"),
        }
        assert!(guard.observe("run_terminal_command", args).is_none()); // already nudged
        match guard.observe("run_terminal_command", args) {
            Some(LoopIntervention::HardStop(msg)) => assert!(msg.contains("HARD STOP")),
            other => panic!("expected hard stop, got {other:?}"),
        }
    }

    #[test]
    fn whitespace_normalized_shell_args_match() {
        let a = fingerprint("bash", r#"{"command":"ls  -la\n"}"#);
        let b = fingerprint("bash", r#"{"command":"ls -la"}"#);
        assert_eq!(a, b);
    }

    #[test]
    fn different_args_reset_streak() {
        let mut guard = ToolLoopGuard::default();
        assert!(guard
            .observe("run_terminal_command", r#"{"command":"id"}"#)
            .is_none());
        assert!(guard
            .observe("run_terminal_command", r#"{"command":"id"}"#)
            .is_some()); // nudge
        assert!(guard
            .observe("run_terminal_command", r#"{"command":"whoami"}"#)
            .is_none());
    }
}
