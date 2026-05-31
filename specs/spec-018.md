# Spec 018: ADF Converter Unit Tests

- [ ] Not implemented

## Goal

Comprehensive unit tests for the ADF ↔ Markdown converter covering all node types and edge cases.

## Requirements

### Round-Trip Tests

- For each major formatting type, test: markdown → ADF → markdown produces semantically equivalent output
- Test types: bold, italic, code, links, strikethrough, headings (1-6), bullet lists, ordered lists, code blocks, blockquotes, horizontal rules, tables, task lists, images, combined marks (bold+italic)

### Specific ADF Fixtures

- Test with real ADF JSON from Jira (at least 5 diverse fixtures)
- Test with empty document (`{"type":"doc","version":1,"content":[]}`)
- Test with deeply nested structure (list in list in list)
- Test with rich text containing all mark types simultaneously

### Edge Cases

- **Empty document**: ADF → MD = `""`, MD → ADF = minimal doc
- **Deeply nested lists**: Verify correct indentation
- **Code blocks with language**: Verify language preserved in round-trip
- **Tables with empty cells**: Verify correct handling
- **Unknown ADF nodes**: Verify debug passthrough, no panic
- **HTML in markdown**: Verify `<details>` → `expand` node
- **Links without text**: Verify `[url](url)` rendering
- **Consecutive hard breaks**: Verify single vs double newline semantics
- **Mention fallback**: Verify `@displayName` output when no user ID
- **Unicode/emoji**: Verify emoji nodes and inline emoji survive conversion

### Test Organization

- `tests/integration/adf.rs` for round-trip and fixture-based tests
- `#[cfg(test)] mod tests` blocks in `adf/to_markdown.rs` and `adf/from_markdown.rs` for unit-level tests
