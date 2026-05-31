# Spec 054: Integration Tests — API Operations (Happy Paths)

- [ ] Not implemented

## Goal

Integration tests using `wiremock` covering all API operations with successful responses.

## Requirements

### Covered Endpoints

Each endpoint tested with a mock server:

1. `GET /issue/{key}` — returns full issue with ADF description
2. `POST /issue` — creates issue, returns key/id/url
3. `PUT /issue/{key}` — edits issue, returns 204
4. `DELETE /issue/{key}` — deletes issue, returns 204
5. `PUT /issue/{key}/assignee` — assigns user, returns 204
6. `POST /issue/{key}/watchers` — adds watcher, returns 204
7. `DELETE /issue/{key}/watchers` — removes watcher, returns 204
8. `POST /search` — returns paginated search results
9. `GET /issue/{key}/transitions` — returns available transitions
10. `POST /issue/{key}/transitions` — executes transition, returns 204/200
11. `GET /issue/{key}/comment` — returns paginated comments
12. `GET /issue/{key}/comment/{id}` — returns single comment
13. `POST /issue/{key}/comment` — adds comment, returns id/created
14. `GET /project/search` — returns project list
15. `GET /project/{key}` — returns project details
16. `GET /field` — returns field metadata
17. `GET /myself` — returns current user

### Test Requirements

- Each test starts a `wiremock` server on a random port
- Config points at mock server URL
- Assert JSON output matches expected shape
- Assert `ok: true` and correct data types
- Assert HTTP request body matches expected shape (for POST/PUT)

### Fixtures

Realistic JSON fixtures for each endpoint stored in `tests/fixtures/`
