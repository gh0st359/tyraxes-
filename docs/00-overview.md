# Overview

Tyraxes is a terminal-native **offensive security agent**. It is not a chatbot
wrapper around a scanner. It is an operator loop with tools, memory, skills,
specialist subagents, and durable engagement workspaces.

## Design pillars

1. **Operator-first** — plan → act → evidence → adapt.
2. **Provider-agnostic** — any OpenAI-compatible endpoint (cloud or local).
3. **Harness-strict for red-team** — curated toolset and full prompt mode.
4. **Anti-loop for local models** — client-side fingerprints stop spirals.
5. **Evidence durable** — findings and artifacts live on disk under `.tyraxes/redteam/`.
6. **Safe surface rebrand** — customer name Tyraxes; internal crates stay stable.

## What Tyraxes can do

- Scope and document authorized engagements
- Recon and attack-surface mapping
- Vulnerability triage and hypothesis-driven exploitation
- Post-exploitation assessment and lateral-path analysis
- Privilege-escalation workflows
- Professional reporting with evidence links
- Parallel specialist delegation (`recon`, `vuln-triage`, `exploit-dev`, `reporting`)
- MCP tool discovery for custom offensive tooling

## What Tyraxes is not

- A replacement for written authorization / ROE
- A guaranteed exploit factory
- A simulation playground (it executes real commands you approve)

## Primary surfaces

| Surface | Entry |
|---------|-------|
| Interactive TUI | `tyraxes` |
| Headless prompt | `tyraxes -p "..."` |
| Agent ACP/stdio | `tyraxes agent stdio` |
| Config | `~/.tyraxes/config.toml` |
| Presets | `~/.tyraxes/providers.presets.toml` |
