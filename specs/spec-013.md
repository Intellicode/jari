# Spec 013: ADF Type Definitions

- [ ] Not implemented

## Goal

Define Rust types that represent the Atlassian Document Format (ADF) node tree for serialization/deserialization.

## Requirements

### Core Types

```rust
// Top-level document
pub struct Doc {
    pub version: u32,           // Always 1
    #[serde(rename = "type")]
    pub node_type: String,      // Always "doc"
    pub content: Vec<Node>,
}

// Generic ADF node
pub struct Node {
    #[serde(rename = "type")]
    pub node_type: String,      // "paragraph", "heading", "text", etc.
    pub content: Option<Vec<Node>>,
    pub text: Option<String>,
    pub marks: Option<Vec<Mark>>,
    pub attrs: Option<serde_json::Value>,
}

// Inline marks (bold, italic, link, etc.)
pub struct Mark {
    #[serde(rename = "type")]
    pub mark_type: String,      // "strong", "em", "link", "code", etc.
    pub attrs: Option<serde_json::Value>,
}
```

### Requirements

- `Doc` validates `version == 1` and `node_type == "doc"` on deserialization
- All fields use `#[serde(default)]` where possible for forward compatibility
- Unknown node types should not fail deserialization (use untagged enum or `Value` fallback)
- Types defined in `src/adf/mod.rs`
- Derive `Serialize`, `Deserialize`, `Debug`, `Clone`
