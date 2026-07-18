# Grok Red Team

Grok ships a first-class **offensive security terminal agent** (`red-team`) on
top of the same TUI/CLI you already use. It keeps the full-screen UX, slash
commands, model picker, MCP, skills, and headless modes — and re-orients the
agent loop toward authorized red-team, pentest, and adversary-emulation work.

---

## Activate

### Config (persistent)

```toml
[agent]
name = "red-team"
system_prompt_label = "Grok Red Team"
```

### Environment

```bash
export GROK_AGENT=red-team
grok
```

### Per-model

```toml
[model.ollama-local]
model = "qwen2.5-coder:32b"
base_url = "http://localhost:11434/v1"
agent_type = "red-team"
context_window = 32768
```

Provider presets (OpenAI, OpenRouter, Ollama, LM Studio) are extracted to
`~/.grok/providers.presets.toml` on startup — copy the blocks you need into
`~/.grok/config.toml`.

### Headless

```bash
GROK_AGENT=red-team grok -p "Map the attack surface for 10.0.0.0/24 — authorized lab" \
  -m ollama-local --always-approve
```

---

## What you get

| Surface | Behavior |
|---------|----------|
| System prompt | Full offensive-security operator prompt (methodology, adaptive thinking, evidence discipline) |
| Toolset | Shell, files, search, web, monitors, todos, subagents, MCP discovery |
| Subagents | `recon`, `vuln-triage`, `exploit-dev`, `reporting` (+ stock `general-purpose` / `explore` / `plan`) |
| Skills | `/recon`, `/adaptive-offense`, `/tool-arsenal`, `/web-app-attack`, `/network-attack`, `/privilege-escalation`, `/engagement-reporting` |
| Workspace | `.grok/redteam/{scope,attack-graph,findings,artifacts,notes,report}` |
| Anti-loop | Client-side repeated tool-call + narration guards (critical for local models) |

---

## Providers

| Provider | Typical `base_url` | Auth |
|----------|-------------------|------|
| OpenAI | `https://api.openai.com/v1` | `OPENAI_API_KEY` |
| OpenRouter | `https://openrouter.ai/api/v1` | `OPENROUTER_API_KEY` + optional `extra_headers` |
| Ollama | `http://localhost:11434/v1` | none |
| LM Studio | `http://localhost:1234/v1` | none |
| xAI | `https://api.x.ai/v1` | `XAI_API_KEY` |

See [Custom Models](11-custom-models.md) for the full BYOK schema.

---

## Anti-loop (local models)

Local models can spiral. Grok Red Team enables client-side guards for all
backends:

1. **Identical tool calls** — same tool + normalized args → nudge, then hard stop
2. **Near-identical narration** — repeated assistant text without tools → nudge / hard stop
3. **Prompt discipline** — the red-team system prompt forbids blind retries

Server-side doom-loop recovery (xAI Responses) remains available when configured.

---

## Research posture

The `red-team` agent uses `bypassPermissions` in its definition and a full
prompt without the coding-agent soft refusal block. Your team can layer
hooks, managed policy, sandbox profiles, and allow/deny rules later.

Always operate only against systems you are authorized to test.
