# Configuration

## Files

| File | Role |
|------|------|
| `~/.tyraxes/config.toml` | Primary user config |
| `~/.tyraxes/providers.presets.toml` | Extracted presets (copy from) |
| `~/.tyraxes/managed_config.toml` | Managed overlays |
| `/etc/tyraxes/*` | System overlays (Unix) |
| `.tyraxes/` in project | Project agents/skills/rules |

Legacy `.grok` / `/etc/grok` paths remain valid.

## Minimal red-team config

```toml
[agent]
name = "red-team"
system_prompt_label = "Tyraxes Red Team"

[model.ollama-local]
model = "qwen2.5-coder:32b"
base_url = "http://localhost:11434/v1"
api_backend = "chat_completions"
context_window = 32768
agent_type = "red-team"
temperature = 0.2
```

## Compat cells

Vendor discovery for Cursor/Claude/Codex skills/rules/agents/mcps can be
toggled under `[compat]` (defaults on). Tyraxes/Grok dirs are always scanned.

## Inspect

```sh
tyraxes inspect
tyraxes inspect --json
```
