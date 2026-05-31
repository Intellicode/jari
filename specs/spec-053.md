# Spec 053: Unit Tests — Error Serialization

- [ ] Not implemented

## Goal

Unit tests verifying that all `JariError` variants serialize to the correct JSON error envelope.

## Requirements

### Test Scenarios

For each error variant, test:
1. Serialization produces valid JSON
2. `error.code` matches the expected stable string
3. `error.message` is present and non-empty
4. `error.suggestion` is present and non-empty
5. `error.http_status` is correct (when applicable)
6. `error.jira_errors` is present (for `Validation` variant)

### Variant-Specific Tests

- `Config("token is empty")`: code=`config_error`, no http_status
- `Auth`: code=`auth_failed`, http_status=401
- `Permission`: code=`permission_denied`, http_status=403
- `NotFound`: code=`not_found`, http_status=404
- `Validation { jira_errors }`: code=`validation_error`, http_status=422, jira_errors present
- `RateLimit { retry_after }`: code=`rate_limited`, http_status=429, retry_after serialized
- `ServerError`: code=`server_error`, http_status=500
- `Network(error)`: code=`network_error`, no http_status
- `AdfConversion("bad node")`: code=`adf_error`, no http_status
- `Cli("usage error")`: code=`cli_error`, no http_status

### Output Shape Verification

- Use `assert-json-diff` for snapshot-style comparison
- Snapshot files stored for each variant
- Verify full JSON structure matches the output envelope spec

### Test Files

- `src/error.rs` — `#[cfg(test)] mod tests`
- Snapshot files: `tests/snapshots/error_*.json`
