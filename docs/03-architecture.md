# Architecture

Tyraxes is a Rust monorepo. Customer-facing branding is Tyraxes; crate names
remain `xai-grok-*`.

```text
┌────────────────────────────────────────────────────────────┐
│  tyraxes binary (xai-grok-pager-bin)                       │
│  TUI / CLI / headless / ACP entrypoints                    │
└───────────────────────────┬────────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────────┐
│  xai-grok-pager                                            │
│  Scrollback, prompts, modals, rendering                    │
└───────────────────────────┬────────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────────┐
│  xai-grok-shell  (SessionActor / real agent loop)          │
│  Turns, tool execution, anti-loop, MCP, skills             │
└───────┬───────────────────────────────┬────────────────────┘
        │                               │
┌───────▼──────────┐          ┌─────────▼────────────────────┐
│ xai-grok-agent   │          │ xai-grok-tools               │
│ definitions      │          │ shell/files/search/web/...   │
│ prompts/toolsets │          │ loop-aware execution         │
└──────────────────┘          └──────────────────────────────┘
```

## Critical separation

- **`xai-grok-agent`** owns definitions, prompts, toolsets, discovery.
- **`xai-grok-shell`** owns the live session loop (`SessionActor`).
- Anti-loop guards live on session state and fire during tool/turn execution.

## Red-team harness properties

- `PromptMode::Full`
- curated `red_team_toolset()`
- `inject_default_tools = false` (strict harness)
- `PermissionMode::BypassPermissions` (research posture; layer your own policy later)
- skills preloaded

## Data on disk

- User config: `~/.tyraxes/`
- Engagement workspace: `./.tyraxes/redteam/`
- Sessions: under resolved home (compat with historical layout)
