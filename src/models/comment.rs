use crate::models::common::User;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub author: Option<User>,
    #[serde(default)]
    pub body_markdown: Option<String>,
    #[serde(rename = "body", default)]
    pub body_raw: Option<serde_json::Value>,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommentsPage {
    pub start_at: usize,
    pub max_results: usize,
    pub total: usize,
    pub is_last: bool,
    pub values: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatedComment {
    pub id: String,
    pub created: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentRequest {
    pub body: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CommentVisibility>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentVisibility {
    #[serde(rename = "type")]
    pub visibility_type: String,
    pub value: String,
}
