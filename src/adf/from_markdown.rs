use crate::error::JariError;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use serde_json::{Map, Value};

pub fn markdown_to_adf(md: &str) -> Result<Value, JariError> {
    if md.is_empty() {
        return Ok(serde_json::json!({
            "type": "doc",
            "version": 1,
            "content": []
        }));
    }

    let mut options = Options::all();
    options.remove(Options::ENABLE_SMART_PUNCTUATION);
    options.remove(Options::ENABLE_FOOTNOTES);
    options.remove(Options::ENABLE_OLD_FOOTNOTES);
    let parser = Parser::new_ext(md, options);

    let mut state = ADFState::new();
    state.push_context("doc", Map::new());

    for event in parser {
        state.process_event(event);
    }

    // Close remaining open contexts
    while state.node_types.len() > 1 {
        state.pop_and_attach();
    }

    let root_content = state.content_stacks.last().cloned().unwrap_or_default();

    Ok(serde_json::json!({
        "type": "doc",
        "version": 1,
        "content": root_content
    }))
}

struct ADFState {
    /// Stack of content arrays (one per nesting level)
    content_stacks: Vec<Vec<Value>>,
    /// Stack of node types
    node_types: Vec<String>,
    /// Stack of node attrs
    attrs_stacks: Vec<Map<String, Value>>,
    /// Active inline marks (pushed/popped by emphasis/strong/link/strike tags)
    active_marks: Vec<Value>,
    /// Track if we're inside a table header section
    in_table_header: bool,
    /// Whether the current list contains task items
    current_list_is_tasklist: bool,
    /// Pending task checked state (from TaskListMarker event)
    pending_task_checked: Option<bool>,
    /// Inside a paragraph currently being built
    in_paragraph: bool,
    /// Skip events until image end tag (for inline images)
    in_image: bool,
}

impl ADFState {
    fn new() -> Self {
        Self {
            content_stacks: vec![],
            node_types: vec![],
            attrs_stacks: vec![],
            active_marks: vec![],
            in_table_header: false,
            current_list_is_tasklist: false,
            pending_task_checked: None,
            in_paragraph: false,
            in_image: false,
        }
    }

    fn push_context(&mut self, node_type: &str, attrs: Map<String, Value>) {
        self.node_types.push(node_type.to_string());
        self.attrs_stacks.push(attrs);
        self.content_stacks.push(Vec::new());
    }

    fn pop_and_attach(&mut self) -> Option<Value> {
        let node_type = self.node_types.pop()?;
        let attrs = self.attrs_stacks.pop()?;
        let content = self.content_stacks.pop()?;

        let mut node = serde_json::json!({
            "type": node_type,
        });

        if !content.is_empty() {
            node["content"] = Value::Array(content);
        }
        if !attrs.is_empty() {
            node["attrs"] = Value::Object(attrs);
        }

        // Attach to parent content
        if let Some(parent_content) = self.content_stacks.last_mut() {
            parent_content.push(node.clone());
        }

        Some(node)
    }

    fn add_text(&mut self, text: &str) {
        let marks: Vec<Value> = self.active_marks.clone();
        let text_node = if marks.is_empty() {
            serde_json::json!({
                "type": "text",
                "text": text
            })
        } else {
            serde_json::json!({
                "type": "text",
                "text": text,
                "marks": marks
            })
        };
        self.add_to_current(text_node);
    }

    fn add_code(&mut self, code: &str) {
        let mut marks: Vec<Value> = self.active_marks.clone();
        marks.push(serde_json::json!({"type": "code"}));
        let code_node = serde_json::json!({
            "type": "text",
            "text": code,
            "marks": marks
        });
        self.add_to_current(code_node);
    }

    fn add_to_current(&mut self, node: Value) {
        if let Some(content) = self.content_stacks.last_mut() {
            content.push(node);
        }
    }

    fn add_hard_break(&mut self) {
        self.add_to_current(serde_json::json!({"type": "hardBreak"}));
    }

    fn add_rule(&mut self) {
        self.add_to_current(serde_json::json!({"type": "rule"}));
    }

    fn push_mark(&mut self, mark: Value) {
        self.active_marks.push(mark);
    }

    fn pop_mark(&mut self) {
        self.active_marks.pop();
    }

    fn process_event(&mut self, event: Event) {
        // Skip events while we're inside an already-processed image
        if self.in_image {
            if let Event::End(TagEnd::Image) = event {
                self.in_image = false;
            }
            return;
        }

        match event {
            Event::Start(tag) => self.handle_start(tag),
            Event::End(tag_end) => self.handle_end(tag_end),
            Event::Text(text) => self.add_text(text.as_ref()),
            Event::Code(code) => self.add_code(code.as_ref()),
            Event::SoftBreak => self.add_hard_break(),
            Event::HardBreak => self.add_hard_break(),
            Event::Rule => self.add_rule(),
            Event::TaskListMarker(checked) => {
                self.pending_task_checked = Some(checked);
                // The parent list becomes a taskList
                self.current_list_is_tasklist = true;
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                self.handle_html(html.as_ref());
            }
            Event::InlineMath(math) => {
                self.add_text(math.as_ref());
            }
            Event::DisplayMath(math) => {
                let code_node = serde_json::json!({
                    "type": "codeBlock",
                    "attrs": {"language": "math"},
                    "content": [{"type": "text", "text": math.as_ref()}]
                });
                self.add_to_current(code_node);
            }
            Event::FootnoteReference(_) => {}
        }
    }

    fn handle_start(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => {
                self.in_paragraph = true;
                self.push_context("paragraph", Map::new());
            }

            Tag::Heading {
                level,
                id: _,
                classes: _,
                attrs: _,
            } => {
                let level_num: u64 = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                let mut attrs = Map::new();
                attrs.insert("level".into(), Value::Number(level_num.into()));
                self.push_context("heading", attrs);
            }

            Tag::List(start_num) => {
                self.current_list_is_tasklist = false;
                self.pending_task_checked = None;
                let list_type = match start_num {
                    Some(_) => "orderedList",
                    None => "bulletList",
                };
                let mut attrs = Map::new();
                if let Some(n) = start_num {
                    attrs.insert("order".into(), Value::Number(n.into()));
                }
                self.push_context(list_type, attrs);
            }

            Tag::Item => {
                self.push_context("listItem", Map::new());
            }

            Tag::CodeBlock(code_block_kind) => {
                let lang = match &code_block_kind {
                    CodeBlockKind::Fenced(s) => s.as_ref(),
                    CodeBlockKind::Indented => "",
                };
                let mut attrs = Map::new();
                if !lang.is_empty() {
                    attrs.insert("language".into(), Value::String(lang.to_string()));
                }
                self.push_context("codeBlock", attrs);
            }

            Tag::BlockQuote(_kind) => {
                self.push_context("blockquote", Map::new());
            }

            Tag::Table(alignments) => {
                let _align_strs: Vec<Value> = alignments
                    .iter()
                    .map(|a| {
                        Value::String(
                            match a {
                                pulldown_cmark::Alignment::None => "none",
                                pulldown_cmark::Alignment::Left => "left",
                                pulldown_cmark::Alignment::Center => "center",
                                pulldown_cmark::Alignment::Right => "right",
                            }
                            .to_string(),
                        )
                    })
                    .collect();
                let attrs = Map::new();
                self.push_context("table", attrs);
            }

            Tag::TableHead => {
                self.in_table_header = true;
                // pulldown-cmark emits cells directly under TableHead without a TableRow
                self.push_context("tableRow", Map::new());
            }

            Tag::TableRow => {
                // Only push if not already in a header row (handled by TableHead)
                if !self.in_table_header {
                    self.push_context("tableRow", Map::new());
                }
            }

            Tag::TableCell => {
                let node_type = if self.in_table_header {
                    "tableHeader"
                } else {
                    "tableCell"
                };
                self.push_context(node_type, Map::new());
            }

            Tag::Emphasis => {
                self.push_mark(serde_json::json!({"type": "em"}));
            }

            Tag::Strong => {
                self.push_mark(serde_json::json!({"type": "strong"}));
            }

            Tag::Strikethrough => {
                self.push_mark(serde_json::json!({"type": "strike"}));
            }

            Tag::Link {
                dest_url,
                title: _,
                id: _,
                link_type: _,
            } => {
                self.push_mark(serde_json::json!({
                    "type": "link",
                    "attrs": {"href": dest_url.as_ref()}
                }));
            }

            Tag::Image {
                dest_url,
                title,
                id: _,
                link_type: _,
            } => {
                // Images are block-level mediaSingle > media
                // Close paragraph first if we're inside one, but don't keep empty paragraphs
                if self.in_paragraph {
                    self.node_types.pop();
                    self.attrs_stacks.pop();
                    let _ = self.content_stacks.pop();
                    self.in_paragraph = false;
                }

                let alt = title.as_ref();
                let mut media_attrs = Map::new();
                media_attrs.insert("url".into(), Value::String(dest_url.to_string()));
                media_attrs.insert("type".into(), Value::String("file".to_string()));
                media_attrs.insert(
                    "collection".into(),
                    Value::String("contentId-123".to_string()),
                );
                if !alt.is_empty() {
                    media_attrs.insert("alt".into(), Value::String(alt.to_string()));
                }

                let media = serde_json::json!({
                    "type": "media",
                    "attrs": media_attrs,
                });

                // Create a mediaSingle context
                let ms_attrs = Map::new();
                self.push_context("mediaSingle", ms_attrs);
                self.add_to_current(media);
                self.pop_and_attach();
                // Skip remaining events until End(Image)
                self.in_image = true;
            }

            Tag::HtmlBlock => {
                // Track as a raw HTML context
                self.push_context("_html_block", Map::new());
            }

            Tag::FootnoteDefinition(_) => {
                self.push_context("_footnote", Map::new());
            }

            Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition => {
                self.push_context("paragraph", Map::new());
            }

            Tag::MetadataBlock(_) => {
                // Ignore metadata blocks
            }
        }
    }

    fn handle_end(&mut self, tag_end: TagEnd) {
        match tag_end {
            TagEnd::Paragraph => {
                // Only close if we're still inside a paragraph
                // (might have been closed early by an inline image)
                if self.in_paragraph || self.node_types.last().map(|t| t.as_str()) == Some("paragraph") {
                    self.pop_and_attach();
                    self.in_paragraph = false;
                }
            }

            TagEnd::Heading(_level) => {
                self.pop_and_attach();
            }

            TagEnd::List(_is_ordered) => {
                // Before popping, check if this list should be a taskList
                if self.current_list_is_tasklist {
                    // Change the node type of the list in-place
                    if let Some(node_type) = self.node_types.last_mut() {
                        *node_type = "taskList".to_string();
                    }
                }
                self.pop_and_attach();
                self.current_list_is_tasklist = false;
                self.pending_task_checked = None;
            }

            TagEnd::Item => {
                let checked = self.pending_task_checked.take();

                // If this item belongs to a task list, convert it to taskItem
                if self.current_list_is_tasklist {
                    if let Some(node_type) = self.node_types.last_mut() {
                        *node_type = "taskItem".to_string();
                    }
                    if let Some(checked) = checked {
                        if let Some(attrs) = self.attrs_stacks.last_mut() {
                            let state = if checked { "DONE" } else { "TODO" };
                            attrs.insert("state".into(), Value::String(state.to_string()));
                        }
                    }
                }

                self.pop_and_attach();
            }

            TagEnd::CodeBlock => {
                self.pop_and_attach();
            }

            TagEnd::BlockQuote(_kind) => {
                self.pop_and_attach();
            }

            TagEnd::Table => {
                self.pop_and_attach();
            }

            TagEnd::TableHead => {
                // Pop the header row that was pushed in Tag::TableHead
                self.pop_and_attach();
                self.in_table_header = false;
            }

            TagEnd::TableRow => {
                // Check if this row is a separator row (all cells are --- :--- etc.)
                // If so, discard it instead of attaching to the table
                let is_separator = self.content_stacks
                    .last()
                    .map(|stack| {
                        stack.iter().all(|cell| {
                            cell.get("content")
                                .and_then(|c| c.as_array())
                                .and_then(|arr| arr.first())
                                .and_then(|n| n.get("text"))
                                .and_then(|t| t.as_str())
                                .map(|text| {
                                    text.chars().all(|c| c == '-' || c == ':' || c == ' ' || c == '|')
                                })
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false);

                if !self.in_table_header && is_separator {
                    // Discard separator row
                    self.node_types.pop();
                    self.attrs_stacks.pop();
                    self.content_stacks.pop();
                } else {
                    self.pop_and_attach();
                }
            }

            TagEnd::TableCell => {
                self.pop_and_attach();
            }

            TagEnd::Emphasis => {
                self.pop_mark();
            }

            TagEnd::Strong => {
                self.pop_mark();
            }

            TagEnd::Strikethrough => {
                self.pop_mark();
            }

            TagEnd::Link => {
                self.pop_mark();
            }

            TagEnd::Image => {
                // Nothing to pop - we already closed mediaSingle on start
            }

            TagEnd::HtmlBlock => {
                let content = self.content_stacks.pop().unwrap_or_default();
                let _node_type = self.node_types.pop();
                let _attrs = self.attrs_stacks.pop();

                // Try to parse as <details>
                let mut html_str = String::new();
                for node in &content {
                    if let Some(text) = node.get("text").and_then(|t| t.as_str()) {
                        html_str.push_str(text);
                    }
                }

                if let Some(expand_node) = try_parse_details(&html_str) {
                    self.add_to_current(expand_node);
                } else {
                    // Add content back as text
                    for node in content {
                        self.add_to_current(node);
                    }
                }
            }

            TagEnd::FootnoteDefinition => {
                let _content = self.content_stacks.pop().unwrap_or_default();
                let _node_type = self.node_types.pop();
                let _attrs = self.attrs_stacks.pop();
            }

            TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition => {
                self.pop_and_attach();
            }

            TagEnd::MetadataBlock(_) => {}
        }
    }

    fn handle_html(&mut self, html: &str) {
        let trimmed = html.trim();

        if let Some(expand_node) = try_parse_details(trimmed) {
            self.add_to_current(expand_node);
            return;
        }

        // Strip other HTML, or pass through as text
        // Simple HTML stripping: remove tags
        let stripped = strip_html(trimmed);
        if !stripped.is_empty() {
            self.add_text(&stripped);
        }
    }
}

fn try_parse_details(html: &str) -> Option<Value> {
    let html = html.trim();

    if html == "</details>" || html == "</DETAILS>" {
        return None; // end tag handled separately
    }

    if let Some(rest) = html
        .strip_prefix("<details")
        .or_else(|| html.strip_prefix("<DETAILS"))
    {
        let rest = rest.trim_start();

        // Try to find <summary> inside
        let summary_end = rest.find(">")?;
        let inner = &rest[summary_end + 1..];

        if inner.is_empty() && !html.ends_with("</details>") {
            return None;
        }

        let title = if let Some(sum_start) = inner.find("<summary>") {
            let after_sum = &inner[sum_start + 9..];
            if let Some(sum_end) = after_sum.find("</summary>") {
                after_sum[..sum_end].to_string()
            } else {
                String::new()
            }
        } else if let Some(sum_start) = inner.find("<SUMMARY>") {
            let after_sum = &inner[sum_start + 9..];
            if let Some(sum_end) = after_sum.find("</SUMMARY>") {
                after_sum[..sum_end].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Find content between </summary> and </details>
        let content_start = if let Some(sum_close) = inner.find("</summary>") {
            sum_close + 10
        } else if let Some(sum_close) = inner.find("</SUMMARY>") {
            sum_close + 10
        } else {
            summary_end + 1
        };

        let content_end = if let Some(end) = inner.find("</details>") {
            end
        } else if let Some(end) = inner.find("</DETAILS>") {
            end
        } else {
            inner.len()
        };

        let body = inner[content_start..content_end].trim();

        let mut attrs = Map::new();
        attrs.insert("title".into(), Value::String(title));

        let body_content = if body.is_empty() {
            vec![]
        } else {
            // Nest content in a paragraph
            let text_node = serde_json::json!({
                "type": "text",
                "text": body
            });
            let para = serde_json::json!({
                "type": "paragraph",
                "content": [text_node]
            });
            vec![para]
        };

        Some(serde_json::json!({
            "type": "expand",
            "attrs": attrs,
            "content": body_content
        }))
    } else {
        None
    }
}

fn strip_html(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_document() {
        let result = markdown_to_adf("").unwrap();
        assert_eq!(result["type"], "doc");
        assert_eq!(result["version"], 1);
        assert!(result["content"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_plain_paragraph() {
        let result = markdown_to_adf("Hello world").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content.len(), 1);
        assert_eq!(content[0]["type"], "paragraph");
        let para_content = content[0]["content"].as_array().unwrap();
        assert_eq!(para_content.len(), 1);
        assert_eq!(para_content[0]["text"], "Hello world");
    }

    #[test]
    fn test_bold() {
        let result = markdown_to_adf("**bold**").unwrap();
        let para = &result["content"][0]["content"].as_array().unwrap();
        assert_eq!(para[0]["text"], "bold");
        assert_eq!(para[0]["marks"][0]["type"], "strong");
    }

    #[test]
    fn test_italic() {
        let result = markdown_to_adf("*italic*").unwrap();
        let para = &result["content"][0]["content"].as_array().unwrap();
        assert_eq!(para[0]["text"], "italic");
        assert_eq!(para[0]["marks"][0]["type"], "em");
    }

    #[test]
    fn test_bold_italic() {
        let result = markdown_to_adf("***both***").unwrap();
        let para = &result["content"][0]["content"].as_array().unwrap();
        assert_eq!(para[0]["text"], "both");
        assert_eq!(para.len(), 1);
        let marks: Vec<&str> = para[0]["marks"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m["type"].as_str().unwrap())
            .collect();
        assert!(marks.contains(&"strong"));
        assert!(marks.contains(&"em"));
    }

    #[test]
    fn test_code() {
        let result = markdown_to_adf("`code`").unwrap();
        let para = &result["content"][0]["content"].as_array().unwrap();
        assert_eq!(para[0]["text"], "code");
        assert_eq!(para[0]["marks"][0]["type"], "code");
    }

    #[test]
    fn test_link() {
        let result = markdown_to_adf("[link](https://example.com)").unwrap();
        let para = &result["content"][0]["content"].as_array().unwrap();
        assert_eq!(para[0]["text"], "link");
        assert_eq!(para[0]["marks"][0]["type"], "link");
        assert_eq!(para[0]["marks"][0]["attrs"]["href"], "https://example.com");
    }

    #[test]
    fn test_strikethrough() {
        let result = markdown_to_adf("~~strike~~").unwrap();
        let para = &result["content"][0]["content"].as_array().unwrap();
        assert_eq!(para[0]["text"], "strike");
        assert_eq!(para[0]["marks"][0]["type"], "strike");
    }

    #[test]
    fn test_headings() {
        let result = markdown_to_adf("# H1\n## H2\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "heading");
        assert_eq!(content[0]["attrs"]["level"], 1);
        assert_eq!(content[1]["type"], "heading");
        assert_eq!(content[1]["attrs"]["level"], 2);
    }

    #[test]
    fn test_unordered_list() {
        let result = markdown_to_adf("- Item 1\n- Item 2\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "bulletList");
        let items = content[0]["content"].as_array().unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_ordered_list() {
        let result = markdown_to_adf("1. First\n2. Second\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "orderedList");
    }

    #[test]
    fn test_code_block() {
        let result = markdown_to_adf("```rust\nfn main() {}\n```\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "codeBlock");
        assert_eq!(content[0]["attrs"]["language"], "rust");
    }

    #[test]
    fn test_blockquote() {
        let result = markdown_to_adf("> quoted\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "blockquote");
    }

    #[test]
    fn test_horizontal_rule() {
        let result = markdown_to_adf("---\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "rule");
    }

    #[test]
    fn test_image() {
        let result = markdown_to_adf("![alt](https://example.com/img.png)\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "mediaSingle");
        let media = &content[0]["content"][0];
        assert_eq!(media["type"], "media");
        assert_eq!(media["attrs"]["url"], "https://example.com/img.png");
    }

    #[test]
    fn test_task_list() {
        let result = markdown_to_adf("- [x] done\n- [ ] todo\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "taskList");
        let items = content[0]["content"].as_array().unwrap();
        assert_eq!(items[0]["type"], "taskItem");
        assert_eq!(items[0]["attrs"]["state"], "DONE");
        assert_eq!(items[1]["type"], "taskItem");
        assert_eq!(items[1]["attrs"]["state"], "TODO");
    }

    #[test]
    fn test_details_html() {
        let result = markdown_to_adf("<details><summary>Title</summary>Content</details>").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "expand");
        assert_eq!(content[0]["attrs"]["title"], "Title");
    }

    #[test]
    fn test_multiple_paragraphs() {
        let result = markdown_to_adf("Para 1\n\nPara 2\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content.len(), 2);
        assert_eq!(content[0]["type"], "paragraph");
        assert_eq!(content[1]["type"], "paragraph");
    }

    #[test]
    fn test_nested_lists() {
        let result = markdown_to_adf("- Top\n  - Nested\n").unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "bulletList");
    }

    #[test]
    fn test_gtf_table() {
        let input = "| A | B |\n| --- | --- |\n| 1 | 2 |\n";
        let result = markdown_to_adf(input).unwrap();
        let content = result["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "table");
        let rows = content[0]["content"].as_array().unwrap();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_hard_break() {
        let result = markdown_to_adf("line1  \nline2\n").unwrap();
        let para = &result["content"][0];
        let nodes = para["content"].as_array().unwrap();
        assert_eq!(nodes[0]["text"], "line1");
        assert_eq!(nodes[1]["type"], "hardBreak");
        assert_eq!(nodes[2]["text"], "line2");
    }

    #[test]
    fn test_at_mention_as_text() {
        let result = markdown_to_adf("@username says hello").unwrap();
        let para = &result["content"][0]["content"].as_array().unwrap();
        // @mention is treated as plain text in v1
        let first_text = para
            .iter()
            .find(|n| n["text"].as_str().unwrap_or("").contains("@username"));
        assert!(first_text.is_some());
    }

    #[test]
    fn test_roundtrip_bold() {
        let md = "**bold**";
        let adf = markdown_to_adf(md).unwrap();
        let roundtrip = super::super::to_markdown::adf_to_markdown(&adf).unwrap();
        assert_eq!(roundtrip.trim(), "**bold**");
    }

    #[test]
    fn test_roundtrip_italic() {
        let md = "*italic*";
        let adf = markdown_to_adf(md).unwrap();
        let roundtrip = super::super::to_markdown::adf_to_markdown(&adf).unwrap();
        assert_eq!(roundtrip.trim(), "*italic*");
    }

    #[test]
    fn test_roundtrip_strikethrough() {
        let md = "~~deleted~~";
        let adf = markdown_to_adf(md).unwrap();
        let roundtrip = super::super::to_markdown::adf_to_markdown(&adf).unwrap();
        assert_eq!(roundtrip.trim(), "~~deleted~~");
    }

    #[test]
    fn test_roundtrip_code() {
        let md = "`fn main()`";
        let adf = markdown_to_adf(md).unwrap();
        let roundtrip = super::super::to_markdown::adf_to_markdown(&adf).unwrap();
        assert_eq!(roundtrip.trim(), "`fn main()`");
    }

    #[test]
    fn test_roundtrip_link() {
        let md = "[click](https://example.com)";
        let adf = markdown_to_adf(md).unwrap();
        let roundtrip = super::super::to_markdown::adf_to_markdown(&adf).unwrap();
        assert_eq!(roundtrip.trim(), "[click](https://example.com)");
    }
}
