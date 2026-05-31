# Spec 037: CLI — Issue Assign

- [ ] Not implemented

## Goal

Wire the `issue assign` subcommand to change issue assignee.

## Requirements

### CLI Interface

```
jari issue assign <KEY> <USER>
```

- `<KEY>`: positional, required (issue key)
- `<USER>`: positional, required (account ID or email)

### Special User Values

- `me` or `currentUser()`: resolve to current user's account ID by calling `get_current_user()` first
- `unassigned` or `none`: unassign the issue (send `{"accountId": null}`)
- Email address (contains `@`): use `name` field in API
- Otherwise: use `accountId` field

### Behavior

1. Resolve user value (fetch current user if `me`)
2. Call `client.assign_issue(key, resolved_user)`
3. Output confirmation

### Output Shape

```json
{
  "ok": true,
  "data": {
    "key": "PROJ-123",
    "assignee": {
      "account_id": "5b10a2844c20165700ede21g",
      "display_name": "Bot User"
    }
  },
  "meta": { "command": "jari issue assign PROJ-123 me", "duration_ms": 200 }
}
```

### Error Handling

- Issue not found: `NotFound`
- User not found: `Validation` error from Jira
- `me` resolution fails: `Config` error "Could not determine current user"
