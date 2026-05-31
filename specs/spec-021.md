# Spec 021: Client — Comments (List & Get)

- [ ] Not implemented

## Goal

Implement API methods to list comments on an issue (with auto-pagination) and get a specific comment by ID.

## Requirements

### List Comments

```rust
async fn list_comments(&self, key: &str, max: Option<usize>) -> Result<Vec<Comment>>
```

- HTTP `GET /rest/api/3/issue/{key}/comment`
- Standard pagination: `startAt`, `maxResults` query params
- Auto-paginates if `max` is `None` or larger than page size
- Results ordered newest first (API default)
- Converts each comment's ADF `body` to `body_markdown` using ADF converter

### Get Comment

```rust
async fn get_comment(&self, key: &str, comment_id: &str) -> Result<Comment>
```

- HTTP `GET /rest/api/3/issue/{key}/comment/{id}`
- Convert ADF `body` to `body_markdown`
- 404 → `NotFound` error

### Comment Model

```rust
pub struct Comment {
    pub id: String,
    pub author: User,
    pub body_markdown: Option<String>,   // ADF converted
    pub created: String,
    pub updated: String,
}
```

### Requirements

- `Comment` model defined in `src/models/comment.rs`
- Client methods in `src/client/comments.rs`
- ADF conversion integrated on read (body → body_markdown)
- `#[serde(default)]` for all optional fields
