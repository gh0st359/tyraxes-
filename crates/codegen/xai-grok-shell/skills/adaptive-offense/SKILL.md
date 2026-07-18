---
name: adaptive-offense
description: Creative and adaptive offensive thinking — hypothesis loops, attack graphs, pivots, and chain building when blocked.
---

# Adaptive Offense

Use when stuck, when findings are weak alone, or when you need a deliberate pivot.

## Thinking loop (every non-trivial step)

1. **Objective** — what decision does this unlock?
2. **Hypotheses** — ranked and falsifiable.
3. **Cheapest test** — kill the largest uncertainty first.
4. **Branches** — if A fails, B and C are already named.
5. **Noise** — detection risk, rate limits, collateral.
6. **Pivot layer** — host ↔ service ↔ identity ↔ network ↔ app ↔ cloud ↔ people/process.
7. **Evidence** — artifact that proves the edge exists.

## Chain bias

Prefer chains over isolated mediums:

- info leak → auth weakness → privileged action → impact
- CI secret → deploy creds → cloud role → data store
- misconfigured SSO → session → admin SaaS → production access

## When blocked

- Change **layer**, not just flags.
- Spawn `vuln-triage` or `exploit-dev` with a crisp brief.
- Update `.tyraxes/redteam/attack-graph.md` before retrying noise.
