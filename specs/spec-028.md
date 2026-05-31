# Spec 028: CLI — Field List

- [ ] Not implemented

## Goal

Wire the `field list` subcommand.

## Requirements

### CLI Interface

```
jari field list
```

- No arguments required
- Calls `client.list_fields()`
- Output: array of `Field` sorted alphabetically by name

### Output Shape

```json
{
  "ok": true,
  "data": [
    { "id": "assignee", "name": "Assignee", "custom": false, "schema": {...} },
    { "id": "customfield_10010", "name": "Story Points", "custom": true, "schema": {...} },
    ...
  ],
  "meta": { "command": "jari field list", "duration_ms": 150 }
}
```

### Use Case

LLMs use this to:
- Discover available custom fields before creating/editing issues
- Identify which fields have ADF content (schema type `"doc"`)
- Learn field IDs for use in `--fields` flags

### Error Handling

- API error: propagate as JSON error
