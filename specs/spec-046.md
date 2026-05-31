# Spec 046: Schema — Anthropic Tool-Use Format

- [ ] Not implemented

## Goal

Output schema in Anthropic tool-use format when `jari schema --anthropic` is invoked.

## Requirements

### CLI Interface

```
jari schema --anthropic
```

### Output Format

```json
{
  "name": "jari",
  "description": "Jira Cloud CLI for retrieving tasks, updating status, and creating issues. All commands output JSON.",
  "version": "0.1.0",
  "tools": [
    {
      "name": "issue_get",
      "description": "Get full details of a single Jira issue...",
      "input_schema": {
        "type": "object",
        "properties": {
          "key": {
            "type": "string",
            "description": "The issue key (e.g., 'PROJ-123')"
          },
          "fields": {
            "type": "string",
            "description": "Comma-separated additional fields to include"
          }
        },
        "required": ["key"]
      }
    },
    ...
  ]
}
```

### Requirements

- Top-level `name`: `"jari"`
- Top-level `description`: purpose overview
- `tools` array: one entry per tool-eligible subcommand
- Each tool has `name`, `description`, `input_schema` (JSON Schema object)
- `input_schema.required` for mandatory parameters
- Does NOT include `command_template` or `output_schema`
- Output to stdout as JSON

### Comparison with OpenAI Format

- `tools` instead of `functions`
- `input_schema` instead of `parameters`
- Otherwise structurally identical
