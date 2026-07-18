<div align="center">

# Tyraxes (`tyraxes`)

**Tyraxes** is an agentic offensive-security terminal agent. It runs as a
full-screen TUI that plans and executes authorized red-team, pentest, and
adversary-emulation work — interactively, headlessly for scripting/CI, or
embedded in editors via the Agent Client Protocol (ACP).

Activate the red-team harness with `TYRAXES_AGENT=red-team` (or
`GROK_AGENT=red-team` for compatibility). Provider presets cover OpenAI,
OpenRouter, Ollama, LM Studio, and xAI BYOK. Specialist subagents, methodology
skills, and client-side anti-loop protocols keep local models productive.

[Quickstart](docs/01-quickstart.md) ·
[Documentation](docs/README.md) ·
[Building from source](#building-from-source) ·
[Repository layout](#repository-layout) ·
[Compatibility](docs/27-compatibility.md)

</div>

---

## What it is

| Surface | Purpose |
|---------|---------|
| `tyraxes` CLI/TUI | Primary customer-facing binary (`grok` remains a compat alias) |
| `red-team` agent | Full offensive operator prompt + curated toolset |
| Specialists | `recon`, `vuln-triage`, `exploit-dev`, `reporting` |
| Skills | Recon, adaptive offense, web/network/privesc, reporting, tool arsenal |
| Config home | `~/.tyraxes` preferred; `~/.grok` still works |
| Workspace | `.tyraxes/redteam/` for scope, attack graph, findings, artifacts |

Internal Rust crates keep the historical `xai-grok-*` names. That is intentional
so the surface rebrand stays safe and reviewable.

## Quick start

```sh
# Build
cargo build -p xai-grok-pager-bin --release

# Prefer the Tyraxes binary name
./target/release/tyraxes --version

# Activate red-team + a local model
export TYRAXES_AGENT=red-team
# Copy model blocks from ~/.tyraxes/providers.presets.toml into config.toml
tyraxes -m ollama-local
```

Headless:

```sh
TYRAXES_AGENT=red-team tyraxes -p "Scope and recon 10.0.0.0/24 — authorized lab" \
  -m ollama-local --always-approve
```

## Building from source

Requirements:

- **Rust** — toolchain pinned by [`rust-toolchain.toml`](rust-toolchain.toml)
- **[DotSlash](https://dotslash-cli.com)** — hermetic tools under [`bin/`](bin/)
- **protoc** — via [`bin/protoc`](bin/protoc) or `$PROTOC`

```sh
cargo install dotslash   # once
cargo run -p xai-grok-pager-bin              # build + launch (tyraxes default-run)
cargo build -p xai-grok-pager-bin --release  # target/release/tyraxes (+ grok alias)
cargo check -p xai-grok-pager-bin
```

## Documentation

Product docs live in [`docs/`](docs/README.md) — architecture, agents, skills,
providers, anti-loop, engagement methodology, ROE, MCP, headless mode, and more.

In-crate user guide pages remain under
[`crates/codegen/xai-grok-pager/docs/user-guide/`](crates/codegen/xai-grok-pager/docs/user-guide/)
for keyboard shortcuts, theming, and legacy Grok Build topics.

## Repository layout

| Path | Contents |
|------|----------|
| `docs/` | Tyraxes product documentation |
| `crates/codegen/xai-grok-pager-bin` | Composition-root; builds `tyraxes` / `grok` / `xai-grok-pager` |
| `crates/codegen/xai-grok-pager` | TUI: scrollback, prompt, modals, rendering |
| `crates/codegen/xai-grok-shell` | Agent runtime + leader/stdio/headless entry points |
| `crates/codegen/xai-grok-agent` | Agent definitions, prompts, toolsets (`red-team`, …) |
| `crates/codegen/xai-grok-tools` | Tool implementations |
| `crates/codegen/xai-grok-config` | Paths, branding, config merge |
| `crates/codegen/...` | MCP, markdown, sandbox, workspace, … |
| `third_party/` | Vendored upstream source |

> [!IMPORTANT]
> The root `Cargo.toml` (workspace members, dependency versions, lints,
> profiles) is **generated** — treat it as read-only. Prefer editing per-crate
> `Cargo.toml` files.

## Development

```sh
cargo check -p <crate>        # always target specific crates; full-workspace builds are slow
cargo test -p xai-grok-config
cargo test -p xai-grok-agent --lib
cargo clippy -p <crate>
cargo fmt --all
```

## Compatibility

- Binary: `tyraxes` preferred; `grok` and `xai-grok-pager` still build
- Home: `$TYRAXES_HOME` / `~/.tyraxes` preferred; `$GROK_HOME` / `~/.grok` work
- Agent env: `TYRAXES_AGENT` preferred; `GROK_AGENT` works
- Project dirs: `.tyraxes/` preferred; `.grok/` still discovered

## License

First-party code in this repository is licensed under the **Apache License,
Version 2.0** — see [`LICENSE`](LICENSE).

Third-party and vendored code remains under its original licenses. See
[`THIRD-PARTY-NOTICES`](THIRD-PARTY-NOTICES) and
[`third_party/NOTICE`](third_party/NOTICE).
