# FAQ

### Is Tyraxes just a rename of Grok Build?
Customer-facing yes (surface rebrand). Internally crates remain `xai-grok-*`
and legacy paths/env still work.

### Does it include exploit malware payloads?
No. It orchestrates tools, methodology, and evidence. You bring authorized
tooling via shell/MCP.

### Will local models work?
Yes, with presets and anti-loop. Larger instruct/coder models work best.

### Can I keep using `grok`?
Yes. It is built as a compatibility binary.

### Where do findings go?
`.tyraxes/redteam/findings/` by convention.

### How do I force red-team mode?
`TYRAXES_AGENT=red-team`, `[agent] name = "red-team"`, or per-model
`agent_type = "red-team"`.
