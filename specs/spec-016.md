# Spec 016: Wire ADF Conversion into Issue Get (Read Direction)

- [ ] Not implemented

## Goal

Integrate the ADF → Markdown converter into the issue retrieval pipeline so descriptions appear as clean Markdown.

## Requirements

- After fetching an `Issue`, extract the raw ADF `description` field
- Convert ADF to Markdown via `adf_to_markdown()`
- Store the result in `IssueFields.description_markdown`
- Remove or hide the raw ADF `description` field from final output
- Also convert `comment.body` ADF to `comment.body_markdown` for embedded comments
- On conversion failure: set `description_markdown` to a fallback like `"[ADF conversion error: <message>]"`
- On null/missing description: `description_markdown` remains `None`
- Handle nested ADF in custom fields: pass through as-is (identified by field schema type `"doc"`)

### Fallback / Raw Access

- Add `--raw` flag to `issue get` that returns the raw ADF JSON instead of converted markdown
- This allows LLMs to access original formatting when conversion is lossy

### File

- Integration in `src/client/issues.rs` or the command handler in `src/cli.rs`
