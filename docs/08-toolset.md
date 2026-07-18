# Toolset

Tyraxes Red Team uses a curated toolset rather than the full default coding
injection set.

## Categories

| Kind | Typical use in offense |
|------|------------------------|
| execute / shell | scanners, exploit runners, cloud CLIs, living-off-the-land |
| read / edit / search / list | notes, configs, loot parsing, script iteration |
| web_search / web_fetch | CVE research, advisories, technique refresh |
| monitor | listeners, tunnels, long-running watchers |
| plan / todos | kill-chain tracking |
| subagent | specialist delegation |
| MCP | external offensive tool servers |

## Guidance baked into the prompt

- Prefer specialized file tools over shell for file ops
- Never use `echo`/`printf` as the operator channel
- Save raw tool output under `.tyraxes/redteam/artifacts/`
- Update attack graph before noisy retries

## Extending tools

- Project MCP servers (see [15-mcp-integration.md](15-mcp-integration.md))
- Custom skills that teach tool usage patterns
- Custom agents with alternate tool allowlists
