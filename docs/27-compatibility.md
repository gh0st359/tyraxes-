# Compatibility

Tyraxes is a **surface rebrand**, not a crate rename.

## Kept stable

- Rust package names (`xai-grok-*`)
- Most internal APIs and env markers used by tests
- Ability to run as `grok`

## Preferred new names

| Old | New |
|-----|-----|
| Grok Build / Grok Red Team | Tyraxes / Tyraxes Red Team |
| `grok` binary | `tyraxes` (alias retained) |
| `~/.grok` | `~/.tyraxes` |
| `$GROK_HOME` | `$TYRAXES_HOME` |
| `$GROK_AGENT` | `$TYRAXES_AGENT` |
| `.grok/redteam` | `.tyraxes/redteam` |
| `/etc/grok` | `/etc/tyraxes` |

## Migration

No forced migration. If `~/.grok` exists and `~/.tyraxes` does not, Tyraxes
continues using the legacy home. New installs prefer `~/.tyraxes`.

To move:

```sh
mv ~/.grok ~/.tyraxes
export TYRAXES_HOME="$HOME/.tyraxes"
```
