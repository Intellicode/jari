# Spec 034: CLI — Issue Create

- [ ] Not implemented

## Goal

Wire the `issue create` subcommand with all optional flags and markdown-to-ADF conversion.

## Requirements

### CLI Interface

```
jari issue create
    --project <KEY>          Required. Project key.
    --summary <TEXT>         Required. Issue title.
    --type <TYPE>            Issue type: Story, Bug, Task, etc. [default: Task]
    --description <TEXT|@FILE>  Description in markdown. Use @file.md for file input.
    --priority <PRI>         Priority: Highest, High, Medium, Low, Lowest
    --assignee <USER>        Assignee account ID or email
    --labels <LABELS>        Comma-separated labels
    --parent <KEY>           Parent issue key (for subtasks)
    --epic-link <KEY>        Epic link key
```

### Behavior

1. Parse all flags
2. If `--description` starts with `@`, read file contents (strip `@` prefix)
3. Convert description markdown to ADF via `markdown_to_adf()`
4. Parse `--labels` comma-separated string to `Vec<String>`
5. Build `CreateIssueRequest` with all fields
6. Call `client.create_issue()`
7. Output `CreatedIssue`

### Output Shape

```json
{
  "ok": true,
  "data": {
    "key": "PROJ-456",
    "id": "10042",
    "url": "https://company.atlassian.net/browse/PROJ-456"
  },
  "meta": { "command": "jari issue create --project PROJ --summary 'Add dark mode'", "duration_ms": 500 }
}
```

### Error Handling

- Missing required fields: `Cli` error
- Invalid project: `Validation` error from Jira
- Invalid issue type: `Validation` error from Jira
- File not found (`@file.md`): `Config` error with path
- ADF conversion failure: `AdfConversion` error
