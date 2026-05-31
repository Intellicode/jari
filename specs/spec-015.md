# Spec 015: Markdown → ADF Converter

- [ ] Not implemented

## Goal

Parse a Markdown string using `pulldown-cmark` and convert it into an ADF JSON document tree.

## Requirements

### Conversion Mapping

| Markdown | ADF Output |
|----------|------------|
| Document | `doc` with version 1, type `"doc"` |
| Paragraph | `paragraph` with `text` children |
| Heading (level N) | `heading` with `attrs.level = N` |
| `**bold**` | `text` with `marks: [{type: "strong"}]` |
| `*italic*` | `text` with `marks: [{type: "em"}]` |
| `` `code` `` | `text` with `marks: [{type: "code"}]` |
| `[text](url)` | `text` with `marks: [{type: "link", attrs: {href: "url"}}]` |
| `~~strike~~` | `text` with `marks: [{type: "strike"}]` |
| Unordered list | `bulletList` > `listItem` > `paragraph` |
| Ordered list | `orderedList` > `listItem` > `paragraph` |
| Code block (fenced) | `codeBlock` with `attrs.language` |
| Blockquote | `blockquote` |
| `---` (horizontal rule) | `rule` |
| GFM table | `table` > `tableRow` > `tableCell` / `tableHeader` |
| `![alt](url)` image | `mediaSingle` > `media` with attrs |
| Task list `- [ ]` / `- [x]` | `taskList` > `taskItem` |
| HTML `<details>` | `expand` node |
| Unknown HTML | Stripped or `text` passthrough |

### API

```rust
pub fn markdown_to_adf(md: &str) -> Result<serde_json::Value, JariError>
```

### Edge Cases

- **Nested marks**: Bold + italic → `[{type: "strong"}, {type: "em"}]`
- **Empty document**: `{"type": "doc", "version": 1, "content": []}`
- **Mentions from markdown**: `@name` treated as plain text in v1 (no user ID resolution)
- **Line breaks**: Single `\n` → `hardBreak`; double `\n\n` → new paragraph
- **Nested lists**: Indentation level drives nesting depth
- **Code blocks**: Capture language from fenced code block info string

### File

- `src/adf/from_markdown.rs`
