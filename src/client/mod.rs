use crate::config::Config;
use crate::error::{ErrorCollection, JariError};
use base64::Engine;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;
use tracing::trace;

pub mod comments;
pub mod fields;
pub mod issues;
pub mod projects;
pub mod search;
pub mod transitions;
pub mod users;

pub struct JiraClient {
    pub base_url: String,
    http: Client,
    auth_header: String,
    pub max_page_size: usize,
}

impl JiraClient {
    pub fn new(config: &Config) -> Result<Self, JariError> {
        let encoded = base64::engine::general_purpose::STANDARD
            .encode(format!("{}:{}", config.email, config.token));

        let auth_header = format!("Basic {}", encoded);

        let client = Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .read_timeout(Duration::from_secs(60))
            .user_agent(format!("jari/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| JariError::Network(e.to_string()))?;

        Ok(Self {
            base_url: config.base_url().to_string(),
            http: client,
            auth_header,
            max_page_size: config.max_results,
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, JariError> {
        let url = self.api_url(path);
        trace!("GET {}", url);

        let response = self
            .http
            .get(&url)
            .header(AUTHORIZATION, &self.auth_header)
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, JariError> {
        let url = self.api_url(path);
        trace!("POST {}", url);

        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, &self.auth_header)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn put_no_body<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), JariError> {
        let url = self.api_url(path);
        trace!("PUT {}", url);

        let response = self
            .http
            .put(&url)
            .header(AUTHORIZATION, &self.auth_header)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_response_no_body(response).await
    }

    pub async fn post_no_body<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), JariError> {
        let url = self.api_url(path);
        trace!("POST {}", url);

        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, &self.auth_header)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_response_no_body(response).await
    }

    pub async fn delete(&self, path: &str) -> Result<(), JariError> {
        let url = self.api_url(path);
        trace!("DELETE {}", url);

        let response = self
            .http
            .delete(&url)
            .header(AUTHORIZATION, &self.auth_header)
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        self.handle_response_no_body(response).await
    }

    fn api_url(&self, path: &str) -> String {
        let path = path.trim_start_matches('/');
        format!("{}/rest/api/3/{}", self.base_url, path)
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, JariError> {
        let status = response.status();

        if status.is_success() {
            let body = response.json::<T>().await?;
            Ok(body)
        } else {
            Err(self.parse_error_response(status, response).await)
        }
    }

    async fn handle_response_no_body(&self, response: Response) -> Result<(), JariError> {
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            Err(self.parse_error_response_no_body(status, response).await)
        }
    }

    async fn parse_error_response(&self, status: StatusCode, response: Response) -> JariError {
        let retry_after = parse_retry_after(response.headers());
        let body_text = response.text().await.unwrap_or_default();

        let jira_errors = serde_json::from_str::<ErrorCollection>(&body_text)
            .unwrap_or(ErrorCollection {
                error_messages: vec![body_text.clone()],
                errors: std::collections::HashMap::new(),
            });

        map_status_to_error(status, Some(jira_errors), retry_after)
    }

    async fn parse_error_response_no_body(
        &self,
        status: StatusCode,
        response: Response,
    ) -> JariError {
        let retry_after = parse_retry_after(response.headers());
        let body_text = response.text().await.unwrap_or_default();

        let jira_errors = serde_json::from_str::<ErrorCollection>(&body_text)
            .unwrap_or(ErrorCollection {
                error_messages: vec![body_text],
                errors: std::collections::HashMap::new(),
            });

        map_status_to_error(status, Some(jira_errors), retry_after)
    }

}

fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<u64> {
    headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
}

fn map_status_to_error(
    status: StatusCode,
    jira_errors: Option<ErrorCollection>,
    retry_after: Option<u64>,
) -> JariError {
    match status.as_u16() {
        401 => JariError::Auth,
        403 => JariError::Permission,
        404 => JariError::NotFound,
        429 => JariError::RateLimit { retry_after },
        code if (400..=499).contains(&code) => {
            let errors = jira_errors
                .map(|ec| ec.errors)
                .unwrap_or_default();
            JariError::Validation {
                jira_errors: errors,
            }
        }
        _ => JariError::ServerError,
    }
}


