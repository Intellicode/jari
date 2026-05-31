# Spec 014: ADF → Markdown Converter

- [ ] Not implemented

## Goal

Implement a recursive walker that converts an ADF JSON document tree into a clean Markdown string.

## Requirements

### Conversion Mapping

| ADF Node Type | Markdown Output |
|---------------|-----------------|
| `doc` | Traverse children (no output) |
| `paragraph` | Text content + `\n\n` |
| `heading` (level 1-6) | `#` × level + ` ` + text + `\n\n` |
| `text` (plain) | Literal text content |
| `text` + `strong` mark | `**text**` |
| `text` + `em` mark | `*text*` |
| `text` + `code` mark | `` `text` `` |
| `text` + `link` mark | `[text](href)` using `attrs.href` |
| `text` + `strike` mark | `~~text~~` |
| `text` + `underline` mark | `<u>text</u>` (no native markdown) |
| `text` + `subsup` mark | `<sub>text</sub>` / `<sup>text</sup>` |
| `text` + `textColor` mark | Plain text (color lost) |
| `bulletList` | `- ` per `listItem` child |
| `orderedList` | `1. ` per `listItem` (sequential numbering) |
| `listItem` | Content indented + `\n` |
| `codeBlock` | ` ``` ` + language + `\n` + code + `\n` + ` ``` ` + `\n\n` |
| `blockquote` | `> ` prefixed per line |
| `rule` | `---\n\n` |
| `mention` | `@` + `attrs.displayName` (fallback: `@` + `attrs.id`) |
| `emoji` | `:` + `attrs.shortName` + `:` |
| `hardBreak` | `\n` (single newline) |
| `table` | GFM table with header separator row |
| `tableRow` | `\| cell1 \| cell2 \|` |
| `tableCell` | Cell text |
| `tableHeader` | Like `tableCell` but triggers separator row |
| `mediaSingle` / `media` | `![alt](url)` using `attrs.alt` or filename |
| `inlineCard` | `[url](url)` using `attrs.url` |
| `blockCard` | `[title](url)` using `attrs.data.title` |
| `panel` | `> **type:** content` (admonition-style) |
| `taskList` | `- [x] ` or `- [ ] ` per `taskItem` |
| `taskItem` | Checkbox + content |
| `date` | ISO date string from `attrs.timestamp` |
| `status` | `[STATUS: text]` |
| `expand` | `<details><summary>title</summary>content</details>` |
| `placeholder` | `[PLACEHOLDER: text]` |
| Unknown node | `<pre><code>JSON</code></pre>` debug passthrough |

### API

```rust
pub fn adf_to_markdown(json: &serde_json::Value) -> Result<String, JariError>
```

### Edge Cases

- **Nested marks**: Bold + italic = `***text***`
- **Empty document**: Returns empty string `""`
- **Soft error handling**: Unknown nodes render as debug passthrough, never fail
- **Nested lists**: Handle indentation-based nesting
- **Deeply nested structures**: Iterative or stack-safe recursion

### File

- `src/adf/to_markdown.rs`
