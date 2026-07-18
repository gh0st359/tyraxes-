//! Client-side assistant-text anti-loop protocols for local / weak models.
//!
//! Detects near-identical consecutive assistant messages (common Ollama /
//! LM Studio failure mode) and emits pivot reminders before the agent
//! wastes more turns narrating the same plan.

use std::collections::VecDeque;

/// Minimum normalized length before similarity checks apply.
pub const MIN_CHARS: usize = 80;
/// Consecutive near-duplicate assistant texts before nudge.
pub const DEFAULT_NUDGE_THRESHOLD: u32 = 2;
/// Consecutive near-duplicates before hard stop.
pub const DEFAULT_HARD_STOP_THRESHOLD: u32 = 3;
/// Jaccard similarity threshold over word trigrams (0.0–1.0).
pub const DEFAULT_SIMILARITY: f32 = 0.92;

#[derive(Debug, Clone, PartialEq)]
pub enum TextLoopIntervention {
    Nudge(String),
    HardStop(String),
}

#[derive(Debug, Clone)]
pub struct ResponseLoopGuard {
    recent: VecDeque<String>,
    cap: usize,
    streak: u32,
    nudge_threshold: u32,
    hard_stop_threshold: u32,
    similarity: f32,
    nudged: bool,
    hard_stopped: bool,
}

impl Default for ResponseLoopGuard {
    fn default() -> Self {
        Self::new(
            6,
            DEFAULT_NUDGE_THRESHOLD,
            DEFAULT_HARD_STOP_THRESHOLD,
            DEFAULT_SIMILARITY,
        )
    }
}

impl ResponseLoopGuard {
    pub fn new(
        cap: usize,
        nudge_threshold: u32,
        hard_stop_threshold: u32,
        similarity: f32,
    ) -> Self {
        Self {
            recent: VecDeque::with_capacity(cap.max(2)),
            cap: cap.max(2),
            streak: 0,
            nudge_threshold: nudge_threshold.max(1),
            hard_stop_threshold: hard_stop_threshold.max(2),
            similarity: similarity.clamp(0.5, 0.999),
            nudged: false,
            hard_stopped: false,
        }
    }

    /// Observe an assistant text payload (non-tool narration).
    pub fn observe(&mut self, text: &str) -> Option<TextLoopIntervention> {
        let norm = normalize_text(text);
        if norm.len() < MIN_CHARS {
            self.streak = 0;
            self.nudged = false;
            self.hard_stopped = false;
            self.push(norm);
            return None;
        }

        let similar = self
            .recent
            .back()
            .map(|prev| jaccard_trigram(prev, &norm) >= self.similarity)
            .unwrap_or(false);

        if similar {
            // Prior message counted as 1; this duplicate bumps the streak.
            self.streak = self.streak.max(1).saturating_add(1);
        } else {
            self.streak = 1;
            self.nudged = false;
            self.hard_stopped = false;
        }
        self.push(norm);

        if self.streak >= self.hard_stop_threshold && !self.hard_stopped {
            self.hard_stopped = true;
            return Some(TextLoopIntervention::HardStop(
                "ANTI-LOOP HARD STOP: your last several responses are nearly identical. \
                 Stop narrating the same plan. Execute one concrete differing action now, \
                 or ask the operator a single blocking question."
                    .into(),
            ));
        }
        if self.streak >= self.nudge_threshold && !self.nudged {
            self.nudged = true;
            return Some(TextLoopIntervention::Nudge(
                "ANTI-LOOP NUDGE: you repeated nearly the same response. Do not restate the \
                 plan — take a different next step or pivot technique."
                    .into(),
            ));
        }
        None
    }

    fn push(&mut self, norm: String) {
        self.recent.push_back(norm);
        while self.recent.len() > self.cap {
            self.recent.pop_front();
        }
    }
}

fn normalize_text(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_whitespace() {
                ' '
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn jaccard_trigram(a: &str, b: &str) -> f32 {
    let ta = trigrams(a);
    let tb = trigrams(b);
    if ta.is_empty() || tb.is_empty() {
        return if a == b { 1.0 } else { 0.0 };
    }
    let mut inter = 0usize;
    for t in &ta {
        if tb.contains(t) {
            inter += 1;
        }
    }
    let union = ta.len() + tb.len() - inter;
    if union == 0 {
        0.0
    } else {
        inter as f32 / union as f32
    }
}

fn trigrams(s: &str) -> Vec<String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.len() < 3 {
        return words.into_iter().map(str::to_string).collect();
    }
    words
        .windows(3)
        .map(|w| format!("{} {} {}", w[0], w[1], w[2]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn near_duplicate_triggers_nudge() {
        let mut g = ResponseLoopGuard::default();
        let msg = "I will scan the target network carefully and then enumerate services \
                   on interesting hosts before attempting any exploitation path at all.";
        assert!(g.observe(msg).is_none());
        match g.observe(msg) {
            Some(TextLoopIntervention::Nudge(_)) => {}
            other => panic!("expected nudge, got {other:?}"),
        }
    }

    #[test]
    fn different_text_resets() {
        let mut g = ResponseLoopGuard::default();
        let a = "I will scan the target network carefully and then enumerate services \
                 on interesting hosts before attempting any exploitation path at all.";
        let b = "Switching approach: checking web endpoints with httpx and nuclei next, \
                 because the port scan already finished with only 443 open on the perimeter.";
        assert!(g.observe(a).is_none());
        assert!(g.observe(b).is_none());
    }
}
