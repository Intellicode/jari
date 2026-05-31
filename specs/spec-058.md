# Spec 058: Snapshot Tests — JSON Output Format Stability

- [ ] Not implemented

## Goal

Snapshot tests that lock in the JSON output format for each command, catching unintended shape changes.

## Requirements

### Approach

- Use `assert-json-diff` or `insta` crate for snapshot testing
- For each command, run against a `wiremock` server with known fixture
- Capture the full JSON output
- Compare against stored snapshot
- Snapshot files committed to repo
- CI diffs snapshots on failure

### Snapshot Scenarios

- `issue get PROJ-123` — full success shape
- `issue get NOPE-999` — 404 error shape
- `search "project = PROJ"` — search results shape
- `search "project = NONE"` — empty results shape
- `transition list PROJ-123` — transitions array shape
- `transition do PROJ-123 "Done"` — transition result shape
- `comment list PROJ-123` — comments array shape
- `project list` — projects array shape
- `me` — current user shape
- `field list` — fields array shape
- `config show` — config display shape
- `schema --openai` — OpenAI function schema shape
- `schema --anthropic` — Anthropic tool schema shape

### Snapshot Update Process

- `cargo test` with `INSTA_UPDATE=always` or review mode
- Snapshot files stored alongside tests or in `tests/snapshots/`
- CI runs with `INSTA_UPDATE=never` to catch drift

### Test Organization

- `tests/snapshots/` directory
- One snapshot file per scenario, named descriptively
