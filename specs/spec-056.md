# Spec 056: Integration Tests — Auto-Pagination

- [ ] Not implemented

## Goal

Integration tests verifying that automatic pagination correctly aggregates multiple pages of results.

## Requirements

### Test Scenarios

**Search pagination:**
- Mock Jira returning 250 results across 3 pages (100 + 100 + 50)
- Assert search returns exactly 250 issues
- Assert correct `startAt` values sent in each request (0, 100, 200)
- Assert stops at `is_last: true`

**Search with max override:**
- Mock 500 total results (5 pages of 100)
- `--max 150` request
- Assert returns exactly 150 results
- Assert only 2 pages fetched (100 + 50)

**Comments pagination:**
- Mock 75 comments across 2 pages (50 + 25)
- Assert all 75 comments returned
- Assert comments in newest-first order

**Project list pagination:**
- Mock 5 projects in one page (is_last: true)
- Assert all 5 returned

**Edge cases:**
- Single page exactly equal to page size: assert `is_last` handled correctly
- Zero results: assert empty array, not endless loop
- Total changes between pages: use initial `total` for stop condition (graceful handling)

### Mock Setup

- `wiremock` mount with scenario-based responses (sequence of pages)
- Each response includes correct `startAt`, `maxResults`, `total`, `isLast`
- Verify request query parameters: `startAt` increments correctly

### Test Files

- `tests/integration/search.rs` — pagination sub-module
- `tests/integration/comments.rs` — pagination tests
