# Telemetry and artifacts

## Session telemetry

The runtime may emit local debug/telemetry depending on build and config.
Treat engagement evidence as separate from product telemetry:

- product logs ≠ customer report evidence
- always keep raw command outputs in `artifacts/`

## Artifact hygiene

| Do | Don’t |
|----|-------|
| timestamp directories | overwrite the only copy of raw output |
| redact secrets before sharing | commit API keys to git |
| link findings → artifacts | rely on chat scrollback |
| compress large pcaps/logs | paste megabytes into findings |

## Suggested closeout bundle

```text
.tyraxes/redteam/
  scope.md
  attack-graph.md
  findings/
  report/
  artifacts/   # maybe sampled / redacted for export
```
