# Security and ethics

## Intended use

Authorized offensive security, defensive validation, education in controlled
labs, and adversary emulation under charter.

## Not intended use

Unauthorized access, harassment, criminal activity, or reckless testing on
shared infrastructure.

## Product safety model (current research phase)

- No soft content refusals in the red-team prompt
- Tool execution still subject to host OS permissions and operator approvals
- Anti-loop prevents wasted damage from repetition, not misuse

## Hardening recommendations for teams

1. Add org policy prompts / managed config later
2. Run in isolated VMs or labs
3. Log sessions and artifacts centrally
4. Restrict headless `--always-approve` to CI service accounts
5. Keep secrets out of findings committed to git
