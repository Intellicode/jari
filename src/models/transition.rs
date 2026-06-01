use crate::models::common::StatusCategory;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transition {
    pub id: String,
    pub name: String,
    pub to: TransitionDestination,
    #[serde(default)]
    pub has_screen: Option<bool>,
    #[serde(default)]
    pub is_conditional: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransitionDestination {
    pub name: String,
    pub id: String,
    #[serde(default)]
    pub status_category: Option<StatusCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransitionResult {
    pub transition: String,
    pub from_status: String,
    pub to_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionsResponse {
    pub transitions: Vec<Transition>,
}
