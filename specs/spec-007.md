# Spec 007: Common Model Types

- [ ] Not implemented

## Goal

Define the shared model types used across multiple API resources: pagination, error responses, users, statuses, priorities, issue types.

## Requirements

### Types to Define

```rust
pub struct User {
    pub account_id: Option<String>,
    pub email_address: Option<String>,
    pub display_name: String,
    pub active: bool,
}

pub struct Status {
    pub id: String,
    pub name: String,
    pub status_category: Option<StatusCategory>,
}

pub struct StatusCategory {
    pub id: u32,
    pub key: String,           // "indeterminate", "new", "done"
    pub name: String,
}

pub struct Priority {
    pub id: String,
    pub name: String,
    pub icon_url: Option<String>,
}

pub struct IssueType {
    pub id: String,
    pub name: String,
    pub subtask: bool,
    pub description: Option<String>,
}

pub struct Pagination {
    pub start_at: usize,
    pub max_results: usize,
    pub total: usize,
    pub is_last: bool,
}

pub struct ErrorCollection {
    #[serde(default)]
    pub error_messages: Vec<String>,
    #[serde(default)]
    pub errors: HashMap<String, String>,
}
```

### Additional Common Types

- `Resolution { id, name, description }`
- `Component { id, name, description }`
- `Version { id, name, description, released, archived }`
- `IssueLink { id, key }` — for parent/subtask references
- `IssueLinkType { id, name, inward_issue: IssueLink, outward_issue: IssueLink }`
- `ProjectSummary { id, key, name }` — lightweight project reference
- `Votes { has_voted: bool, votes: i64 }`
- `Watches { is_watching: bool, watch_count: i64 }`
- `TimeTracking { original_estimate, remaining_estimate, time_spent }`
- `Worklog { total: i64 }`
- `CommentsPage { total: i64 }` — lightweight pagination for embedded comments

### Requirements

- All types derive `Deserialize`, `Serialize`, `JsonSchema`
- All optional fields use `#[serde(default)]`
- All fields use `#[serde(rename = "camelCase")]` at module level where possible
- `HashMap` used for `ErrorCollection.errors`
- Types defined in `src/models/common.rs`
