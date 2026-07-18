# Tyraxes Documentation

Tyraxes is an **agentic offensive-security terminal agent**: a full-screen TUI/CLI
that plans and executes authorized red-team, penetration-testing, and
adversary-emulation engagements end-to-end.

This folder is the product documentation for the customer-facing Tyraxes surface.
Internal crate names remain `xai-grok-*` for compatibility; see
[27-compatibility.md](27-compatibility.md).

## Start here

| Doc | Topic |
|-----|-------|
| [00-overview.md](00-overview.md) | Product vision and capabilities |
| [01-quickstart.md](01-quickstart.md) | First engagement in minutes |
| [02-installation.md](02-installation.md) | Build and install |
| [05-red-team-agent.md](05-red-team-agent.md) | The `red-team` harness |
| [09-providers.md](09-providers.md) | OpenAI, OpenRouter, Ollama, LM Studio |
| [11-anti-loop.md](11-anti-loop.md) | Local-model loop protection |
| [16-engagement-workflow.md](16-engagement-workflow.md) | Kill-chain operating model |
| [24-rules-of-engagement.md](24-rules-of-engagement.md) | Authorization and scope |

## Full index

### Product & architecture
- [00 — Overview](00-overview.md)
- [01 — Quickstart](01-quickstart.md)
- [02 — Installation](02-installation.md)
- [03 — Architecture](03-architecture.md)
- [04 — Agent loop](04-agent-loop.md)

### Agents, skills, tools
- [05 — Red Team agent](05-red-team-agent.md)
- [06 — Specialist subagents](06-specialist-subagents.md)
- [07 — Skills](07-skills.md)
- [08 — Toolset](08-toolset.md)

### Models & reliability
- [09 — Providers](09-providers.md)
- [10 — Local models](10-local-models.md)
- [11 — Anti-loop](11-anti-loop.md)

### Configuration & modes
- [12 — Configuration](12-configuration.md)
- [13 — Environment variables](13-environment-variables.md)
- [14 — Headless mode](14-headless-mode.md)
- [15 — MCP integration](15-mcp-integration.md)

### Offensive methodology
- [16 — Engagement workflow](16-engagement-workflow.md)
- [17 — Recon methodology](17-recon-methodology.md)
- [18 — Web application offense](18-web-app-offense.md)
- [19 — Network offense](19-network-offense.md)
- [20 — Privilege escalation](20-privilege-escalation.md)
- [21 — Reporting](21-reporting.md)
- [22 — Attack graph](22-attack-graph.md)
- [23 — Evidence discipline](23-evidence-discipline.md)

### Safety, ops, extension
- [24 — Rules of engagement](24-rules-of-engagement.md)
- [25 — Security and ethics](25-security-and-ethics.md)
- [26 — Workspace layout](26-workspace-layout.md)
- [27 — Compatibility](27-compatibility.md)
- [28 — Troubleshooting](28-troubleshooting.md)
- [29 — CLI reference](29-cli-reference.md)
- [30 — TUI guide](30-tui-guide.md)
- [31 — Extending agents](31-extending-agents.md)
- [32 — Custom skills](32-custom-skills.md)
- [33 — Development](33-development.md)
- [34 — FAQ](34-faq.md)
- [35 — Operator playbook](35-operator-playbook.md)
- [36 — Detection gap analysis](36-detection-gap-analysis.md)
- [37 — Cloud and identity offense](37-cloud-identity-offense.md)
- [38 — Telemetry and artifacts](38-telemetry-and-artifacts.md)
