use crate::client::JiraClient;
use crate::error::JariError;
use crate::models::field::*;

impl JiraClient {
    pub async fn list_fields(&self) -> Result<Vec<Field>, JariError> {
        let page_size = self.max_page_size;
        let mut accumulated: Vec<Field> = Vec::new();
        let mut start_at: usize = 0;

        loop {
            let path = format!(
                "/field?startAt={}&maxResults={}",
                start_at, page_size
            );

            let response: FieldSearchResponse = self.get(&path).await?;

            let field_count = response.values.len();
            accumulated.extend(response.values);

            if response.is_last || field_count == 0 {
                break;
            }

            start_at += page_size;
        }

        accumulated.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        Ok(accumulated)
    }
}
