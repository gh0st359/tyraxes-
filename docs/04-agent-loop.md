# Agent loop

Each interactive or headless turn follows roughly:

1. Assemble system prompt + skills + context
2. Call the model (chat_completions / responses / messages backend)
3. Stream assistant content into the TUI / headless sink
4. Execute requested tool calls
5. Apply **tool-loop guard** fingerprints
6. Feed tool results back
7. Apply **response-loop guard** on narration similarity
8. Continue until stop condition / approval / hard stop

## Why this matters for offense

Offensive work is tool-heavy. Local models often:

- repeat the same `nmap` / `curl` / `ls` call
- narrate the same plan without progressing
- thrash between two equivalent commands

Tyraxes treats those as first-class failure modes (see
[11-anti-loop.md](11-anti-loop.md)).

## Subagent turns

Specialists run nested agent definitions with narrower prompts. The parent
keeps the attack graph and findings directory as the shared truth.

## Approval modes

Depending on CLI flags and permission mode, shell/file tools may require
operator approval. Headless automation typically uses `--always-approve` in
trusted labs only.
