# Workspace layout

## Engagement workspace (project-local)

```text
.tyraxes/redteam/
  scope.md
  attack-graph.md
  findings/
  artifacts/
    recon/
    web/
    net/
    privesc/
  notes/
  report/
```

## Project config

```text
.tyraxes/
  agents/
  skills/
  rules/
```

Legacy `.grok/` equivalents still load.

## User home

```text
~/.tyraxes/
  config.toml
  providers.presets.toml
  agents/
  skills/
  auth.json          # if using auth flows
  ...
```
