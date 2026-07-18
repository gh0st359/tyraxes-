---
name: web-app-attack
description: Web and API offensive methodology — recon, auth, injection, IDOR, SSRF, JWT, GraphQL, and evidence-backed exploitation.
---

# Web / API Attack Playbook

## Sequence

1. Map attack surface — hosts, vhosts, paths, APIs, auth mechanisms.
2. Fingerprint — frameworks, WAF, CDN, language, API style (REST/GraphQL/gRPC-web).
3. Authn/authz — login flows, SSO, MFA gaps, session fixation, JWT/JWKS issues.
4. Input attacks — injection, SSTI, path traversal, deserialization (targeted).
5. Access control — IDOR, privilege mismatch, broken function-level auth.
6. Server-side — SSRF, request smuggling candidates, upload abuse.
7. Prove impact with minimal PoC; capture request/response evidence.

## Tooling

```bash
httpx -l hosts.txt -title -tech-detect -status-code -o .tyraxes/redteam/artifacts/web/httpx.txt
nuclei -l alive.txt -o .tyraxes/redteam/artifacts/web/nuclei.txt
ffuf -u https://target/FUZZ -w wordlist.txt -o .tyraxes/redteam/artifacts/web/ffuf.json
```

## Notes

- Prefer authenticated testing when creds are in scope.
- Rate-limit and respect fragile environments.
- Record exact HTTP transcripts for reproduction.
