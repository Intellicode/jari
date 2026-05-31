# Spec 051: Unit Tests — ADF Converter

- [ ] Not implemented

## Goal

Comprehensive unit tests for the ADF ↔ Markdown converter covering all conversion mappings and edge cases.

## Requirements

### ADF → Markdown Tests

- Test each node type from the mapping table (at least 25 node types):
  - `doc`, `paragraph`, `heading` (all 6 levels), `text` (plain), `text` + `strong`, `text` + `em`, `text` + `code`, `text` + `link`, `text` + `strike`, `text` + `underline`, `text` + `subsup`, `text` + `textColor`
  - `bulletList`, `orderedList`, `listItem`, `codeBlock`, `blockquote`, `rule`, `mention`, `emoji`, `hardBreak`
  - `table`, `tableRow`, `tableCell`, `tableHeader`, `mediaSingle`/`media`, `inlineCard`, `blockCard`, `panel`, `taskList`, `taskItem`, `date`, `status`, `expand`, `placeholder`
- Test nested marks: `strong` + `em` = `***text***`
- Test unknown node: debug `<pre><code>` passthrough
- Test empty document: `""`
- Test deeply nested lists (3+ levels)

### Markdown → ADF Tests

- Test each markdown construct:
  - Paragraphs, headings (all levels), bold, italic, code, links, strikethrough
  - Unordered lists, ordered lists, nested lists
  - Code blocks (with and without language)
  - Blockquotes, horizontal rules
  - Tables (GFM), images, task lists
- Test empty markdown: minimal ADF doc
- Test combined marks: bold + italic
- Test HTML passthrough: `<details>` → `expand`

### Round-Trip Tests

- For each formatting type: markdown → ADF → markdown
- Assert semantic equivalence (exact string match may not always hold, use lenient comparison)
- Test with real Jira ADF fixtures (at least 5 diverse examples)

### Test Organization

- Unit tests inline with `#[cfg(test)] mod tests` in:
  - `src/adf/to_markdown.rs`
  - `src/adf/from_markdown.rs`
- Integration-level ADF tests in `tests/integration/adf.rs`
- Fixture files in `tests/fixtures/adf/`
