# Spec 029: CLI — Me (Current User)

- [ ] Not implemented

## Goal

Wire the `me` subcommand to display the currently authenticated user.

## Requirements

### CLI Interface

```
jari me
```

- No arguments
- Calls `client.get_current_user()`
- Output: single `User` object

### Output Shape

```json
{
  "ok": true,
  "data": {
    "account_id": "5b10a2844c20165700ede21g",
    "email_address": "bot@company.com",
    "display_name": "Bot User",
    "active": true
  },
  "meta": { "command": "jari me", "duration_ms": 120 }
}
```

### Use Case

- Verify authentication is working
- LLM discovers its own account ID for `issue assign` operations
- Configuration validation (`config init` calls this internally)

### Error Handling

- 401: `Auth` error with fix suggestion
