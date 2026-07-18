---
name: engagement-reporting
description: Red-team findings and engagement report structure — severity, evidence, reproduction, remediations, ATT&CK/OWASP mapping.
---

# Engagement Reporting

## Finding note template

Write each finding to `.grok/redteam/findings/<id>-<slug>.md`:

```markdown
# <Title>
- Severity:
- Assets:
- Evidence:
- Reproduction:
- Impact:
- Remediation:
- Mapping: (ATT&CK / OWASP / CVE)
```

## Final report

Under `.grok/redteam/report/`:

1. Executive summary (business risk, access achieved, top issues)
2. Scope and methodology
3. Timeline / kill chain narrative
4. Technical findings (ordered by severity)
5. Detection gaps and blue-team notes
6. Appendix (tools, raw artifact index)

Never invent evidence. If proof is missing, mark the item as hypothesis.
