# Evidence discipline

Tyraxes is only as trustworthy as its artifacts.

## Principles

1. Raw output is sacred — save it before summarizing
2. Chat is ephemeral — disk is durable
3. Claims need pointers
4. Timestamps matter
5. Suspected ≠ confirmed

## Suggested tree

```text
.tyraxes/redteam/
  scope.md
  attack-graph.md
  artifacts/<phase>/
  notes/
  findings/
  report/
```

## Operator prompts that help

```text
Do not summarize until raw outputs are saved under artifacts/.
```

```text
For each finding, link the exact artifact path and command.
```
