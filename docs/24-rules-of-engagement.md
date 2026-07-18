# Rules of engagement

Tyraxes does not replace authorization. Operators do.

## Before you run anything

- Written authorization covering targets and dates
- Explicit inclusions / exclusions
- Emergency contacts
- Data handling constraints
- Allowed intensity (DoS? phishing? persistence?)

## Encode ROE in the workspace

Put it in `.tyraxes/redteam/scope.md` so every specialist reads it.

## Hard stops

- Out-of-scope host/domain discovered → stop and ask
- Production safety tripwire hit → stop
- Credential access beyond charter → stop and escalate to human

## Research posture note

The red-team harness intentionally omits soft product refusals so research
teams can layer policy later. That increases power and responsibility.
