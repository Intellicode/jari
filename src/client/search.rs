use crate::client::JiraClient;
use crate::error::JariError;
use crate::models::issue::IssueSummary;
use crate::models::search::*;

const MAX_SEARCH_RESULTS: usize = 1000;

impl JiraClient {
    pub async fn search(
        &self,
        jql: &str,
        fields: Option<&[String]>,
        max_override: Option<usize>,
    ) -> Result<Vec<IssueSummary>, JariError> {
        if jql.trim().is_empty() {
            return Err(JariError::Cli("JQL query must not be empty".into()));
        }

        let page_size = self.max_page_size;
        let max_results = max_override
            .unwrap_or(MAX_SEARCH_RESULTS)
            .min(MAX_SEARCH_RESULTS);

        let default_fields = vec![
            "summary".into(),
            "status".into(),
            "assignee".into(),
            "priority".into(),
            "issuetype".into(),
            "created".into(),
            "updated".into(),
        ];
        let field_list: Vec<String> = fields.map(|f| f.to_vec()).unwrap_or(default_fields);

        let mut accumulated: Vec<IssueSummary> = Vec::new();
        let mut next_page_token: Option<String> = None;

        loop {
            let request = SearchRequest {
                jql: jql.to_string(),
                next_page_token: next_page_token.clone(),
                max_results: page_size,
                fields: field_list.clone(),
                fields_by_keys: false,
            };

            let response: SearchResults = self.post("/search/jql", &request).await?;

            let issue_count = response.issues.len();
            accumulated.extend(response.issues);

            if accumulated.len() >= max_results {
                accumulated.truncate(max_results);
                break;
            }

            if response.is_last || issue_count == 0 {
                break;
            }

            next_page_token = response.next_page_token;
        }

        Ok(accumulated)
    }
}
