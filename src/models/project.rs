use crate::models::common::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub lead: Option<User>,
    #[serde(default)]
    pub project_type_key: Option<String>,
    #[serde(default)]
    pub simplified: bool,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub versions: Vec<Version>,
    #[serde(default)]
    pub components: Vec<Component>,
    #[serde(default)]
    pub issue_types: Option<Vec<IssueType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSearchResponse {
    pub start_at: usize,
    pub max_results: usize,
    pub total: usize,
    pub is_last: bool,
    pub values: Vec<Project>,
}
