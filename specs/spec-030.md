# Spec 030: Integration Tests — Read Operations

- [ ] Not implemented

## Goal

Integration tests for all read-side operations using `wiremock`.

## Requirements

### Test Scenarios

**Search:**
- Successful search with results: mock POST `/rest/api/3/search`, assert paginated results merged
- Empty search results: assert empty array, `ok: true`
- Search with `--max`: assert result count <= max
- Invalid JQL: mock 400 with `ErrorCollection`, assert `error.code == "validation_error"`

**Comments:**
- List comments with 3 comments: assert all in array, markdown bodies present
- List comments on issue with 0 comments: assert empty array
- Get single comment: assert correct ID and body

**Projects:**
- List projects with type filter: verify URL includes `type=` param
- Get project: verify full fields (versions, components, lead)
- Get nonexistent project: assert `not_found`

**Fields:**
- List fields: assert array has system and custom fields
- Verify `custom` boolean correctly set
- Verify `schema` present for relevant fields

**Me:**
- Get current user: assert `account_id` and `display_name` present
- Auth failure: mock 401, assert `error.code == "auth_failed"`

### Fixtures

Create test fixtures:
- `tests/fixtures/search_results.json`
- `tests/fixtures/comments.json`
- `tests/fixtures/project_list.json`
- `tests/fixtures/project_PROJ.json`
- `tests/fixtures/fields.json`
- `tests/fixtures/myself.json`

### Test Files

- `tests/integration/search.rs`
- `tests/integration/comments.rs`
- `tests/integration/projects.rs`
- `tests/integration/fields.rs`
- `tests/integration/users.rs`
