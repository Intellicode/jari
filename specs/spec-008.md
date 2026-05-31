# Spec 008: Issue Model

- [ ] Not implemented

## Goal

Define the full `Issue` and `IssueSummary` model types matching the Jira Cloud REST API v3 response shape.

## Requirements

### Issue (full — for `issue get`)

```rust
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

pub struct IssueFields {
    pub summary: String,
    pub description_markdown: Option<String>,   // ADF converted on read
    pub issuetype: IssueType,
    pub status: Status,
    pub priority: Option<Priority>,
    pub assignee: Option<User>,
    pub reporter: Option<User>,
    pub created: String,       // ISO 8601
    pub updated: String,
    pub duedate: Option<String>,
    pub resolution: Option<Resolution>,
    pub labels: Vec<String>,
    pub components: Vec<Component>,
    pub fix_versions: Vec<Version>,
    pub versions: Vec<Version>,
    pub parent: Option<IssueLink>,
    pub subtasks: Vec<IssueLink>,
    pub issuelinks: Vec<IssueLinkType>,
    pub timetracking: Option<TimeTracking>,
    pub votes: Option<Votes>,
    pub watches: Option<Watches>,
    pub worklog: Option<Worklog>,
    pub comment: Option<CommentsPage>,
    pub project: ProjectSummary,
}
```

### IssueSummary (for search results)

```rust
pub struct IssueSummary {
    pub id: String,
    pub key: String,
    pub fields: IssueSummaryFields,
}

pub struct IssueSummaryFields {
    pub summary: String,
    pub issuetype: IssueType,
    pub status: Status,
    pub priority: Option<Priority>,
    pub assignee: Option<User>,
    pub created: String,
    pub updated: String,
    pub duedate: Option<String>,
    pub labels: Vec<String>,
}
```

### Requirements

- All types derive `Deserialize`, `Serialize`, `JsonSchema`
- `description_markdown` is NOT a direct API field — it is populated post-fetch by ADF conversion
- The raw ADF `description` field from the API is stored temporarily for conversion
- All optional fields use `#[serde(default)]` for forward compatibility with API changes
- Types defined in `src/models/issue.rs`
