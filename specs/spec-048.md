# Spec 048: CLI — Config Init (Interactive Setup Wizard)

- [ ] Not implemented

## Goal

Implement an interactive setup wizard that prompts for credentials, validates them, and writes the config file.

## Requirements

### CLI Interface

```
jari config init
```

### Flow

1. **Prompt for URL**: "Jira Cloud URL (e.g., https://company.atlassian.net):"
   - Validate format matches `https://*.atlassian.net` or `https://*.jira.com`
   - Re-prompt if invalid
   - Accept empty to skip

2. **Prompt for email**: "Email:"
   - Validate contains `@`
   - Re-prompt if invalid

3. **Prompt for API token**: "API token (input hidden):"
   - Use hidden input (no echo) — use `rpassword` crate or equivalent
   - Link to token generation: "Generate at https://id.atlassian.com/manage-profile/security/api-tokens"
   - Validate non-empty

4. **Validate connection**: Call `/rest/api/3/myself` with provided credentials
   - On success: "Connected as: {display_name} ({email})"
   - On failure: show error, offer to re-enter credentials or save anyway

5. **Write config**: Save to `~/.config/jari/config.toml`
   - Create parent directories if needed
   - If file exists, prompt for overwrite confirmation
   - Set file permissions to 600 (owner read/write only) on Unix

6. **Success message**: "Config saved to ~/.config/jari/config.toml"

### Output Shape

```json
{
  "ok": true,
  "data": {
    "path": "/Users/name/.config/jari/config.toml",
    "user": {
      "display_name": "Jane Smith",
      "email": "jane@company.com"
    }
  },
  "meta": { "command": "jari config init", "duration_ms": 5000 }
}
```

### Requirements

- Interactive mode detects if stdin is a TTY; if not, error with "config init requires an interactive terminal"
- Token never echoed to terminal
- Config file permissions set to `0o600`
- Handles keyboard interrupt (Ctrl+C) gracefully
