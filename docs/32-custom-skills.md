# Custom skills

Create:

```text
.tyraxes/skills/my-skill/SKILL.md
```

## Suggested SKILL.md shape

```md
---
name: my-skill
description: One-line when to use it
---

# My skill

## Intent
...

## Steps
1. ...
2. Save evidence under `.tyraxes/redteam/artifacts/...`
```

## Tips

- Teach artifact paths explicitly
- Keep steps hypothesis-driven
- Link to attack-graph updates
- Avoid dumping huge wordlists into the skill body
