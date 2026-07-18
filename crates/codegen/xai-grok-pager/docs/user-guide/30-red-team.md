# Tyraxes Red Team

Tyraxes ships a first-class **offensive security terminal agent** (`red-team`) on
top of the same TUI/CLI you already use. It keeps the full-screen UX, slash
commands, model picker, MCP, skills, and headless modes — and re-orients the
agent loop toward authorized red-team, pentest, and adversary-emulation work.

See the full product documentation under [`docs/`](../../../../../docs/) at the
repository root.

---

## Activate

### Config (persistent)

```toml
[agent]
name = "red-team"
system_prompt_label = "Tyraxes Red Team"
```

### Environment

```bash
export TYRAXES_AGENT=red-team   # preferred
# export GROK_AGENT=red-team    # compatibility alias
tyraxes
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
`~/.tyraxes/providers.presets.toml` on startup (or `~/.grok/…` for legacy
homes) — copy the blocks you need into `config.toml`.

### Headless

```bash
TYRAXES_AGENT=red-team tyraxes -p "Map the attack surface for 10.0.0.0/24 — authorized lab" \
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
| Workspace | `.tyraxes/redteam/{scope,attack-graph,findings,artifacts,notes,report}` |
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

Local models can spiral. Tyraxes Red Team enables client-side guards for all
backends:

1. **Tool-loop guard** — identical tool + args fingerprints nudge, then hard-stop.
2. **Response-loop guard** — near-duplicate assistant narration nudges, then hard-stop.

Tune temperature down (0.1–0.3) and keep `max_completion_tokens` bounded for
local models. Details: [`docs/11-anti-loop.md`](../../../../../docs/11-anti-loop.md).

---

## Engagement workspace

Create and maintain:

```text
.tyraxes/redteam/
  scope.md
  attack-graph.md
  findings/
  artifacts/
  notes/
  report/
```

Use skills (`/recon`, `/engagement-reporting`, …) to keep evidence durable across
turns and subagents.
