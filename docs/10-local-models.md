# Local models

Tyraxes is designed to stay productive with Ollama / LM Studio.

## Recommended defaults

```toml
temperature = 0.2
context_window = 32768   # honest to the model
max_completion_tokens = 8192
agent_type = "red-team"
```

## Why local models fail in agent loops

- Weak tool-call discipline
- Repetition / mode collapse
- Over-narration without action
- Invented command output

## Mitigations Tyraxes applies

1. Strict red-team harness + methodology skills
2. Tool-loop fingerprint guard
3. Response-loop narration guard
4. Durable disk workspace so progress survives chat noise

## Operator tips

- Prefer larger instruct/coder models for tool use
- Keep prompts short and scoped per turn
- Force artifact writes early (`scope.md`, inventory tables)
- Delegate specialists instead of one mega-prompt
- Stop and restate scope when the model drifts
