# Spec 036: CLI — Issue Delete

- [ ] Not implemented

## Goal

Wire the `issue delete` subcommand with confirmation requirement.

## Requirements

### CLI Interface

```
jari issue delete <KEY> [--force]
```

### Behavior

- Without `--force`: output a confirmation prompt via stderr, exit with error if not confirmed
- With `--force`: skip confirmation
- Call `client.delete_issue(key)`
- Output: success confirmation

### Confirmation (stderr prompt)

```
Are you sure you want to delete PROJ-123? This cannot be undone. (y/N)
```

- Read one line from stdin
- Accept `y`, `Y`, `yes`, `Yes` (case-insensitive)
- Anything else: abort with `Cli` error "Deletion cancelled"

### Output Shape

```json
{
  "ok": true,
  "data": {
    "deleted": "PROJ-123"
  },
  "meta": { "command": "jari issue delete PROJ-123 --force", "duration_ms": 250 }
}
```

### Error Handling

- Issue not found: `NotFound`
- Permission denied: `Permission` (403)
