# Spec 042: Integration Tests — Write Operations

- [ ] Not implemented

## Goal

Integration tests for all write-side operations with `wiremock`.

## Requirements

### Test Scenarios

**Issue Create:**
- Create a story with all fields: assert response contains key, id, url
- Create a subtask with parent: verify parent in request body
- Create with `@file.md` description: verify file contents read and converted
- Create with minimal fields (only project + summary): assert defaults used

**Issue Edit:**
- Edit summary: assert PUT body contains only `fields.summary`
- Edit description: assert description converted to ADF in body
- Edit with multiple fields: assert all in body
- Edit nonexistent issue: assert `not_found`

**Issue Delete:**
- Delete with `--force`: assert DELETE request sent
- Delete without force + confirmation "yes": assert success
- Delete without force + confirmation "no": assert cancelled

**Issue Assign:**
- Assign to account ID: assert `accountId` in body
- Assign to email: assert `name` in body
- Assign `me`: verify current user fetched first
- Assign `unassigned`: assert `accountId: null`

**Issue Watch:**
- Add watcher: assert POST to watchers endpoint
- Remove watcher: assert DELETE with accountId query param

**Transitions:**
- List transitions: assert all returned
- Do transition by ID: assert correct body
- Do transition by name: assert fuzzy match resolves to correct ID
- Do transition with comment: assert comment ADF in update body
- Do transition with resolution: assert resolution in fields body
- Invalid transition name: assert error with available list

**Comment Add:**
- Add comment: assert body is ADF JSON
- Add comment with visibility group: assert visibility in body
- Add comment with visibility role: assert visibility in body

### Fixtures

- `tests/fixtures/transitions.json`
- `tests/fixtures/create_issue_response.json`

### Test Files

- `tests/integration/transitions.rs`
- Update existing `tests/integration/issues.rs` with write tests
- `tests/integration/comments.rs` add write tests
