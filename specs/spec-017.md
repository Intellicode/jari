# Spec 017: Wire ADF Conversion into Write Operations (Write Direction)

- [ ] Not implemented

## Goal

Integrate the Markdown → ADF converter so that `--description` and `--comment` inputs are auto-converted before sending to the Jira API.

## Requirements

- When creating an issue: convert `--description` markdown to ADF before POST body
- When editing an issue: convert `--description` markdown to ADF before PUT body
- When adding a comment: convert body markdown to ADF before POST body
- When adding a transition comment: convert `--comment` markdown to ADF
- Handle `@file.md` syntax: read file contents, convert to ADF
- Handle empty/minimal markdown: produce valid minimal ADF `{"type":"doc","version":1,"content":[]}`

### Conversion Pipeline

```
User input (markdown string or @file.md)
  → Read file if @file.md syntax
  → markdown_to_adf(md)
  → Serialize ADF into POST/PUT body
  → Send to Jira API
```

### Error Handling

- File read errors: return `Config` error with "Cannot read file: {path}"
- Conversion errors: return `AdfConversion` error with context

### Files

- Integration in command handlers for `issue create`, `issue edit`, `comment add`, `transition do`
