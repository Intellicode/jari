# Spec 026: CLI — Comment List & Get

- [ ] Not implemented

## Goal

Wire the `comment list` and `comment get` subcommands.

## Requirements

### comment list

```
jari comment list <KEY> [--max <N>]
```

- `<KEY>`: positional, required (issue key)
- `--max <N>`: limit comments (default: all)
- Calls `client.list_comments(key, max)`
- Output: array of `Comment` with markdown bodies

### comment get

```
jari comment get <KEY> <ID>
```

- `<KEY>`: positional, required (issue key)
- `<ID>`: positional, required (comment ID)
- Calls `client.get_comment(key, id)`
- Output: single `Comment` with markdown body

### Output Shape

List:
```json
{
  "ok": true,
  "data": [
    { "id": "12345", "author": {...}, "body_markdown": "...", "created": "...", "updated": "..." }
  ],
  "meta": { "command": "jari comment list PROJ-123", "duration_ms": 123 }
}
```

### Error Handling

- Issue not found: `NotFound`
- Comment not found: `NotFound`
- No comments: empty array (not an error)
