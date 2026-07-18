# Installation

## From source (recommended for this tree)

### Requirements

- Rust (see `rust-toolchain.toml`)
- DotSlash (`cargo install dotslash`)
- `protoc` via `bin/protoc` or `$PROTOC`

### Build

```sh
cargo build -p xai-grok-pager-bin --release
```

Artifacts:

| Binary | Role |
|--------|------|
| `target/release/tyraxes` | Preferred CLI name |
| `target/release/grok` | Compatibility alias |
| `target/release/xai-grok-pager` | Historical internal name |

### Install on PATH

```sh
install -m 755 target/release/tyraxes ~/.local/bin/tyraxes
# optional alias
ln -sf ~/.local/bin/tyraxes ~/.local/bin/grok
```

## Config home

| Priority | Location |
|----------|----------|
| 1 | `$TYRAXES_HOME` |
| 2 | `$GROK_HOME` |
| 3 | existing `~/.tyraxes` |
| 4 | existing `~/.grok` |
| 5 | create `~/.tyraxes` |

## System config (Unix)

`/etc/tyraxes` preferred, `/etc/grok` fallback.

## Verify

```sh
tyraxes --version
tyraxes models
echo $TYRAXES_HOME
ls ~/.tyraxes
```
