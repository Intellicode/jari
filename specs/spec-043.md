# Spec 043: Schema Generation — Walk clap Tree & Build Tool Definitions

- [ ] Not implemented

## Goal

Programmatically walk the `clap` command tree to generate accurate tool-use schema definitions that never drift from the actual CLI.

## Requirements

### Approach

- Use `clap::Command` reflection to iterate over all subcommands
- For each leaf subcommand, extract:
  - `name`: snake_case combining parent and subcommand (e.g., `issue_get`, `transition_do`)
  - `description`: from the subcommand's about/help text
  - `parameters`: from the subcommand's defined arguments (name, type, description, required)
  - `command_template`: string with `{placeholder}` values (e.g., `jari issue get {key}`)

### Function Signature

```rust
pub fn generate_tool_definitions(cli: &clap::Command) -> Vec<ToolDefinition>
```

### ToolDefinition

```rust
pub struct ToolDefinition {
    pub name: String,                    // "issue_get"
    pub description: String,
    pub command_template: String,        // "jari issue get {key}"
    pub parameters: ToolParameters,
    pub output_schema: Option<serde_json::Value>,  // From schemars
}

pub struct ToolParameters {
    #[serde(rename = "type")]
    pub param_type: String,              // "object"
    pub properties: HashMap<String, ParameterProperty>,
    pub required: Vec<String>,
}

pub struct ParameterProperty {
    #[serde(rename = "type")]
    pub prop_type: String,               // "string", "integer", "boolean"
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
}
```

### Subcommand → Tool Name Mapping

| CLI Path | Tool Name |
|----------|-----------|
| `issue get` | `issue_get` |
| `issue create` | `issue_create` |
| `issue edit` | `issue_edit` |
| `issue delete` | `issue_delete` |
| `issue assign` | `issue_assign` |
| `issue watch add` | `issue_watch_add` |
| `issue watch remove` | `issue_watch_remove` |
| `search` | `search` |
| `transition list` | `transition_list` |
| `transition do` | `transition_do` |
| `comment list` | `comment_list` |
| `comment get` | `comment_get` |
| `comment add` | `comment_add` |
| `project list` | `project_list` |
| `project get` | `project_get` |
| `field list` | `field_list` |
| `me` | `me` |

### Requirements

- Definitions generated at runtime from actual clap definitions — no manual maintenance
- Each parameter gets `type` (string/integer/boolean) inferred from clap argument type
- `required` array lists parameter names that are mandatory
- `command_template` uses `{name}` for positional args and `--name {name}` for optional flags
- Config/connection commands (`config`, `schema`, `completions`) excluded from tool definitions

### File

- `src/schema/generate.rs`
