# Attack graph

The attack graph is the engagement’s living map of privilege and trust.

## File

```text
.tyraxes/redteam/attack-graph.md
```

## What to record

- Nodes: identities, hosts, apps, roles, secrets
- Edges: auth paths, exploits, relays, token reuse, CI abuse
- Evidence pointers
- Blocked / failed edges (so loops do not repeat)

## Example

```md
# Attack graph

## Nodes
- ext:www
- ident:svc-deploy
- host:runner-3
- role:prod-admin

## Edges
- ext:www --rce--> host:runner-3 (findings/001-rce.md)
- host:runner-3 --ci-token--> ident:svc-deploy (artifacts/ci/token.txt)
- ident:svc-deploy --assume--> role:prod-admin (blocked: MFA)
```

## Cadence

Update before noisy retries and after every meaningful foothold.
