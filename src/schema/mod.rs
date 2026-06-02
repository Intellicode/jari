use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ParameterProperty {
    #[serde(rename = "type")]
    pub prop_type: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ToolParameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: std::collections::HashMap<String, ParameterProperty>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters,
    pub command_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct OpenAISchema {
    pub name: String,
    pub description: String,
    pub version: String,
    pub functions: Vec<OpenAIFunction>,
}

#[derive(Debug, Serialize)]
pub struct OpenAIFunction {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters,
}

#[derive(Debug, Serialize)]
pub struct AnthropicSchema {
    pub name: String,
    pub description: String,
    pub version: String,
    pub tools: Vec<AnthropicTool>,
}

#[derive(Debug, Serialize)]
pub struct AnthropicTool {
    pub name: String,
    pub description: String,
    pub input_schema: ToolParameters,
}

pub fn generate(openai: bool, anthropic: bool) -> serde_json::Value {
    if openai {
        return serde_json::to_value(generate_openai()).unwrap();
    }
    if anthropic {
        return serde_json::to_value(generate_anthropic()).unwrap();
    }
    serde_json::to_value(generate_tools()).unwrap()
}

fn generate_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "issue_get".into(),
            description: "Get full details of a single Jira issue. Returns description in markdown, status, assignee, priority, and all standard fields.".into(),
            command_template: "jari issue get {key}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [("key".into(), ParameterProperty {
                    prop_type: "string".into(),
                    description: "The issue key (e.g., 'PROJ-123')".into(),
                    default: None,
                }),
                ("fields".into(), ParameterProperty {
                    prop_type: "string".into(),
                    description: "Comma-separated additional fields to include".into(),
                    default: None,
                })].into_iter().collect(),
                required: vec!["key".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "issue_create".into(),
            description: "Create a new Jira issue (story, bug, task, subtask). Description accepts markdown which is auto-converted to Jira's rich text format.".into(),
            command_template: "jari issue create --project {project} --summary '{summary}'".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("project".into(), ParameterProperty { prop_type: "string".into(), description: "Project key (required)".into(), default: None }),
                    ("summary".into(), ParameterProperty { prop_type: "string".into(), description: "Issue title/summary (required)".into(), default: None }),
                    ("type".into(), ParameterProperty { prop_type: "string".into(), description: "Issue type: Story, Bug, Task, Subtask".into(), default: Some(serde_json::json!("Task")) }),
                    ("description".into(), ParameterProperty { prop_type: "string".into(), description: "Issue description in markdown".into(), default: None }),
                    ("priority".into(), ParameterProperty { prop_type: "string".into(), description: "Priority: Highest, High, Medium, Low, Lowest".into(), default: None }),
                    ("assignee".into(), ParameterProperty { prop_type: "string".into(), description: "Assignee account ID or email".into(), default: None }),
                    ("labels".into(), ParameterProperty { prop_type: "string".into(), description: "Comma-separated labels".into(), default: None }),
                    ("parent".into(), ParameterProperty { prop_type: "string".into(), description: "Parent issue key (for subtasks)".into(), default: None }),
                    ("epic_link".into(), ParameterProperty { prop_type: "string".into(), description: "Epic issue key to link".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["project".into(), "summary".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "issue_edit".into(),
            description: "Edit issue fields (summary, description, priority, labels).".into(),
            command_template: "jari issue edit {key}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key (e.g., 'PROJ-123')".into(), default: None }),
                    ("summary".into(), ParameterProperty { prop_type: "string".into(), description: "New summary".into(), default: None }),
                    ("description".into(), ParameterProperty { prop_type: "string".into(), description: "New description in markdown".into(), default: None }),
                    ("priority".into(), ParameterProperty { prop_type: "string".into(), description: "New priority".into(), default: None }),
                    ("labels".into(), ParameterProperty { prop_type: "string".into(), description: "New labels (replaces existing)".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["key".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "issue_delete".into(),
            description: "Delete an issue (requires --force flag).".into(),
            command_template: "jari issue delete {key} --force".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key to delete".into(), default: None }),
                    ("force".into(), ParameterProperty { prop_type: "boolean".into(), description: "Skip confirmation".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["key".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "issue_assign".into(),
            description: "Assign an issue to a user. Use 'me' to assign to yourself or 'unassigned' to remove assignment.".into(),
            command_template: "jari issue assign {key} {user}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key".into(), default: None }),
                    ("user".into(), ParameterProperty { prop_type: "string".into(), description: "Account ID, email, 'me', or 'unassigned'".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["key".into(), "user".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "issue_watch_add".into(),
            description: "Start watching an issue (current user).".into(),
            command_template: "jari issue watch add {key}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key".into(), default: None })].into_iter().collect(),
                required: vec!["key".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "issue_watch_remove".into(),
            description: "Stop watching an issue (current user).".into(),
            command_template: "jari issue watch remove {key}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key".into(), default: None })].into_iter().collect(),
                required: vec!["key".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "search".into(),
            description: "Search Jira issues using JQL. Automatically fetches ALL pages — no manual pagination needed. Returns up to 1000 issues.".into(),
            command_template: "jari search '{jql}'".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("jql".into(), ParameterProperty { prop_type: "string".into(), description: "JQL query. Examples: 'project = PROJ AND assignee = currentUser()', 'status = \"In Progress\" ORDER BY priority DESC'".into(), default: None }),
                    ("max".into(), ParameterProperty { prop_type: "integer".into(), description: "Maximum results (default: all, capped at 1000)".into(), default: None }),
                    ("fields".into(), ParameterProperty { prop_type: "string".into(), description: "Comma-separated fields (default: summary,status,assignee,priority,issuetype,created,updated)".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["jql".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "transition_list".into(),
            description: "List all available workflow transitions for an issue. Returns transition IDs and names.".into(),
            command_template: "jari transition list {key}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key (e.g., 'PROJ-123')".into(), default: None })].into_iter().collect(),
                required: vec!["key".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "transition_do".into(),
            description: "Execute a workflow transition on an issue (e.g., move to 'In Progress', 'Done'). Use transition_list first to see available transitions.".into(),
            command_template: "jari transition do {key} {transition}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key".into(), default: None }),
                    ("transition".into(), ParameterProperty { prop_type: "string".into(), description: "Transition ID or name (partial match supported, e.g., 'In Progress', 'Done')".into(), default: None }),
                    ("comment".into(), ParameterProperty { prop_type: "string".into(), description: "Optional comment to add during transition (markdown)".into(), default: None }),
                    ("resolution".into(), ParameterProperty { prop_type: "string".into(), description: "Optional resolution name to set (e.g., 'Done', 'Won\\'t Do')".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["key".into(), "transition".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "comment_list".into(),
            description: "List comments on an issue (newest first). Comments are auto-converted to markdown.".into(),
            command_template: "jari comment list {key}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key".into(), default: None }),
                    ("max".into(), ParameterProperty { prop_type: "integer".into(), description: "Maximum comments (default: all)".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["key".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "comment_get".into(),
            description: "Get a specific comment by ID.".into(),
            command_template: "jari comment get {key} {id}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key".into(), default: None }),
                    ("id".into(), ParameterProperty { prop_type: "string".into(), description: "Comment ID".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["key".into(), "id".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "comment_add".into(),
            description: "Add a comment to an issue. Comment text is markdown auto-converted to Jira's rich text.".into(),
            command_template: "jari comment add {key} '{body}'".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [
                    ("key".into(), ParameterProperty { prop_type: "string".into(), description: "Issue key".into(), default: None }),
                    ("body".into(), ParameterProperty { prop_type: "string".into(), description: "Comment text in markdown".into(), default: None }),
                    ("visibility".into(), ParameterProperty { prop_type: "string".into(), description: "Visibility: group:<name> or role:<id>".into(), default: None }),
                ].into_iter().collect(),
                required: vec!["key".into(), "body".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "project_list".into(),
            description: "List all accessible Jira projects.".into(),
            command_template: "jari project list".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [("type".into(), ParameterProperty { prop_type: "string".into(), description: "Filter: software, service_desk, business".into(), default: None })].into_iter().collect(),
                required: vec![],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "project_get".into(),
            description: "Get project details including lead, versions, components, and issue types.".into(),
            command_template: "jari project get {key}".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [("key".into(), ParameterProperty { prop_type: "string".into(), description: "Project key".into(), default: None })].into_iter().collect(),
                required: vec!["key".into()],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "field_list".into(),
            description: "List all fields (system + custom) with IDs and metadata. Use this to discover custom fields before creating/editing issues.".into(),
            command_template: "jari field list".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [].into_iter().collect(),
                required: vec![],
            },
            output_schema: None,
        },
        ToolDefinition {
            name: "me".into(),
            description: "Get the currently authenticated Jira user's information (accountId, name, email).".into(),
            command_template: "jari me".into(),
            parameters: ToolParameters {
                param_type: "object".into(),
                properties: [].into_iter().collect(),
                required: vec![],
            },
            output_schema: None,
        },
    ]
}

fn generate_openai() -> OpenAISchema {
    let tools = generate_tools();
    OpenAISchema {
        name: "jari".into(),
        description: "Jira Cloud CLI for retrieving tasks, updating status, and creating issues. All commands output JSON.".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        functions: tools.into_iter().map(|t| OpenAIFunction {
            name: t.name,
            description: t.description,
            parameters: t.parameters,
        }).collect(),
    }
}

fn generate_anthropic() -> AnthropicSchema {
    let tools = generate_tools();
    AnthropicSchema {
        name: "jari".into(),
        description: "Jira Cloud CLI for retrieving tasks, updating status, and creating issues. All commands output JSON.".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        tools: tools.into_iter().map(|t| AnthropicTool {
            name: t.name,
            description: t.description,
            input_schema: t.parameters,
        }).collect(),
    }
}
