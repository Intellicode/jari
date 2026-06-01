use crate::client::JiraClient;
use crate::error::JariError;
use crate::models::common::User;

impl JiraClient {
    pub async fn get_current_user(&self) -> Result<User, JariError> {
        self.get("/myself").await
    }
}
