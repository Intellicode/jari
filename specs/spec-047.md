# Spec 047: CLI — Config Show & Path

- [ ] Not implemented

## Goal

Wire the `config show` and `config path` subcommands for inspecting configuration.

## Requirements

### config show

```
jari config show
```

- Load config with full resolution (file + env + flags)
- Output all config values
- **Mask the token**: show first 4 chars + `...` + last 4 chars (or `***` if too short)
- Show source of each value (file, env, CLI, default)

### Output Shape

```json
{
  "ok": true,
  "data": {
    "connection": {
      "url": "https://company.atlassian.net",
      "source_url": "config_file",
      "email": "bot@company.com",
      "source_email": "env_var",
      "token": "ATAT...FfGx",
      "source_token": "cli_flag"
    },
    "defaults": {
      "project": "PROJ",
      "max_results": 100
    },
    "output": {
      "format": "json",
      "timezone": "local"
    }
  },
  "meta": { "command": "jari config show", "duration_ms": 5 }
}
```

### config path

```
jari config path
```

- Output the resolved config file path (even if it doesn't exist)
- Does NOT require valid config — just prints the path

### Output Shape

```json
{
  "ok": true,
  "data": {
    "path": "/Users/name/.config/jari/config.toml"
  },
  "meta": { "command": "jari config path", "duration_ms": 2 }
}
```

### Requirements

- Both subcommands work even without valid credentials (config path always works, config show works with partial config)
- Token masking is deterministic and safe
- Source tracking: each config value knows if it came from file, env, or CLI
