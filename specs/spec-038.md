# Spec 038: CLI — Issue Watch (Add / Remove)

- [ ] Not implemented

## Goal

Wire the `issue watch add` and `issue watch remove` subcommands.

## Requirements

### CLI Interface

```
jari issue watch add <KEY>
jari issue watch remove <KEY>
```

- Each takes a single positional `<KEY>` argument
- Watcher is always the current authenticated user

### Behavior

- `watch add`: call `client.add_watcher(key, current_user.account_id)`
- `watch remove`: call `client.remove_watcher(key, current_user.account_id)`
- Fetch current user's `account_id` via `get_current_user()` first
- Output confirmation

### Output Shape

```json
{
  "ok": true,
  "data": {
    "key": "PROJ-123",
    "watching": true,
    "account_id": "5b10a2844c20165700ede21g"
  },
  "meta": { "command": "jari issue watch add PROJ-123", "duration_ms": 180 }
}
```

### Error Handling

- Issue not found: `NotFound`
- Already watching (add): Jira may return 400 — handle gracefully, treat as success
- Not watching (remove): handle gracefully, treat as success
