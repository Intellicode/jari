# Spec 035: CLI — Issue Edit

- [ ] Not implemented

## Goal

Wire the `issue edit` subcommand for partial issue updates.

## Requirements

### CLI Interface

```
jari issue edit <KEY>
    --summary <TEXT>         New summary
    --description <TEXT|@FILE>  New description (markdown)
    --priority <PRI>         New priority
    --labels <LABELS>        New labels (replaces existing)
    --add-label <LBL>        Add a label (can repeat)
```

### Behavior

- At least one edit flag must be provided
- Build a `HashMap<String, Value>` of only the specified fields
- Convert `--description` markdown to ADF if provided (including `@file.md`)
- `--labels` replaces all labels; `--add-label` adds to existing (multiple allowed)
- Call `client.edit_issue(key, fields)`
- Output: confirmation with updated fields or empty success

### Output Shape

```json
{
  "ok": true,
  "data": {
    "key": "PROJ-123",
    "updated_fields": ["summary", "priority"]
  },
  "meta": { "command": "jari issue edit PROJ-123 --priority High", "duration_ms": 300 }
}
```

### Error Handling

- No edit flags: `Cli` error — "At least one field to edit is required"
- Issue not found: `NotFound`
- Invalid field values: `Validation` with Jira error details
