---
name: recon
description: Authorized reconnaissance methodology — asset discovery, attack-surface mapping, and evidence capture for red-team engagements.
---

# Reconnaissance Playbook

Use this skill for scoped recon. Prefer installed tools; save evidence under `.tyraxes/redteam/`.

## Goals

1. Confirm scope, exclusions, and success criteria in `.tyraxes/redteam/scope.md`.
2. Build an asset inventory (domains, IPs, hosts, apps, cloud accounts, identities).
3. Fingerprint services and expose ranked entry points.
4. Hand off hypotheses — do not stop at raw scan dumps.

## Workflow

1. **Passive first** — DNS, certificates, public metadata, repo/org intel in scope.
2. **Active mapping** — host discovery, port/service scans with rate discipline.
3. **Service depth** — banners, TLS, HTTP titles/tech, API schemas, shares.
4. **Normalize** — write `.tyraxes/redteam/notes/recon-inventory.md` as a table.
5. **Rank** — top 5–15 entry points with why they matter.

## Tool patterns

```bash
command -v nmap httpx nuclei ffuf naabu 2>/dev/null
mkdir -p .tyraxes/redteam/artifacts/recon .tyraxes/redteam/notes
# Start narrow; widen only with evidence. Save every raw output.
```

## Anti-loop

- Do not re-run identical scans after success or hard failure.
- Change ports, wordlists, timing, or auth context — or pivot technique.
- After three fruitless attempts on one micro-goal, escalate or reframe.
