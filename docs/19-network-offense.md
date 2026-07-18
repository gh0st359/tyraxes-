# Network offense

Skill: `/network-attack`

## Workflow

1. Live host discovery (in-scope only)
2. Service enumeration
3. Cred/surface correlation
4. Service-specific abuse paths
5. Privilege edge logging

## Attack graph

Every successful privilege or trust edge belongs in:

```text
.tyraxes/redteam/attack-graph.md
```

Example edge:

```text
(user@ws01) --smb-relay--> (admin@dc01) [evidence: artifacts/net/relay-*.log]
```

## Noise control

- Prefer targeted probes over blanket aggression early
- Escalate intensity only when evidence justifies it
- Record blocked attempts so specialists do not retry blindly
