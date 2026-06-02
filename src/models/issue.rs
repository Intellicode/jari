use crate::models::common::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueFields {
    pub summary: String,
    #[serde(default, skip_deserializing)]
    pub description_markdown: Option<String>,
    #[serde(rename = "description", default)]
    pub description_raw: Option<serde_json::Value>,
    pub issuetype: IssueType,
    pub status: Status,
    pub priority: Option<Priority>,
    pub assignee: Option<User>,
    pub reporter: Option<User>,
    pub created: String,
    pub updated: String,
    pub duedate: Option<String>,
    pub resolution: Option<Resolution>,
    pub labels: Vec<String>,
    pub components: Vec<Component>,
    pub fix_versions: Vec<Version>,
    pub versions: Vec<Version>,
    pub parent: Option<IssueLink>,
    pub subtasks: Vec<IssueLink>,
    pub issuelinks: Vec<IssueLinkType>,
    pub timetracking: Option<TimeTracking>,
    pub votes: Option<Votes>,
    pub watches: Option<Watches>,
    pub worklog: Option<Worklog>,
    pub comment: Option<CommentsPage>,
    pub project: ProjectSummary,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssueRequest {
    pub project: String,
    pub summary: String,
    pub issuetype: String,
    pub description_adf: Option<serde_json::Value>,
    pub priority: Option<String>,
    pub assignee: Option<String>,
    pub labels: Option<Vec<String>>,
    pub parent: Option<String>,
    pub epic_link: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatedIssue {
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub url: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueSummary {
    pub id: String,
    pub key: String,
    pub fields: IssueSummaryFields,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueSummaryFields {
    pub summary: String,
    pub issuetype: IssueType,
    pub status: Status,
    #[serde(default)]
    pub priority: Option<Priority>,
    #[serde(default)]
    pub assignee: Option<User>,
    pub created: String,
    pub updated: String,
    #[serde(default)]
    pub duedate: Option<String>,
    #[serde(default)]
    pub labels: Vec<String>,
}
