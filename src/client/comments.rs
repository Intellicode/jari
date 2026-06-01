use crate::client::JiraClient;
use crate::error::JariError;
use crate::models::comment::*;

impl JiraClient {
    pub async fn list_comments(
        &self,
        key: &str,
        max: Option<usize>,
    ) -> Result<Vec<Comment>, JariError> {
        let page_size = self.max_page_size;
        let mut accumulated: Vec<Comment> = Vec::new();
        let mut start_at: usize = 0;

        loop {
            let path = format!(
                "/issue/{}/comment?startAt={}&maxResults={}",
                key, start_at, page_size
            );

            let response: CommentsPage = self.get(&path).await?;

            let comment_count = response.values.len();
            accumulated.extend(response.values);

            if let Some(max_val) = max {
                if accumulated.len() >= max_val {
                    accumulated.truncate(max_val);
                    break;
                }
            }

            if response.is_last || comment_count == 0 {
                break;
            }

            start_at += page_size;
        }

        Ok(accumulated)
    }

    pub async fn get_comment(
        &self,
        key: &str,
        comment_id: &str,
    ) -> Result<Comment, JariError> {
        let path = format!("/issue/{}/comment/{}", key, comment_id);
        self.get(&path).await
    }

    pub async fn add_comment(
        &self,
        key: &str,
        body_md: &str,
        visibility: Option<&str>,
    ) -> Result<CreatedComment, JariError> {
        let adf = crate::adf::from_markdown::markdown_to_adf(body_md)?;

        let visibility_obj = visibility.and_then(|v| {
            let parts: Vec<&str> = v.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some(CommentVisibility {
                    visibility_type: parts[0].to_string(),
                    value: parts[1].to_string(),
                })
            } else {
                None
            }
        });

        let request = AddCommentRequest {
            body: adf,
            visibility: visibility_obj,
        };

        let path = format!("/issue/{}/comment", key);
        self.post(&path, &request).await
    }
}
