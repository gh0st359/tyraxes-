# Skills

Skills are slash-invocable methodology packs bundled with Tyraxes.

| Skill | Purpose |
|-------|---------|
| `/recon` | Scoped recon playbook |
| `/adaptive-offense` | Hypothesis → test → adapt loop |
| `/tool-arsenal` | Prefer installed tools; capture raw artifacts |
| `/web-app-attack` | Web app attack surface workflow |
| `/network-attack` | Network/service offense workflow |
| `/privilege-escalation` | Local/privesc assessment |
| `/engagement-reporting` | Finding + report templates |

## Discovery order

Project/user skill dirs (preferred first):

1. `.tyraxes/skills`
2. `.grok/skills` (legacy)
3. `.agents`, `.claude`, `.cursor` (compat cells)

User home skills under the resolved Tyraxes/Grok home are also loaded.

## Authoring

See [32-custom-skills.md](32-custom-skills.md).
