# Web application offense

Skill: `/web-app-attack`

## Workflow

1. Enumerate hosts/vhosts and alive HTTP(S)
2. Tech detect + content discovery
3. Auth/session mapping
4. Input surface mapping
5. Hypothesis tests (injection, authz, SSRF, desync, etc.)
6. Evidence + replayable PoC

## Artifact suggestions

```text
.tyraxes/redteam/artifacts/web/
  httpx.txt
  nuclei.txt
  ffuf.json
  notes.md
```

## Discipline

- Separate unauthenticated vs authenticated testing
- Track role assumptions in findings
- Prefer deterministic reproduction steps
- Update attack graph when a web foothold unlocks network/cloud pivots
