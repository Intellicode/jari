# Spec 040: CLI — Comment Add

- [ ] Not implemented

## Goal

Wire the `comment add` subcommand with markdown-to-ADF conversion.

## Requirements

### CLI Interface

```
jari comment add <KEY> <BODY> [--visibility <V>]
```

- `<KEY>`: positional, required (issue key)
- `<BODY>`: positional, required (comment text in markdown)
- `--visibility <V>`: `group:<name>` or `role:<id>`

### Behavior

1. Convert `<BODY>` markdown to ADF
2. Parse `--visibility` if provided (split on `:`, first part is type, rest is value)
3. Call `client.add_comment(key, body_md, visibility)`
4. Output `CreatedComment`

### Output Shape

```json
{
  "ok": true,
  "data": {
    "id": "10234",
    "created": "2026-05-31T10:30:00Z"
  },
  "meta": { "command": "jari comment add PROJ-123 'Fixed in PR #42'", "duration_ms": 300 }
}
```

### Error Handling

- Empty body: `Cli` error — "Comment body is required"
- Issue not found: `NotFound`
- Invalid visibility format: `Cli` error — "Visibility must be group:<name> or role:<id>"
