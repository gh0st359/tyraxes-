# CLI reference

```sh
tyraxes --help
```

## High-value entrypoints

| Command | Use |
|---------|-----|
| `tyraxes` | Interactive TUI |
| `tyraxes -p TEXT` | Headless prompt |
| `tyraxes -m MODEL` | Select configured model |
| `tyraxes models` | List models |
| `tyraxes inspect` | Show discovered config |
| `tyraxes mcp ...` | MCP management |
| `tyraxes sessions ...` | Session management |
| `tyraxes agent stdio` | ACP stdio server |
| `tyraxes login` / `logout` | Auth flows (when applicable) |

Compatibility: `grok` accepts the same command surface.
