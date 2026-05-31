# Spec 055: Integration Tests — Error Paths

- [ ] Not implemented

## Goal

Integration tests covering all error response scenarios with proper JSON error output.

## Requirements

### Error Scenarios

| Scenario | Status | Expected `error.code` |
|----------|--------|----------------------|
| Invalid credentials | 401 | `auth_failed` |
| Permission denied | 403 | `permission_denied` |
| Issue not found | 404 | `not_found` |
| Invalid issue key format | 400 | `validation_error` |
| Missing required fields | 400 | `validation_error` |
| Invalid field values | 422 | `validation_error` |
| Rate limited | 429 | `rate_limited` |
| Server error | 500 | `server_error` |
| Server error | 502 | `server_error` |
| Server error | 503 | `server_error` |
| Connection refused | — | `network_error` |
| DNS failure | — | `network_error` |
| Config missing URL | — | `config_error` |
| Config invalid token | — | `config_error` |
| Malformed JQL | 400 | `validation_error` |
| Transition not allowed | 400 | `validation_error` |

### Test Requirements

- Mock each HTTP status code with realistic Jira error response body
- Assert `ok: false` in output
- Assert correct `error.code` string
- Assert `error.http_status` matches (when applicable)
- Assert `error.suggestion` is helpful and non-empty
- Assert `error.jira_errors` contains raw Jira details (for 400/422)
- For 429: assert `error.retry_after` is parsed from `Retry-After` header
- For network errors: mock server disabled/unreachable, assert `network_error`

### Test Files

- `tests/integration/error_paths.rs` or distributed across existing test files
