# Spec 033: Client — Add Comment (Write)

- [ ] Not implemented

## Goal

Implement the comment creation API method with markdown-to-ADF conversion.

## Requirements

```rust
async fn add_comment(
    &self,
    key: &str,
    body_md: &str,
    visibility: Option<&str>,
) -> Result<CreatedComment>
```

- HTTP `POST /rest/api/3/issue/{key}/comment`
- Body: `{"body": <ADF JSON>}`
- Convert `body_md` to ADF via `markdown_to_adf()` before sending
- Optional visibility: `{"type": "group", "value": "<name>"}` or `{"type": "role", "value": "<id>"}`
- Parse `--visibility` format: `group:<name>` or `role:<id>`

### CreatedComment

```rust
pub struct CreatedComment {
    pub id: String,
    pub created: String,
}
```

### Requirements

- Model in `src/models/comment.rs`
- Client method in `src/client/comments.rs`
- ADF conversion in the client method (or in CLI handler)
- 404 → `NotFound`
- 400 → `Validation`
