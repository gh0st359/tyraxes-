# Engagement workflow

Recommended operating model for Tyraxes Red Team:

```text
Authorize → Scope → Recon → Triage → Exploit → Post-Ex → Report
                ↘──────── attack-graph updates ────────↗
```

## Phase checklist

### 1. Authorize
Confirm written permission, contacts, windows, and exclusions.

### 2. Scope
Write `.tyraxes/redteam/scope.md` before scanning.

### 3. Recon
Use `/recon`. Normalize inventory tables. Save raw outputs.

### 4. Triage
Delegate `vuln-triage`. Rank hypotheses by impact × likelihood × effort.

### 5. Exploit
Delegate `exploit-dev` only for ranked hypotheses. Capture PoCs + evidence.

### 6. Post-exploitation
Map privilege edges in `attack-graph.md`. Avoid unnecessary persistence.

### 7. Report
Use `/engagement-reporting`. Link every claim to artifacts.

## Cadence

- Update attack graph whenever a path changes
- Prefer small parallel specialists over one overloaded parent turn
- Stop at scope boundaries immediately
