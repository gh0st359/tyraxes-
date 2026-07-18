# Reporting

Skill: `/engagement-reporting` · Subagent: `reporting`

## Finding file pattern

```text
.tyraxes/redteam/findings/<id>-<slug>.md
```

Suggested sections:

- Title / ID / severity
- Affected assets
- Description
- Impact
- Reproduction steps
- Evidence links
- Remediation
- Detection ideas

## Report outputs

```text
.tyraxes/redteam/report/
  executive-summary.md
  technical-report.md
  timeline.md
```

## Rules

- No invented evidence
- Every critical claim links to an artifact or command transcript
- Separate confirmed vs suspected
