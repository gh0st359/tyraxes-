# MCP integration

Tyraxes can discover and call MCP servers, letting you attach specialized
offensive tooling without forking the agent.

## Use cases

- Custom scanner wrappers
- Case management / ticketing bridges
- Internal exploit libraries (your org’s)
- Evidence uploaders
- Cloud control-plane helpers

## Configuration

Use the existing MCP management commands:

```sh
tyraxes mcp --help
```

Project and user MCP configs follow the same discovery/compat rules as the
rest of the CLI (Cursor/Claude cells optional).

## Operator guidance

- Keep dangerous MCP tools behind clear names and logging
- Prefer MCP for structured tools; keep ad-hoc shell for living-off-the-land
- Record MCP outputs into `.tyraxes/redteam/artifacts/`
