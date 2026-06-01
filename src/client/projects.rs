use crate::client::JiraClient;
use crate::error::JariError;
use crate::models::project::*;

impl JiraClient {
    pub async fn list_projects(
        &self,
        type_filter: Option<&str>,
    ) -> Result<Vec<Project>, JariError> {
        let page_size = self.max_page_size;
        let mut accumulated: Vec<Project> = Vec::new();
        let mut start_at: usize = 0;

        loop {
            let path = match type_filter {
                Some(filter) => format!(
                    "/project/search?startAt={}&maxResults={}&type={}",
                    start_at, page_size, filter
                ),
                None => format!(
                    "/project/search?startAt={}&maxResults={}",
                    start_at, page_size
                ),
            };

            let response: ProjectSearchResponse = self.get(&path).await?;

            let project_count = response.values.len();
            accumulated.extend(response.values);

            if response.is_last || project_count == 0 {
                break;
            }

            start_at += page_size;
        }

        Ok(accumulated)
    }

    pub async fn get_project(&self, key: &str) -> Result<Project, JariError> {
        let path = format!("/project/{}", key);
        self.get(&path).await
    }
}
