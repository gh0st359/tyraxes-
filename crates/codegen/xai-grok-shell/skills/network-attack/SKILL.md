---
name: network-attack
description: Network and Active Directory offensive methodology — discovery, creds, lateral movement, and trust abuse for authorized engagements.
---

# Network / AD Attack Playbook

## Sequence

1. Segment awareness — what can you reach from current foothold?
2. Service map — SMB, LDAP, WinRM, RDP, SSH, databases, management planes.
3. Credential material — hashes, tickets, keys, tokens, browser/OS stores (in scope).
4. Auth attacks — careful spraying, relay, AS-REP/kerberoast-class techniques when justified.
5. Lateral — admin protocols, trusts, jump hosts, VPN, cloud connectors.
6. Domain/cloud impact — prove control paths without unnecessary destruction.

## Tooling

Prefer `nmap`, `netexec`/`crackmapexec`, Impacket scripts, BloodHound collectors, and living-off-the-land Windows/Linux binaries when present.

## Discipline

- Avoid account lockouts; track attempt budgets.
- Log every privilege edge in `.grok/redteam/attack-graph.md`.
- Cleanup temporary shares/tasks unless persistence was requested.
