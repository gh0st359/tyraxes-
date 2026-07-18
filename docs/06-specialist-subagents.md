# Specialist subagents

Tyraxes Red Team exposes offensive specialists in addition to stock helpers.

| Name | Mission |
|------|---------|
| `recon` | Scope confirmation, enumeration, inventory normalization |
| `vuln-triage` | Hypothesis ranking, false-positive reduction, severity |
| `exploit-dev` | PoC crafting, reliability, constraints, evidence |
| `reporting` | Finding writeups, executive + technical report assembly |
| `general-purpose` | Stock flexible worker |
| `explore` | Stock codebase/host exploration |
| `plan` | Stock planning helper |

## How to use them

Ask the parent agent to delegate:

```text
Spawn recon against the web perimeter; return inventory + open questions.
```

Or rely on the prompt’s delegation rules when the parent decides parallelism
is cheaper than serial thrash.

## Shared state

Specialists should read/write the engagement workspace:

```text
.tyraxes/redteam/
  scope.md
  attack-graph.md
  findings/
  artifacts/
  notes/
  report/
```

Do not keep critical evidence only in chat history.
