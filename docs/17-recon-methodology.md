# Recon methodology

Skill: `/recon` · Subagent: `recon`

## Goals

1. Confirm scope
2. Discover assets in-scope
3. Fingerprint services/tech
4. Normalize an inventory
5. Produce next-test hypotheses

## Evidence layout

```text
.tyraxes/redteam/
  scope.md
  artifacts/recon/
  notes/recon-inventory.md
```

## Good inventory columns

| Field | Example |
|-------|---------|
| Asset | `10.0.0.12` / `app.lab.local` |
| Ports/Services | `443/https nginx` |
| Tech | `Next.js`, `Okta` |
| Exposure | external / internal |
| Notes | WAF?, auth portal? |
| Next tests | nuclei tags, auth flows |

## Anti-patterns

- Scanning out-of-scope ranges “just in case”
- Keeping results only in chat
- Rerunning identical scans without a new hypothesis
