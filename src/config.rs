use crate::error::JariError;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSource {
    None,
    File,
    Env,
    Cli,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionFileConfig {
    pub url: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DefaultsFileConfig {
    pub project: Option<String>,
    pub max_results: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFileConfig {
    #[serde(default = "default_output_format")]
    pub format: String,
    #[serde(default = "default_timezone")]
    pub timezone: String,
}

fn default_output_format() -> String {
    "json".to_string()
}

fn default_timezone() -> String {
    "local".to_string()
}

impl Default for OutputFileConfig {
    fn default() -> Self {
        OutputFileConfig {
            format: default_output_format(),
            timezone: default_timezone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub connection: Option<ConnectionFileConfig>,
    #[serde(default)]
    pub defaults: DefaultsFileConfig,
    #[serde(default)]
    pub output: OutputFileConfig,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub url: String,
    pub email: String,
    pub token: String,
    pub project: Option<String>,
    pub max_results: usize,
    pub output_format: String,
    pub timezone: String,
    pub source_url: ConfigSource,
    pub source_email: ConfigSource,
    pub source_token: ConfigSource,
    pub source_project: ConfigSource,
    pub source_output: ConfigSource,
}

impl Config {
    pub fn load(
        cli_url: Option<&str>,
        cli_email: Option<&str>,
        cli_token: Option<&str>,
        cli_output: Option<&str>,
        cli_project: Option<&str>,
        config_path: Option<&std::path::Path>,
    ) -> Result<Self, JariError> {
        let path = config_path
            .map(PathBuf::from)
            .unwrap_or_else(default_config_path);

        let file_config: Option<FileConfig> = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            Some(toml::from_str(&content).map_err(|e| {
                JariError::Config(format!(
                    "Failed to parse config file '{}': {}",
                    path.display(),
                    e
                ))
            })?)
        } else {
            None
        };

        let (mut url, mut source_url) = (String::new(), ConfigSource::None);
        let (mut email, mut source_email) = (String::new(), ConfigSource::None);
        let (mut token, mut source_token) = (String::new(), ConfigSource::None);
        let (mut project, mut source_project) = (None, ConfigSource::None);
        let (mut output_format, mut source_output) = (String::from("json"), ConfigSource::None);
        let mut max_results: usize = 100;

        if let Some(ref fc) = file_config {
            if let Some(ref conn) = fc.connection {
                if url.is_empty() {
                    url = conn.url.clone();
                    source_url = ConfigSource::File;
                }
                if email.is_empty() {
                    email = conn.email.clone();
                    source_email = ConfigSource::File;
                }
                if token.is_empty() {
                    token = conn.token.clone();
                    source_token = ConfigSource::File;
                }
            }
            if project.is_none() {
                if let Some(ref p) = fc.defaults.project {
                    project = Some(p.clone());
                    source_project = ConfigSource::File;
                }
            }
            if let Some(mr) = fc.defaults.max_results {
                max_results = mr;
            }
            if matches!(source_output, ConfigSource::None) {
                output_format = fc.output.format.clone();
                source_output = ConfigSource::File;
            }
        }

        if url.is_empty() {
            if let Ok(env_url) = std::env::var("JARI_URL") {
                url = env_url;
                source_url = ConfigSource::Env;
            }
        }
        if email.is_empty() {
            if let Ok(env_email) = std::env::var("JARI_EMAIL") {
                email = env_email;
                source_email = ConfigSource::Env;
            }
        }
        if token.is_empty() {
            if let Ok(env_token) = std::env::var("JARI_TOKEN") {
                token = env_token;
                source_token = ConfigSource::Env;
            }
        }
        if project.is_none() {
            if let Ok(env_project) = std::env::var("JARI_PROJECT") {
                project = Some(env_project);
                source_project = ConfigSource::Env;
            }
        }
        if matches!(source_output, ConfigSource::None)
            || matches!(source_output, ConfigSource::File)
        {
            if let Ok(env_output) = std::env::var("JARI_OUTPUT") {
                output_format = env_output;
                source_output = ConfigSource::Env;
            }
        }

        if let Some(flag_url) = cli_url {
            url = flag_url.to_string();
            source_url = ConfigSource::Cli;
        }
        if let Some(flag_email) = cli_email {
            email = flag_email.to_string();
            source_email = ConfigSource::Cli;
        }
        if let Some(flag_token) = cli_token {
            token = flag_token.to_string();
            source_token = ConfigSource::Cli;
        }
        if let Some(flag_project) = cli_project {
            project = Some(flag_project.to_string());
            source_project = ConfigSource::Cli;
        }
        if let Some(flag_output) = cli_output {
            output_format = flag_output.to_string();
            source_output = ConfigSource::Cli;
        }

        Config::validate_url(&url, "URL")?;
        Config::validate_email(&email)?;
        Config::validate_token(&token)?;

        Ok(Config {
            url,
            email,
            token,
            project,
            max_results,
            output_format,
            timezone: "local".to_string(),
            source_url,
            source_email,
            source_token,
            source_project,
            source_output,
        })
    }

    fn validate_url(url: &str, label: &str) -> Result<(), JariError> {
        if url.is_empty() {
            return Err(JariError::Config(format!(
                "{} is required. Set it in your config file, JARI_URL env var, or --url flag.",
                label
            )));
        }

        let url_lower = url.to_lowercase();
        let is_atlassian = url_lower.contains(".atlassian.net") || url_lower.contains(".jira.com");

        if !url.starts_with("https://") {
            return Err(JariError::Config(format!(
                "{} must start with https://. Got: {}",
                label, url
            )));
        }

        if !is_atlassian {
            return Err(JariError::Config(format!(
                "{} must be a Jira Cloud URL matching https://*.atlassian.net or https://*.jira.com. Got: {}",
                label, url
            )));
        }

        Ok(())
    }

    fn validate_email(email: &str) -> Result<(), JariError> {
        if email.is_empty() {
            return Err(JariError::Config(
                "Email is required. Set it in your config file, JARI_EMAIL env var, or --email flag."
                    .to_string(),
            ));
        }
        if !email.contains('@') {
            return Err(JariError::Config(format!(
                "Email must contain '@'. Got: {}",
                email
            )));
        }
        Ok(())
    }

    fn validate_token(token: &str) -> Result<(), JariError> {
        if token.is_empty() {
            return Err(JariError::Config(
                "API token is required. Set it in your config file, JARI_TOKEN env var, or --token flag."
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn base_url(&self) -> &str {
        self.url.trim_end_matches('/')
    }

    pub fn config_path(path: Option<&std::path::Path>) -> PathBuf {
        path.map(PathBuf::from).unwrap_or_else(default_config_path)
    }
}

fn default_config_path() -> PathBuf {
    let base = BaseDirs::new().expect("Could not determine home directory");
    base.config_dir().join("jari").join("config.toml")
}
