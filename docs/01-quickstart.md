# Quickstart

## 1. Build

```sh
cargo build -p xai-grok-pager-bin --release
./target/release/tyraxes --version
```

## 2. Activate red-team

```sh
export TYRAXES_AGENT=red-team
```

Or in `~/.tyraxes/config.toml`:

```toml
[agent]
name = "red-team"
system_prompt_label = "Tyraxes Red Team"
```

## 3. Configure a model

On first run, Tyraxes extracts provider presets to
`~/.tyraxes/providers.presets.toml`. Copy a block into `config.toml`.

**Ollama example:**

```toml
[model.ollama-local]
model = "qwen2.5-coder:32b"
base_url = "http://localhost:11434/v1"
api_backend = "chat_completions"
context_window = 32768
agent_type = "red-team"
temperature = 0.2
```

## 4. Launch

```sh
tyraxes -m ollama-local
```

Then:

```text
Confirm scope for 10.10.0.0/24 lab, write .tyraxes/redteam/scope.md, start recon.
```

## 5. Headless smoke

```sh
TYRAXES_AGENT=red-team tyraxes -p "Create .tyraxes/redteam/scope.md for authorized lab 10.10.0.0/24"   -m ollama-local --always-approve
```

## Next

- [05-red-team-agent.md](05-red-team-agent.md)
- [16-engagement-workflow.md](16-engagement-workflow.md)
- [11-anti-loop.md](11-anti-loop.md)
