# Spec 031: Client — Transitions (List & Execute)

- [ ] Not implemented

## Goal

Implement API methods to list available transitions for an issue and execute a transition by ID or name.

## Requirements

### List Transitions

```rust
async fn list_transitions(&self, key: &str) -> Result<Vec<Transition>>
```

- HTTP `GET /rest/api/3/issue/{key}/transitions`
- Returns `Vec<Transition>` with `id`, `name`, `to` (destination status)

### Execute Transition

```rust
async fn do_transition(
    &self,
    key: &str,
    transition_id_or_name: &str,
    comment: Option<&str>,
    resolution: Option<&str>,
) -> Result<TransitionResult>
```

- HTTP `POST /rest/api/3/issue/{key}/transitions`
- Body: `{"transition": {"id": "<resolved_id>"}}`
- Optional: `{"update": {"comment": [{"add": {"body": ADF}}]}}` if comment provided
- Optional: `{"fields": {"resolution": {"name": "<resolution>"}}}` if resolution provided

### Transition Name Fuzzy Matching

- If `transition_id_or_name` is a numeric ID string, use directly
- If it's a name, fetch transitions list and find a match:
  1. Exact case-insensitive match → use it
  2. Starts-with case-insensitive match → use first
  3. Contains case-insensitive match → use first
  4. No match → return error with list of available transition names as suggestion

### TransitionResult

```rust
pub struct TransitionResult {
    pub transition: String,        // Transition name
    pub from_status: String,
    pub to_status: String,
}
```

### Requirements

- Models in `src/models/transition.rs`
- Client methods in `src/client/transitions.rs`
- ADF-convert comment body if provided
- 404 → `NotFound`
- 400/422 → `Validation` with Jira error details (e.g., missing required fields)
