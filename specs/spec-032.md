# Spec 032: Client — Issue Write Operations (Create, Edit, Delete, Assign, Watch)

- [ ] Not implemented

## Goal

Implement all issue mutation API methods: create, edit, delete, assign, and watch.

## Requirements

### Create Issue

```rust
async fn create_issue(&self, request: &CreateIssueRequest) -> Result<CreatedIssue>
```

- HTTP `POST /rest/api/3/issue`
- Body fields: `{fields: {project, summary, issuetype, description, priority, assignee, labels, parent, customfield_...}}`
- Description must be ADF JSON (converted from markdown upstream)
- Returns `CreatedIssue { id, key, self }` where `self` is the issue URL

### CreateIssueRequest

```rust
pub struct CreateIssueRequest {
    pub project: String,              // Project key
    pub summary: String,
    pub issuetype: String,            // "Story", "Bug", "Task", etc.
    pub description_adf: Option<Value>,  // ADF JSON
    pub priority: Option<String>,     // Priority name
    pub assignee: Option<String>,     // Account ID
    pub labels: Option<Vec<String>>,
    pub parent: Option<String>,       // Parent issue key (for subtasks)
    pub epic_link: Option<String>,    // Epic issue key
}
```

### Edit Issue

```rust
async fn edit_issue(&self, key: &str, fields: &HashMap<String, Value>) -> Result<()>
```

- HTTP `PUT /rest/api/3/issue/{key}`
- Body: `{"fields": { ... }}` — only changed fields
- Returns nothing on success (204 or empty 200)

### Delete Issue

```rust
async fn delete_issue(&self, key: &str) -> Result<()>
```

- HTTP `DELETE /rest/api/3/issue/{key}`
- Returns nothing on success (204)

### Assign Issue

```rust
async fn assign_issue(&self, key: &str, user: &str) -> Result<()>
```

- HTTP `PUT /rest/api/3/issue/{key}/assignee`
- Body: `{"accountId": "<id>"}` if user looks like an account ID (numeric/uuid)
- Body: `{"name": "<name>"}` if user looks like an email or name (fallback)
- Auto-detect: if `user` contains `@`, use `name` field; else use `accountId`

### Watch Add / Remove

```rust
async fn add_watcher(&self, key: &str, account_id: &str) -> Result<()>
async fn remove_watcher(&self, key: &str, account_id: &str) -> Result<()>
```

- HTTP `POST /rest/api/3/issue/{key}/watchers` — body: `accountId`
- HTTP `DELETE /rest/api/3/issue/{key}/watchers?accountId=...`

### Requirements

- Client methods in `src/client/issues.rs`
- `CreateIssueRequest` and `CreatedIssue` in `src/models/issue.rs`
- Proper error mapping for each status code
