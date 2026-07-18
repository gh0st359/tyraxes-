# Red Team agent

The builtin agent name is `red-team` (also accepted as `red_team`).

## Identity

```toml
[agent]
name = "red-team"
system_prompt_label = "Tyraxes Red Team"
```

Env:

```bash
export TYRAXES_AGENT=red-team
# or GROK_AGENT=red-team
```

Per-model force:

```toml
agent_type = "red-team"
```

Because `red-team` is a **strict harness**, selecting a model with
`agent_type = "red-team"` rebuilds the session around that definition even if
another default agent would otherwise win — unless you explicitly override via
`[agent] name` / `TYRAXES_AGENT`.

## Prompt posture

The system prompt frames Tyraxes as an offensive operator:

- real engagements, not demos
- evidence over assertion
- scope discipline
- adaptive kill-chain thinking
- specialist delegation
- durable workspace under `.tyraxes/redteam/`

## Toolset

Curated for offense: shell, files, search, web, monitors, todos, subagents, MCP.
Default coding-assistant tool injection is disabled for harness purity.

## Skills preloaded

- recon
- adaptive-offense
- tool-arsenal
- web-app-attack
- network-attack
- privilege-escalation
- engagement-reporting

## Subagents

See [06-specialist-subagents.md](06-specialist-subagents.md).
