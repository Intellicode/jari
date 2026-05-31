# Spec 045: Schema — OpenAI Function-Calling Format

- [ ] Not implemented

## Goal

Output schema in OpenAI function-calling format when `jari schema --openai` is invoked.

## Requirements

### CLI Interface

```
jari schema --openai
```

### Output Format

```json
{
  "name": "jari",
  "description": "Jira Cloud CLI for retrieving tasks, updating status, and creating issues. All commands output JSON.",
  "version": "0.1.0",
  "functions": [
    {
      "name": "issue_get",
      "description": "Get full details of a single Jira issue...",
      "parameters": {
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
- Top-level `description`: summary of what jari does for LLMs
- Top-level `version`: from `CARGO_PKG_VERSION`
- `functions` array: one entry per tool-eligible subcommand
- Each function has `name`, `description`, `parameters` (JSON Schema object)
- `required` array for mandatory parameters
- Does NOT include `command_template` or `output_schema` (OpenAI doesn't use them)
- Output to stdout as JSON

### Excluded Commands

- `config`, `schema`, `completions` — not useful for LLM tool calling
