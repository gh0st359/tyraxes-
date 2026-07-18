# Environment variables

## Tyraxes-preferred

| Variable | Purpose |
|----------|---------|
| `TYRAXES_HOME` | Config home override |
| `TYRAXES_AGENT` | Agent selection (`red-team`, path, or name) |

## Compatibility (still honored)

| Variable | Purpose |
|----------|---------|
| `GROK_HOME` | Legacy config home |
| `GROK_AGENT` | Legacy agent selection |
| `GROK_AGENT_SECRET` | Agent secret (existing CLI) |
| `GROK_*` auth/proxy vars | Existing auth integrations |

## Provider keys

| Variable | Provider |
|----------|----------|
| `OPENAI_API_KEY` | OpenAI |
| `OPENROUTER_API_KEY` | OpenRouter |
| `XAI_API_KEY` | xAI |

## Resolution notes

- `TYRAXES_AGENT` wins over `GROK_AGENT` when both are set
- `TYRAXES_HOME` wins over `GROK_HOME`
- Existing `~/.grok` is reused if `~/.tyraxes` does not exist yet
