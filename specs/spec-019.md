# Spec 019: Search Models

- [ ] Not implemented

## Goal

Define the request and response model types for JQL search, including pagination support.

## Requirements

### SearchRequest

```rust
pub struct SearchRequest {
    pub jql: String,
    pub start_at: usize,       // Default 0
    pub max_results: usize,    // Default 100
    pub fields: Vec<String>,   // Default: summary,status,assignee,priority,issuetype
    pub fields_by_keys: bool,  // Default false
}
```

### SearchResults

```rust
pub struct SearchResults {
    pub total: usize,
    pub issues: Vec<IssueSummary>,
}
```

### JQL Value Types

- `SearchRequest` serializes to JSON for POST body
- Alternative: GET with query parameters for simple queries (limited length)
- Prefer POST for JQL to avoid URL length limits
- Validate JQL is non-empty

### Requirements

- Derive `Serialize` for `SearchRequest`, `Deserialize` for `SearchResults`
- `#[serde(rename = "camelCase")]` for all fields
- `Default` impl for `SearchRequest` with sensible defaults
- Defined in `src/models/search.rs`
