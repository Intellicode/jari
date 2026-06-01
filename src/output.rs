use crate::error::ErrorCollection;
use crate::error::JariError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OutputMeta {
    pub command: String,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct OutputError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira_errors: Option<ErrorCollection>,
    pub suggestion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct Output<T: Serialize> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<OutputError>,
    pub meta: OutputMeta,
}

impl<T: Serialize> Output<T> {
    pub fn success(data: T, command: String, duration_ms: u64) -> Self {
        Output {
            ok: true,
            data: Some(data),
            error: None,
            meta: OutputMeta {
                command,
                duration_ms,
                total_results: None,
            },
        }
    }

    pub fn print(&self, output_format: Option<&str>) {
        match output_format {
            Some("json-pretty") => {
                println!("{}", serde_json::to_string_pretty(self).unwrap());
            }
            _ => {
                println!("{}", serde_json::to_string(self).unwrap());
            }
        }
    }

    pub fn error(err: &JariError, command: String, duration_ms: u64) -> Self {
        let output_error = OutputError {
            code: err.code().to_string(),
            message: err.to_string(),
            http_status: err.http_status(),
            jira_errors: err.jira_errors().map(|errors| ErrorCollection {
                error_messages: vec![],
                errors,
            }),
            suggestion: err.suggestion().to_string(),
            retry_after: err.retry_after(),
        };

        Output {
            ok: false,
            data: None,
            error: Some(output_error),
            meta: OutputMeta {
                command,
                duration_ms,
                total_results: None,
            },
        }
    }
}
