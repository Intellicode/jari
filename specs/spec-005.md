# Spec 005: JSON Output Envelope

- [ ] Not implemented

## Goal

Define the standardized JSON output format that wraps all command responses — both success and error — for LLM consumption.

## Requirements

### Success Shape

```json
{
  "ok": true,
  "data": { ... },
  "meta": {
    "command": "jari issue get PROJ-123",
    "duration_ms": 234
  }
}
```

### Error Shape

```json
{
  "ok": false,
  "error": {
    "code": "not_found",
    "message": "Issue does not exist...",
    "http_status": 404,
    "jira_errors": { "errorMessages": [...], "errors": {} },
    "suggestion": "Verify the issue key..."
  },
  "meta": {
    "command": "jari issue get NOPE-999",
    "duration_ms": 312
  }
}
```

### Design Principles

- `ok: bool` at top level — LLM branches immediately on success/failure
- `data` shape is stable and predictable per command
- `error.code` is a stable enum string — LLM handles specific error types
- `error.suggestion` gives actionable fix guidance
- `meta.command` reproduces the exact invocation
- `meta.duration_ms` measured from CLI start to finish
- **stdout**: Always valid JSON (success or error)
- **stderr**: Log messages only
- `--output json` (default): compact single-line JSON
- `--output json-pretty`: indented, 2-space JSON

### Implementation

- `Output<T>` generic struct: `ok: bool`, `data: Option<T>`, `error: Option<OutputError>`, `meta: OutputMeta`
- `Output::success(data, command)` constructor
- `Output::error(err, command)` constructor from `JariError`
- `OutputMeta` populated automatically with command string and timing
- Timing measured in `main.rs` around the dispatch
