# Extending agents

## Project agents

```text
.tyraxes/agents/my-specialist.md
```

Legacy: `.grok/agents/`.

## Frontmatter essentials

Custom agents are markdown definitions discovered by name. Mirror builtin
patterns: prompt body, tool allowlists, permission mode, skills.

## When to fork vs skill

| Need | Prefer |
|------|--------|
| Short methodology | Skill |
| Different tool policy / prompt identity | Agent |
| One-off checklist | Skill or note file |

## Discovery priority

Project `.tyraxes/agents` → `.grok/agents` → `.claude/agents` → user home → builtins.
