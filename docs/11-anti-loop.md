# Anti-loop

Client-side guards protect the session from local-model spirals.

## Tool-loop guard

Tracks fingerprints of identical tool name + arguments.

| Stage | Behavior |
|-------|----------|
| First repeats | Nudge the model to change approach |
| Persistent repeats | Hard-stop further identical calls |

Implemented in session tool execution (`tool_loop_guard`).

## Response-loop guard

Tracks near-duplicate assistant narration across turns.

| Stage | Behavior |
|-------|----------|
| Near-duplicate | Nudge + continue agentic loop with correction |
| Persistent | Hard-stop |

Implemented in turn handling (`response_loop_guard`).

## What you should still do

Guards are a safety net, not a strategy:

- Lower temperature
- Narrow the task
- Demand evidence files
- Switch specialists
- Restart the session if context is poisoned

## Debugging loops

If you see repeated nudges:

1. Check whether the tool truly failed (auth, PATH, scope)
2. Inspect `.tyraxes/redteam/notes/` for missing state
3. Ask: “Summarize blocked hypotheses and next distinct test.”
