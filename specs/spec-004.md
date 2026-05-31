# Spec 004: Error Types & JSON Error Serialization

- [ ] Not implemented

## Goal

Define a comprehensive `JariError` enum that covers all failure modes, each mapping to a stable machine-readable error code and actionable human suggestion.

## Requirements

### Error Enum Variants

| Variant | Code | HTTP | Suggestion |
|---------|------|------|------------|
| `Config(String)` | `config_error` | — | Message from config validation |
| `Auth` | `auth_failed` | 401 | "Check your email and API token. Generate a token at https://id.atlassian.com/manage-profile/security/api-tokens" |
| `Permission` | `permission_denied` | 403 | "You don't have permission for this action. Contact your Jira admin." |
| `NotFound` | `not_found` | 404 | "The requested resource was not found. Verify the key/ID." |
| `Validation` | `validation_error` | 400/422 | "Check the required fields and values. Use 'jari field list' to see valid fields." |
| `RateLimit` | `rate_limited` | 429 | "Rate limited. Wait before retrying." |
| `ServerError` | `server_error` | 5xx | "Jira server error. This is usually temporary. Retry in a moment." |
| `Network(reqwest::Error)` | `network_error` | — | "Network error. Check your connection and the Jira URL." |
| `AdfConversion(String)` | `adf_error` | — | "Failed to convert rich text. The content may contain unsupported formatting." |
| `Cli(String)` | `cli_error` | — | "Check the command syntax. Use --help for usage." |

### Behavior

- `Validation` carries raw Jira error details as `HashMap<String, String>`
- `RateLimit` carries optional `retry_after: Option<Duration>`
- `Config` and `AdfConversion` carry contextual message strings
- All variants serialize to the JSON error envelope shape (spec 005)
- `From<reqwest::Error>` conversion for `Network` variant
- `From<serde_json::Error>` conversion for deserialization failures
- Exit codes: `0` success, `1` error, `2` for `Cli` usage error
