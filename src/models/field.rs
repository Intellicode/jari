use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub id: String,
    #[serde(default)]
    pub key: Option<String>,
    pub name: String,
    #[serde(default)]
    pub custom: bool,
    #[serde(default)]
    pub orderable: bool,
    #[serde(default)]
    pub navigable: bool,
    #[serde(default)]
    pub searchable: bool,
    #[serde(default)]
    pub schema: Option<FieldSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldSchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(default)]
    pub items: Option<String>,
    #[serde(default)]
    pub system: Option<String>,
    #[serde(default)]
    pub custom: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldSearchResponse {
    pub start_at: usize,
    pub max_results: usize,
    pub total: usize,
    pub is_last: bool,
    pub values: Vec<Field>,
}
