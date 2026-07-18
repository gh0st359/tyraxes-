# Operator playbook

A practical day-one script for humans driving Tyraxes.

## Minute 0–10

1. Confirm ROE doc exists
2. Launch `TYRAXES_AGENT=red-team tyraxes -m <model>`
3. Ask agent to write `scope.md` from ROE
4. Create workspace dirs if missing

## Minute 10–60

1. `/recon` on primary perimeter
2. Normalize inventory
3. Pick top 3 hypotheses with `vuln-triage`
4. Update attack graph

## Deep work blocks

- One hypothesis per block
- Save raw artifacts before branching
- If anti-loop fires twice, change strategy or specialist

## Closeout

1. `/engagement-reporting`
2. Deduplicate findings
3. Executive summary last
4. Redact secrets from any shareable bundle
