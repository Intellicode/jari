# Spec 020: Client — Search with Auto-Pagination

- [ ] Not implemented

## Goal

Implement JQL search that automatically fetches all pages of results behind a single API call, so the LLM never sees pagination mechanics.

## Requirements

### API

```rust
async fn search(
    &self,
    jql: &str,
    fields: Option<&[String]>,
    max_override: Option<usize>,
) -> Result<Vec<IssueSummary>>
```

### Auto-Pagination Logic

1. Send first request with `start_at: 0`, `max_results: 100` (or configured page size)
2. Append results to accumulator
3. If `start_at + results.len() < total` AND not at `--max` limit, continue
4. Increment `start_at` by page size, send next request, repeat
5. Stop when all pages fetched or `--max` cap reached
6. Return all accumulated results in a single `Vec<IssueSummary>`

### Constraints

- **Soft cap**: Maximum 1000 results (Jira API deep pagination limit)
- **Sequential fetches**: Pages fetched one at a time to avoid rate limiting
- **Configurable page size**: Default 100, overridable via config `defaults.max_results`
- **JQL validation**: Non-empty check before sending

### Error Handling

- 400: parse `ErrorCollection`, return `Validation` error with JQL error details
- 401: `Auth` error
- 429 mid-pagination: respect `Retry-After`, retry with backoff (up to 3 retries)
- 5xx mid-pagination: retry with exponential backoff

### File

- `src/client/search.rs`
