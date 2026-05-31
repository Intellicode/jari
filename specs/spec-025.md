# Spec 025: CLI — Search Subcommand

- [ ] Not implemented

## Goal

Wire the `search` subcommand to run JQL queries and output results.

## Requirements

### CLI Interface

```
jari search <JQL> [--max <N>] [--fields <FIELDS>]
```

- `<JQL>`: positional argument, required
- `--max <N>`: limit results (default: fetch all, capped at 1000)
- `--fields <FIELDS>`: comma-separated field names (default: `summary,status,assignee,priority,issuetype`)

### Behavior

- Parse JQL, fields, max from CLI args
- Call `client.search(jql, fields, max)`
- Convert search results to `Output<Vec<IssueSummary>>`
- Output JSON with `ok: true`, `data: [...]`, `meta.total_results` in data or meta

### Output Shape

```json
{
  "ok": true,
  "data": [
    { "key": "PROJ-123", "fields": { "summary": "...", "status": {...}, ... } },
    ...
  ],
  "meta": {
    "command": "jari search 'project = PROJ'",
    "duration_ms": 456,
    "total_results": 42
  }
}
```

### Error Handling

- Empty JQL: return `Cli` error with "JQL query is required"
- Invalid JQL: return `Validation` error with Jira's error details
- No results: return empty array (not an error)

### File

- Command handler in `src/cli.rs` or dedicated handler module
