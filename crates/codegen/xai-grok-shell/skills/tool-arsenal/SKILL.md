---
name: tool-arsenal
description: Offensive tooling inventory and usage patterns — discover installed scanners/exploit helpers and run them with evidence discipline.
---

# Tool Arsenal

## Discover first

```bash
for t in nmap masscan rustscan naabu httpx nuclei ffuf feroxbuster gobuster katana \
  sqlmap nikto netexec crackmapexec impacket-smbclient bloodhound-python hashcat john \
  responder trufflehog gitleaks semgrep aws gcloud az kubectl pacu prowler trivy \
  chisel ligolo-ng socat jq python3; do
  command -v "$t" >/dev/null && echo "OK $t"
done
```

## Usage contract

1. Confirm binary exists and note version.
2. Start with informational / low-impact modes.
3. Save raw output under `.tyraxes/redteam/artifacts/<phase>/`.
4. Parse into notes/findings; update todos.
5. Prefer one sharp command over speculative flag spam.

## Categories

| Category | Examples |
|----------|----------|
| Network | nmap, naabu, masscan, netexec |
| Web/API | httpx, nuclei, ffuf, sqlmap, katana |
| Identity | impacket, bloodhound-python, responder |
| Secrets/code | trufflehog, gitleaks, semgrep, rg |
| Cloud/K8s | aws/az/gcloud, pacu, prowler, kubectl, trivy |
| Pivots | chisel, ligolo, ssh, socat |
