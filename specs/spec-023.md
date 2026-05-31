# Spec 023: Client — Users (Current User)

- [ ] Not implemented

## Goal

Implement API method to fetch the currently authenticated user.

## Requirements

```rust
async fn get_current_user(&self) -> Result<User>
```

- HTTP `GET /rest/api/3/myself`
- Returns `User` with `account_id`, `email_address`, `display_name`, `active`
- 401 → `Auth` error

### User Model

```rust
pub struct User {
    pub account_id: Option<String>,
    pub email_address: Option<String>,
    pub display_name: String,
    pub active: bool,
}
```

### Additional User Lookup (future)

- `async fn find_user(&self, query: &str) -> Result<Vec<User>>` — search by email or display name
- Can be implemented in v1 if needed for `issue assign` when user passes email instead of account ID

### Requirements

- Model in `src/models/user.rs` (re-exported from common)
- Client method in `src/client/users.rs`
- `#[serde(default)]` for optional fields
