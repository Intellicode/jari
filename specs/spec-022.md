# Spec 022: Client — Projects (List & Get)

- [ ] Not implemented

## Goal

Implement API methods to list all accessible projects and get project details.

## Requirements

### List Projects

```rust
async fn list_projects(&self, type_filter: Option<&str>) -> Result<Vec<Project>>
```

- HTTP `GET /rest/api/3/project/search`
- Optional `type` query param: `software`, `service_desk`, `business`
- Auto-paginates results
- Returns lightweight project list (key, name, id, project_type_key)

### Get Project

```rust
async fn get_project(&self, key: &str) -> Result<Project>
```

- HTTP `GET /rest/api/3/project/{key}`
- Returns full project details including lead, versions, components, issue types
- 404 → `NotFound` error

### Project Model

```rust
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub lead: Option<User>,
    pub project_type_key: Option<String>,  // "software", "service_desk", "business"
    pub simplified: bool,
    pub style: Option<String>,             // "classic" or "next-gen"
    pub versions: Vec<Version>,
    pub components: Vec<Component>,
    pub issue_types: Option<Vec<IssueType>>,
}
```

### Requirements

- Model defined in `src/models/project.rs`
- Client methods in `src/client/projects.rs`
- `#[serde(default)]` for forward compatibility
