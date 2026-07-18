# Cloud and identity offense

Tyraxes treats cloud/IdP abuse as first-class when in scope.

## Common threads

- token/session reuse
- CI/CD identity pivots
- over-privileged roles
- federation trust abuse
- storage exposure → credential harvest

## Workspace tips

```text
.tyraxes/redteam/artifacts/cloud/
.tyraxes/redteam/notes/identity-map.md
```

Map identities as attack-graph nodes. Prefer read-only enumeration before any
mutating API calls; mutate only when charter allows.
