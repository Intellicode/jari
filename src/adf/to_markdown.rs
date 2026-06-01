use crate::adf::{Doc, Mark, Node};
use crate::error::JariError;

pub fn adf_to_markdown(json: &serde_json::Value) -> Result<String, JariError> {
    let doc: Doc = serde_json::from_value(json.clone())
        .map_err(|e| JariError::AdfConversion(format!("Failed to parse ADF JSON: {}", e)))?;

    if doc.node_type != "doc" && doc.node_type != "Doc" {
        return Err(JariError::AdfConversion(
            "Root node type must be 'doc'".into(),
        ));
    }

    let mut output = String::new();
    for node in &doc.content {
        output.push_str(&node_to_markdown(node));
    }
    Ok(output)
}

fn node_to_markdown(node: &Node) -> String {
    match node.node_type.as_str() {
        "doc" => {
            if let Some(ref children) = node.content {
                children.iter().map(node_to_markdown).collect::<String>()
            } else {
                String::new()
            }
        }

        "paragraph" => {
            let content = children_to_markdown_inline(node);
            format!("{}\n\n", content)
        }

        "heading" => {
            let level = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("level"))
                .and_then(|v| v.as_u64())
                .unwrap_or(1);
            let hashes = "#".repeat(level as usize);
            let content = children_to_markdown_inline(node);
            format!("{} {}\n\n", hashes, content)
        }

        "text" => {
            let text = node.text.as_deref().unwrap_or("");
            apply_marks(text, &node.marks)
        }

        "bulletList" => {
            let mut out = String::new();
            if let Some(ref items) = node.content {
                for item in items {
                    out.push_str(&list_item_to_markdown(item, "- ", ""));
                }
            }
            if !out.is_empty() {
                out.push('\n');
            }
            out
        }

        "orderedList" => {
            let mut out = String::new();
            if let Some(ref items) = node.content {
                for (i, item) in items.iter().enumerate() {
                    let num = i + 1;
                    out.push_str(&list_item_to_markdown(item, &format!("{}. ", num), ""));
                }
            }
            if !out.is_empty() {
                out.push('\n');
            }
            out
        }

        "listItem" => children_to_markdown_inline(node),

        "codeBlock" => {
            let lang = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("language"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let code = node
                .content
                .as_ref()
                .and_then(|c| c.first())
                .and_then(|n| n.text.as_deref())
                .unwrap_or("");
            format!("```{}\n{}\n```\n\n", lang, code)
        }

        "blockquote" => {
            let content = children_to_markdown(node);
            let mut out = String::new();
            for line in content.lines() {
                if line.is_empty() {
                    out.push_str(">\n");
                } else {
                    out.push_str(&format!("> {}\n", line));
                }
            }
            if !out.is_empty() {
                out.push('\n');
            }
            out
        }

        "rule" => "---\n\n".to_string(),

        "mention" => {
            let id = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("id"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let display_name = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("displayName"))
                .and_then(|v| v.as_str())
                .unwrap_or(id);
            format!("@{}", display_name)
        }

        "emoji" => {
            let short_name = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("shortName"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            format!(":{}:", short_name)
        }

        "hardBreak" => "\n".to_string(),

        "table" => {
            let mut out = String::new();
            let mut all_rows: Vec<&Node> = Vec::new();
            let mut has_header = false;

            if let Some(ref children) = node.content {
                for child in children {
                    if child.node_type == "tableRow" {
                        all_rows.push(child);
                    } else if child.node_type == "tableHeader" {
                        if let Some(ref header_content) = child.content {
                            // tableHeader wraps a single tableRow
                            if let Some(row) = header_content.first() {
                                all_rows.push(row);
                                has_header = true;
                            }
                        }
                    }
                }
            }

            if all_rows.is_empty() {
                return String::new();
            }

            let col_count = all_rows
                .iter()
                .map(|r| r.content.as_ref().map(|c| c.len()).unwrap_or(0))
                .max()
                .unwrap_or(0);

            if col_count == 0 {
                return String::new();
            }

            let get_cell_text = |row: &Node| -> Vec<String> {
                row.content
                    .as_ref()
                    .map(|cells| {
                        cells
                            .iter()
                            .map(|cell| children_to_markdown_inline(cell).trim().to_string())
                            .collect()
                    })
                    .unwrap_or_default()
            };

            let all_cells: Vec<Vec<String>> = all_rows.iter().map(|r| get_cell_text(r)).collect();

            let start_idx = if has_header { 1 } else { 0 };

            if has_header {
                if let Some(cells) = all_cells.first() {
                    out.push_str(&format_cells(cells));
                    out.push('\n');
                }
                // separator row
                let seps: Vec<String> = (0..col_count).map(|_| "---".to_string()).collect();
                out.push_str(&format_cells(&seps));
                out.push('\n');
            } else {
                let seps: Vec<String> = (0..col_count).map(|_| "---".to_string()).collect();
                out.push_str(&format_cells(&seps));
                out.push('\n');
            }

            for cell in all_cells.iter().skip(start_idx) {
                out.push_str(&format_cells(cell));
                out.push('\n');
            }

            out.push('\n');
            out
        }

        "tableRow" => {
            let cells: Vec<String> = node
                .content
                .as_ref()
                .map(|c| {
                    c.iter()
                        .map(|cell| children_to_markdown_inline(cell).trim().to_string())
                        .collect()
                })
                .unwrap_or_default();
            if cells.is_empty() {
                String::new()
            } else {
                let mut line = String::from("| ");
                for cell in &cells {
                    line.push_str(cell);
                    line.push_str(" | ");
                }
                line.push('\n');
                line
            }
        }

        "tableCell" | "tableHeader" => children_to_markdown_inline(node),

        "mediaSingle" => {
            if let Some(ref children) = node.content {
                for child in children {
                    if child.node_type == "media" {
                        return node_to_markdown(child) + "\n\n";
                    }
                }
            }
            String::new()
        }

        "media" => {
            let url = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("url"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let alt = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("alt"))
                .and_then(|v| v.as_str())
                .or(url.split('/').next_back().and_then(|f| f.split('?').next()))
                .unwrap_or("image");
            format!("![{}]({})", alt, url)
        }

        "inlineCard" => {
            let url = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("url"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            format!("[{}]({})", url, url)
        }

        "blockCard" => {
            let url = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("url"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let title = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("data"))
                .and_then(|d| d.get("title"))
                .and_then(|v| v.as_str())
                .unwrap_or(url);
            format!("[{}]({})", title, url)
        }

        "panel" => {
            let panel_type = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("panelType"))
                .and_then(|v| v.as_str())
                .unwrap_or("info");
            let content = children_to_markdown(node).trim().to_string();
            let mut out = String::new();
            out.push_str(&format!("> **{}:** {}\n", panel_type, content));
            out.push('\n');
            out
        }

        "taskList" => {
            let mut out = String::new();
            if let Some(ref items) = node.content {
                for item in items {
                    out.push_str(&task_item_to_markdown(item));
                }
            }
            if !out.is_empty() {
                out.push('\n');
            }
            out
        }

        "taskItem" => {
            let checked = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("state"))
                .and_then(|v| v.as_str())
                .map(|s| s == "DONE")
                .unwrap_or(false);
            let checkbox = if checked { "- [x] " } else { "- [ ] " };
            let content = children_to_markdown_inline(node);
            format!("{}{}\n", checkbox, content)
        }

        "date" => node
            .attrs
            .as_ref()
            .and_then(|a| a.get("timestamp"))
            .and_then(|v| v.as_str())
            .map(|s| {
                let ms: Result<i64, _> = s.parse();
                match ms {
                    Ok(ts) => {
                        let date = if ts > 1_000_000_000_000 {
                            ts / 1000
                        } else {
                            ts
                        };
                        chrono_like(date)
                    }
                    Err(_) => s.to_string(),
                }
            })
            .unwrap_or_default(),

        "status" => {
            let text = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("text"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            format!("[STATUS: {}]", text)
        }

        "expand" => {
            let title = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("title"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let content = children_to_markdown(node);
            format!(
                "<details><summary>{}</summary>\n\n{}\n\n</details>",
                title, content
            )
        }

        "placeholder" => {
            let text = node
                .attrs
                .as_ref()
                .and_then(|a| a.get("text"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            format!("[PLACEHOLDER: {}]", text)
        }

        _ => {
            let json_str = serde_json::to_string_pretty(node).unwrap_or_default();
            format!("<pre><code>{}</code></pre>", json_str)
        }
    }
}

fn children_to_markdown_inline(node: &Node) -> String {
    match &node.content {
        Some(children) => children.iter().map(node_to_markdown).collect::<String>(),
        None => String::new(),
    }
}

fn children_to_markdown(node: &Node) -> String {
    match &node.content {
        Some(children) => children.iter().map(node_to_markdown).collect::<String>(),
        None => String::new(),
    }
}

fn list_item_to_markdown(item: &Node, prefix: &str, _indent: &str) -> String {
    let content = children_to_markdown_inline(item);
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() {
        return format!("{}\n", prefix);
    }

    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            out.push_str(&format!("{}{}\n", prefix, line));
        } else {
            out.push_str(&format!("  {}\n", line));
        }
    }
    out
}

fn task_item_to_markdown(item: &Node) -> String {
    let checked = item
        .attrs
        .as_ref()
        .and_then(|a| a.get("state"))
        .and_then(|v| v.as_str())
        .map(|s| s == "DONE")
        .unwrap_or(false);
    let checkbox = if checked { "- [x] " } else { "- [ ] " };
    let content = children_to_markdown_inline(item);
    format!("{}{}\n", checkbox, content.trim())
}

fn format_cells(cells: &[String]) -> String {
    let mut line = String::from("| ");
    for cell in cells {
        line.push_str(cell);
        line.push_str(" | ");
    }
    line
}

fn apply_marks(text: &str, marks: &Option<Vec<Mark>>) -> String {
    let marks = match marks {
        Some(m) => m,
        None => return text.to_string(),
    };

    if marks.is_empty() {
        return text.to_string();
    }

    let has_strong = marks.iter().any(|m| m.mark_type == "strong");
    let has_em = marks.iter().any(|m| m.mark_type == "em");
    let has_code = marks.iter().any(|m| m.mark_type == "code");
    let has_strike = marks.iter().any(|m| m.mark_type == "strike");
    let has_link = marks.iter().any(|m| m.mark_type == "link");
    let has_underline = marks.iter().any(|m| m.mark_type == "underline");
    let _has_subsup = marks.iter().any(|m| m.mark_type == "subsup");
    let has_text_color = marks.iter().any(|m| m.mark_type == "textColor");

    let mut result = text.to_string();

    if has_code {
        result = format!("`{}`", result);
    }

    if let Some(mark) = marks.iter().find(|m| m.mark_type == "subsup") {
        let subsup_type = mark
            .attrs
            .as_ref()
            .and_then(|a| a.get("type"))
            .and_then(|v| v.as_str())
            .unwrap_or("sub");
        result = format!("<{}>{}</{}>", subsup_type, result, subsup_type);
    }

    if has_underline {
        result = format!("<u>{}</u>", result);
    }

    if has_strike {
        result = format!("~~{}~~", result);
    }

    if has_strong && has_em {
        result = format!("***{}***", result);
    } else {
        if has_em {
            result = format!("*{}*", result);
        }
        if has_strong {
            result = format!("**{}**", result);
        }
    }

    if has_link {
        let href = marks
            .iter()
            .filter(|m| m.mark_type == "link")
            .filter_map(|m| {
                m.attrs
                    .as_ref()
                    .and_then(|a| a.get("href"))
                    .and_then(|v| v.as_str())
            })
            .next()
            .unwrap_or("");
        result = format!("[{}]({})", result, href);
    }

    if has_text_color {
        // Color is lost in markdown, just return plain text
    }

    result
}

fn chrono_like(timestamp_secs: i64) -> String {
    // Simple ISO date extraction from unix timestamp for formatting
    let secs = timestamp_secs;
    let days = secs / 86400;

    // Days since epoch (1970-01-01)
    let mut year = 1970;
    let mut remaining_days = days;

    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let month_lengths = if is_leap(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &ml in &month_lengths {
        if remaining_days < ml {
            break;
        }
        remaining_days -= ml;
        month += 1;
    }

    let day = remaining_days + 1;

    let remaining_secs = secs % 86400;
    let hours = remaining_secs / 3600;
    let minutes = (remaining_secs % 3600) / 60;
    let seconds = remaining_secs % 60;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_document() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": []
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_plain_paragraph() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "Hello world"
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "Hello world\n\n");
    }

    #[test]
    fn test_bold_text() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "bold",
                    "marks": [{"type": "strong"}]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "**bold**\n\n");
    }

    #[test]
    fn test_italic_text() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "italic",
                    "marks": [{"type": "em"}]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "*italic*\n\n");
    }

    #[test]
    fn test_bold_italic_text() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "both",
                    "marks": [{"type": "strong"}, {"type": "em"}]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "***both***\n\n");
    }

    #[test]
    fn test_code_text() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "fn main()",
                    "marks": [{"type": "code"}]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "`fn main()`\n\n");
    }

    #[test]
    fn test_link_text() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "click here",
                    "marks": [{
                        "type": "link",
                        "attrs": {"href": "https://example.com"}
                    }]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "[click here](https://example.com)\n\n");
    }

    #[test]
    fn test_strikethrough_text() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": "deleted",
                    "marks": [{"type": "strike"}]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "~~deleted~~\n\n");
    }

    #[test]
    fn test_headings() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [
                {"type": "heading", "attrs": {"level": 1}, "content": [{"type": "text", "text": "H1"}]},
                {"type": "heading", "attrs": {"level": 2}, "content": [{"type": "text", "text": "H2"}]}
            ]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "# H1\n\n## H2\n\n");
    }

    #[test]
    fn test_bullet_list() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "bulletList",
                "content": [
                    {"type": "listItem", "content": [
                        {"type": "paragraph", "content": [{"type": "text", "text": "Item 1"}]}
                    ]},
                    {"type": "listItem", "content": [
                        {"type": "paragraph", "content": [{"type": "text", "text": "Item 2"}]}
                    ]}
                ]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("- Item 1"));
        assert!(result.contains("- Item 2"));
    }

    #[test]
    fn test_ordered_list() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "orderedList",
                "content": [
                    {"type": "listItem", "content": [
                        {"type": "paragraph", "content": [{"type": "text", "text": "First"}]}
                    ]},
                    {"type": "listItem", "content": [
                        {"type": "paragraph", "content": [{"type": "text", "text": "Second"}]}
                    ]}
                ]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("1. First"));
        assert!(result.contains("2. Second"));
    }

    #[test]
    fn test_code_block() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "codeBlock",
                "attrs": {"language": "rust"},
                "content": [{"type": "text", "text": "fn main() {}"}]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("```rust"));
        assert!(result.contains("fn main() {}"));
    }

    #[test]
    fn test_blockquote() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "blockquote",
                "content": [{
                    "type": "paragraph",
                    "content": [{"type": "text", "text": "quoted"}]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("> quoted"));
    }

    #[test]
    fn test_rule() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{"type": "rule"}]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "---\n\n");
    }

    #[test]
    fn test_mention() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "mention",
                    "attrs": {"id": "user123", "displayName": "John Doe"}
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "@John Doe\n\n");
    }

    #[test]
    fn test_emoji() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "emoji",
                    "attrs": {"shortName": "smile"}
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, ":smile:\n\n");
    }

    #[test]
    fn test_hard_break() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [
                    {"type": "text", "text": "line1"},
                    {"type": "hardBreak"},
                    {"type": "text", "text": "line2"}
                ]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "line1\nline2\n\n");
    }

    #[test]
    fn test_media_single() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "mediaSingle",
                "content": [{
                    "type": "media",
                    "attrs": {"url": "https://example.com/img.png", "alt": "example"}
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "![example](https://example.com/img.png)\n\n");
    }

    #[test]
    fn test_panel() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "panel",
                "attrs": {"panelType": "info"},
                "content": [{
                    "type": "paragraph",
                    "content": [{"type": "text", "text": "Note text"}]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("**info:**"));
        assert!(result.contains("Note text"));
    }

    #[test]
    fn test_task_list() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "taskList",
                "content": [
                    {
                        "type": "taskItem",
                        "attrs": {"state": "DONE"},
                        "content": [{
                            "type": "paragraph",
                            "content": [{"type": "text", "text": "done task"}]
                        }]
                    },
                    {
                        "type": "taskItem",
                        "attrs": {"state": "TODO"},
                        "content": [{
                            "type": "paragraph",
                            "content": [{"type": "text", "text": "todo task"}]
                        }]
                    }
                ]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("- [x] done task"));
        assert!(result.contains("- [ ] todo task"));
    }

    #[test]
    fn test_status() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "status",
                    "attrs": {"text": "In Progress"}
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "[STATUS: In Progress]\n\n");
    }

    #[test]
    fn test_date() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "date",
                    "attrs": {"timestamp": "1700000000000"}
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_expand() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "expand",
                "attrs": {"title": "More info"},
                "content": [{
                    "type": "paragraph",
                    "content": [{"type": "text", "text": "Hidden content"}]
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("<details><summary>More info</summary>"));
        assert!(result.contains("Hidden content"));
        assert!(result.contains("</details>"));
    }

    #[test]
    fn test_placeholder() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "placeholder",
                    "attrs": {"text": "Enter name"}
                }]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert_eq!(result, "[PLACEHOLDER: Enter name]\n\n");
    }

    #[test]
    fn test_unknown_node() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "someUnknownType",
                "attrs": {"key": "value"}
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("<pre><code>"));
    }

    #[test]
    fn test_nested_lists() {
        let input = serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "bulletList",
                "content": [
                    {
                        "type": "listItem",
                        "content": [
                            {"type": "paragraph", "content": [{"type": "text", "text": "Top level"}]},
                            {
                                "type": "bulletList",
                                "content": [{
                                    "type": "listItem",
                                    "content": [
                                        {"type": "paragraph", "content": [{"type": "text", "text": "Nested"}]}
                                    ]
                                }]
                            }
                        ]
                    }
                ]
            }]
        });
        let result = adf_to_markdown(&input).unwrap();
        assert!(result.contains("- Top level"));
        assert!(result.contains("Nested"));
    }
}
