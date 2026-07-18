# Development

## Crate focus

Always build/test specific crates; full workspace builds are heavy.

```sh
cargo check -p xai-grok-shell
cargo check -p xai-grok-pager-bin
cargo test -p xai-grok-agent --lib
cargo test -p xai-grok-tools --lib
cargo test -p xai-grok-config
```

## Safe extension zones

- builtin agents / prompts / toolsets
- skills under `xai-grok-shell/skills`
- provider presets
- session-side anti-loop
- branding/paths/docs

Avoid rewriting pager ACP stream parsers unless necessary.

## Branding source of truth

`xai-grok-config/src/branding.rs`
