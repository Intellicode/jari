# Spec 012: First Integration Test — Issue Get

- [ ] Not implemented

## Goal

Write the first integration test using `wiremock` to verify the full pipeline: CLI dispatch → HTTP request → JSON output.

## Requirements

### Test Scenario

- Mock Jira Cloud server via `wiremock` at a fake URL
- Mock `GET /rest/api/3/issue/PROJ-123` returning realistic JSON fixture
- Invoke `jari issue get PROJ-123` via config pointing at mock server
- Assert stdout is valid JSON matching the output envelope shape (spec 005)
- Assert `ok: true` at top level
- Assert `data.key == "PROJ-123"`
- Assert `data.fields.summary` is present
- Assert `meta.command` contains the invocation
- Assert `meta.duration_ms` is a positive integer

### Fixture

- Create `tests/fixtures/issue_EX-1.json` with realistic Jira issue JSON
- Include ADF `description` field for later ADF conversion tests

### Additional Tests

- Test 404 error: mock returns 404, assert `ok: false`, `error.code == "not_found"`
- Test 401 error: mock returns 401, assert `error.code == "auth_failed"`
- Test with `--output json-pretty`: assert multi-line, indented output

### File

- `tests/integration/issues.rs`
