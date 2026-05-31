# Spec 003: Configuration Loading & Validation

- [ ] Not implemented

## Goal

Load configuration from TOML file, environment variables, and CLI flags with a clear precedence order, and validate all values.

## Requirements

### Config File

- **Path**: `~/.config/jari/config.toml` (resolved via `directories` crate)
- Override with `--config <PATH>` CLI flag
- TOML structure:

```toml
[connection]
url = "https://your-company.atlassian.net"
email = "you@company.com"
token = "your-api-token"

[defaults]
project = "PROJ"
max_results = 100

[output]
format = "json"         # json | json-pretty
timezone = "local"      # local | utc
```

### Environment Variable Overrides

| Env var | Overrides |
|---------|-----------|
| `JARI_URL` | `connection.url` |
| `JARI_EMAIL` | `connection.email` |
| `JARI_TOKEN` | `connection.token` |
| `JARI_PROJECT` | `defaults.project` |
| `JARI_OUTPUT` | `output.format` |

### Resolution Order

1. CLI flags (`--url`, `--email`, `--token`)
2. Environment variables
3. Config file
4. No defaults for credentials — must be explicitly configured

### Validation

- `url`: must be valid HTTPS URL matching `https://*.atlassian.net` or `https://*.jira.com`
- `email`: must contain `@`
- `token`: must be non-empty string
- Invalid config produces a clear JSON error with actionable fix suggestions
- Config struct uses `Default` for optional fields (`defaults`, `output`)
- Missing config file is NOT an error — only errors when no credentials are found by resolution order
