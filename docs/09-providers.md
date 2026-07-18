# Providers

Tyraxes speaks OpenAI-compatible HTTP. Presets ship in
`providers/presets.toml` and extract to `providers.presets.toml` under the
resolved home.

## Supported presets

| Provider | base_url | Auth |
|----------|----------|------|
| OpenAI | `https://api.openai.com/v1` | `OPENAI_API_KEY` |
| OpenRouter | `https://openrouter.ai/api/v1` | `OPENROUTER_API_KEY` |
| Ollama | `http://localhost:11434/v1` | none |
| LM Studio | `http://localhost:1234/v1` | none |
| xAI BYOK | `https://api.x.ai/v1` | `XAI_API_KEY` |

## Important fields

```toml
[model.my-model]
model = "..."
base_url = "https://..."
api_backend = "chat_completions"  # or responses / messages
env_key = "MY_API_KEY"
context_window = 128000
agent_type = "red-team"
temperature = 0.3
max_completion_tokens = 8192
extra_headers = { "X-Title" = "Tyraxes Red Team" }
```

## Choosing a backend

| Backend | When |
|---------|------|
| `chat_completions` | Broadest compatibility (Ollama, LM Studio, OpenRouter, many proxies) |
| `responses` | OpenAI Responses API models |
| `messages` | Anthropic-style message APIs exposed via compatible gateways |

## Tips

- Pin `agent_type = "red-team"` on offensive models
- Keep local model temperature ≤ 0.3
- Bound `max_completion_tokens` for local runtimes
