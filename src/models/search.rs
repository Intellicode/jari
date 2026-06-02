use crate::models::issue::IssueSummary;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub jql: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(default = "default_max_results")]
    pub max_results: usize,
    #[serde(default = "default_fields")]
    pub fields: Vec<String>,
    #[serde(default)]
    pub fields_by_keys: bool,
}

fn default_max_results() -> usize {
    100
}

fn default_fields() -> Vec<String> {
    vec![
        "summary".into(),
        "status".into(),
        "assignee".into(),
        "priority".into(),
        "issuetype".into(),
    ]
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            jql: String::new(),
            next_page_token: None,
            max_results: 100,
            fields: default_fields(),
            fields_by_keys: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    #[serde(default)]
    pub next_page_token: Option<String>,
    pub is_last: bool,
    pub issues: Vec<IssueSummary>,
}
