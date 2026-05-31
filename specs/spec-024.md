# Spec 024: Client — Fields (List Field Metadata)

- [ ] Not implemented

## Goal

Implement API method to list all available fields (system + custom) with their IDs and metadata.

## Requirements

```rust
async fn list_fields(&self) -> Result<Vec<Field>>
```

- HTTP `GET /rest/api/3/field`
- Auto-paginates (typically one page but handle just in case)
- Returns all fields: system fields + custom fields

### Field Model

```rust
pub struct Field {
    pub id: String,              // e.g., "summary", "customfield_10010"
    pub key: Option<String>,     // e.g., "summary"
    pub name: String,            // e.g., "Summary", "Story Points"
    pub custom: bool,
    pub orderable: bool,
    pub navigable: bool,
    pub searchable: bool,
    pub schema: Option<FieldSchema>,
}

pub struct FieldSchema {
    #[serde(rename = "type")]
    pub schema_type: String,     // "string", "number", "user", "array", "option", "doc"
    pub items: Option<String>,   // For array types
    pub system: Option<String>,  // e.g., "summary", "description"
    pub custom: Option<String>,  // e.g., "com.atlassian.jira...field:storypoints"
}
```

### Requirements

- Model in `src/models/field.rs` (lightweight, can be in `fields.rs`)
- Client method in `src/client/fields.rs`
- `#[serde(default)]` for all optional fields
- Output sorted alphabetically by name for readability
