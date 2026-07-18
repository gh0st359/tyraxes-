# Headless mode

Run Tyraxes without the TUI for labs, CI, and scripting.

```sh
TYRAXES_AGENT=red-team tyraxes -p "..." -m ollama-local --always-approve
```

## Common flags

| Flag | Meaning |
|------|---------|
| `-p` / `--prompt` | One-shot prompt |
| `-m` / `--model` | Model id from config |
| `--always-approve` | Auto-approve tool use (trusted envs only) |
| `agent stdio` | ACP stdio protocol mode |

## Patterns

### Scoped recon job

```sh
TYRAXES_AGENT=red-team tyraxes -p \
  "Authorized lab. Write scope.md then produce recon inventory for 10.0.0.0/24." \
  -m ollama-local --always-approve
```

### Report assembly

```sh
TYRAXES_AGENT=red-team tyraxes -p \
  "Read .tyraxes/redteam/findings and draft technical report under report/" \
  -m openrouter-sonnet --always-approve
```

## Safety

Headless + always-approve is powerful. Restrict to isolated labs, containers,
or explicitly authorized automation.
