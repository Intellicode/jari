use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::models::issue::IssueSummary;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub jql: String,
    #[serde(default)]
    pub start_at: usize,
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
            start_at: 0,
            max_results: 100,
            fields: default_fields(),
            fields_by_keys: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub start_at: usize,
    pub max_results: usize,
    pub total: usize,
    pub is_last: bool,
    pub issues: Vec<IssueSummary>,
}
