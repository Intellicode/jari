use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCollection {
    #[serde(default)]
    pub error_messages: Vec<String>,
    #[serde(default)]
    pub errors: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum JariError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication failed")]
    Auth,

    #[error("Permission denied")]
    Permission,

    #[error("Not found")]
    NotFound,

    #[error("Validation error")]
    Validation {
        jira_errors: HashMap<String, String>,
    },

    #[error("Rate limited")]
    RateLimit { retry_after: Option<u64> },

    #[error("Server error")]
    ServerError,

    #[error("Network error: {0}")]
    Network(String),

    #[error("ADF conversion error: {0}")]
    AdfConversion(String),

    #[error("CLI error: {0}")]
    Cli(String),
}

impl JariError {
    pub fn code(&self) -> &'static str {
        match self {
            JariError::Config(_) => "config_error",
            JariError::Auth => "auth_failed",
            JariError::Permission => "permission_denied",
            JariError::NotFound => "not_found",
            JariError::Validation { .. } => "validation_error",
            JariError::RateLimit { .. } => "rate_limited",
            JariError::ServerError => "server_error",
            JariError::Network(_) => "network_error",
            JariError::AdfConversion(_) => "adf_error",
            JariError::Cli(_) => "cli_error",
        }
    }

    pub fn http_status(&self) -> Option<u16> {
        match self {
            JariError::Auth => Some(401),
            JariError::Permission => Some(403),
            JariError::NotFound => Some(404),
            JariError::Validation { .. } => Some(400),
            JariError::RateLimit { .. } => Some(429),
            JariError::ServerError => Some(500),
            _ => None,
        }
    }

    pub fn suggestion(&self) -> &'static str {
        match self {
            JariError::Config(_) => "Check your configuration. Ensure URL, email, and token are set correctly.",
            JariError::Auth => "Check your email and API token. Generate a token at https://id.atlassian.com/manage-profile/security/api-tokens",
            JariError::Permission => "You don't have permission for this action. Contact your Jira admin.",
            JariError::NotFound => "The requested resource was not found. Verify the key/ID.",
            JariError::Validation { .. } => "Check the required fields and values. Use 'jari field list' to see valid fields.",
            JariError::RateLimit { .. } => "Rate limited. Wait before retrying.",
            JariError::ServerError => "Jira server error. This is usually temporary. Retry in a moment.",
            JariError::Network(_) => "Network error. Check your connection and the Jira URL.",
            JariError::AdfConversion(_) => "Failed to convert rich text. The content may contain unsupported formatting.",
            JariError::Cli(_) => "Check the command syntax. Use --help for usage.",
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            JariError::Cli(_) => 2,
            _ => 1,
        }
    }

    pub fn jira_errors(&self) -> Option<HashMap<String, String>> {
        match self {
            JariError::Validation { jira_errors, .. } => Some(jira_errors.clone()),
            _ => None,
        }
    }

    pub fn retry_after(&self) -> Option<u64> {
        match self {
            JariError::RateLimit { retry_after } => *retry_after,
            _ => None,
        }
    }
}

impl From<reqwest::Error> for JariError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            JariError::Network("Request timed out".to_string())
        } else if err.is_connect() {
            JariError::Network(format!("Connection failed: {}", err))
        } else {
            JariError::Network(err.to_string())
        }
    }
}

impl From<serde_json::Error> for JariError {
    fn from(err: serde_json::Error) -> Self {
        JariError::Network(format!("JSON parse error: {}", err))
    }
}

impl From<std::io::Error> for JariError {
    fn from(err: std::io::Error) -> Self {
        JariError::Config(format!("IO error: {}", err))
    }
}
