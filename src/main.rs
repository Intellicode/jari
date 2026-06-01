use std::collections::HashMap;
use std::time::Instant;

use clap::{CommandFactory, Parser};
use serde::Serialize;

mod adf;
mod cli;
mod client;
mod config;
mod error;
mod logging;
mod models;
mod output;
mod schema;

use crate::adf::from_markdown::markdown_to_adf;
use crate::adf::to_markdown::adf_to_markdown;
use crate::cli::{
    Cli, Commands, CommentCmd, ConfigCmd, FieldCmd, IssueCmd, ProjectCmd, TransitionCmd, WatchCmd,
};
use crate::client::JiraClient;
use crate::config::Config;
use crate::error::JariError;
use crate::output::Output;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let cli = Cli::parse();
    logging::init_logging(cli.verbose);

    let command_string = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    // Commands that don't need config
    match &cli.command {
        Commands::Completions { shell } => {
            let shell = match shell.as_str() {
                "bash" => clap_complete::Shell::Bash,
                "zsh" => clap_complete::Shell::Zsh,
                "fish" => clap_complete::Shell::Fish,
                _ => clap_complete::Shell::Bash,
            };
            clap_complete::generate(shell, &mut Cli::command(), "jari", &mut std::io::stdout());
            return;
        }
        Commands::Config(ConfigCmd::Path) => {
            let path = Config::config_path(cli.config.as_deref());
            let duration = start.elapsed().as_millis() as u64;
            #[derive(Serialize)]
            struct P {
                path: String,
            }
            let p = P {
                path: path.to_string_lossy().to_string(),
            };
            let output = Output::success(&p, command_string, duration);
            output.print(cli.output.as_deref());
            return;
        }
        Commands::Config(ConfigCmd::Init) => {
            let duration = start.elapsed().as_millis() as u64;
            let result = run_config_init().await;
            match result {
                Ok(data) => {
                    let output = Output::success(&data, command_string, duration);
                    output.print(cli.output.as_deref());
                }
                Err(e) => {
                    let output = Output::<()>::error(&e, command_string, duration);
                    output.print(cli.output.as_deref());
                    std::process::exit(e.exit_code());
                }
            }
            return;
        }
        Commands::Schema { openai, anthropic } => {
            let duration = start.elapsed().as_millis() as u64;
            let result = schema::generate(*openai, *anthropic);
            let output = Output::success(&result, command_string, duration);
            output.print(cli.output.as_deref());
            return;
        }
        _ => {}
    }

    // Load config for all other commands
    let config = match Config::load(
        cli.url.as_deref(),
        cli.email.as_deref(),
        cli.token.as_deref(),
        cli.output.as_deref(),
        cli.project.as_deref(),
        cli.config.as_deref(),
    ) {
        Ok(c) => c,
        Err(e) => {
            let duration = start.elapsed().as_millis() as u64;
            let output = Output::<()>::error(&e, command_string, duration);
            output.print(cli.output.as_deref());
            std::process::exit(e.exit_code());
        }
    };

    // Config show needs config but runs special path
    if let Commands::Config(ConfigCmd::Show) = &cli.command {
        let duration = start.elapsed().as_millis() as u64;
        let data = config_show_data(&config, cli.output.as_deref());
        let output = Output::success(&data, command_string, duration);
        output.print(cli.output.as_deref());
        return;
    }

    let duration = start.elapsed().as_millis() as u64;
    let result = dispatch(&cli.command, &config).await;

    match result {
        Ok(value) => {
            let output_str = match cli.output.as_deref() {
                Some("json-pretty") => serde_json::to_string_pretty(&serde_json::json!({
                    "ok": true,
                    "data": value,
                    "meta": {"command": &command_string, "duration_ms": duration}
                }))
                .unwrap(),
                _ => serde_json::to_string(&serde_json::json!({
                    "ok": true,
                    "data": value,
                    "meta": {"command": &command_string, "duration_ms": duration}
                }))
                .unwrap(),
            };
            println!("{}", output_str);
        }
        Err(e) => {
            let output = Output::<()>::error(&e, command_string, duration);
            output.print(cli.output.as_deref());
            std::process::exit(e.exit_code());
        }
    }
}

async fn dispatch(cmd: &Commands, config: &Config) -> Result<serde_json::Value, JariError> {
    let client = JiraClient::new(config)?;

    match cmd {
        Commands::Issue(issue_cmd) => dispatch_issue(issue_cmd, &client).await,
        Commands::Search { jql, max, fields } => {
            let field_strings: Option<Vec<String>> = fields
                .as_ref()
                .map(|f| f.split(',').map(|s| s.trim().to_string()).collect());
            let field_ref: Option<&[String]> = field_strings.as_deref();
            let issues = client.search(jql, field_ref, *max).await?;
            Ok(serde_json::to_value(issues)?)
        }
        Commands::Transition(trans_cmd) => dispatch_transition(trans_cmd, &client).await,
        Commands::Comment(comment_cmd) => dispatch_comment(comment_cmd, &client).await,
        Commands::Project(proj_cmd) => dispatch_project(proj_cmd, &client).await,
        Commands::Field {
            cmd: FieldCmd::List,
        } => {
            let fields = client.list_fields().await?;
            Ok(serde_json::to_value(fields)?)
        }
        Commands::Me => {
            let user = client.get_current_user().await?;
            Ok(serde_json::to_value(user)?)
        }
        _ => Err(JariError::Cli("not yet implemented".into())),
    }
}

async fn dispatch_issue(
    cmd: &IssueCmd,
    client: &JiraClient,
) -> Result<serde_json::Value, JariError> {
    match cmd {
        IssueCmd::Get { key, fields, raw } => {
            let field_slice: Option<Vec<String>> = fields
                .as_ref()
                .map(|f| f.split(',').map(|s| s.trim().to_string()).collect());
            let field_ref: Option<&[String]> = field_slice.as_deref();
            let mut issue = client.get_issue(key, field_ref).await?;

            if !*raw {
                if let Some(ref adf) = issue.fields.description_raw {
                    match adf_to_markdown(adf) {
                        Ok(md) => {
                            issue.fields.description_markdown = Some(md);
                        }
                        Err(e) => {
                            issue.fields.description_markdown =
                                Some(format!("[ADF conversion error: {}]", e));
                        }
                    }
                }
            }

            Ok(serde_json::to_value(issue)?)
        }
        IssueCmd::Create {
            project,
            summary,
            issue_type,
            description,
            priority,
            assignee,
            labels,
            parent,
            epic_link,
        } => {
            let description_adf = if let Some(desc) = description {
                let md = if let Some(path) = desc.strip_prefix('@') {
                    std::fs::read_to_string(path).map_err(|e| {
                        JariError::Config(format!("Cannot read file '{}': {}", path, e))
                    })?
                } else {
                    desc.clone()
                };
                if md.trim().is_empty() {
                    None
                } else {
                    Some(markdown_to_adf(&md)?)
                }
            } else {
                None
            };

            let labels_vec: Option<Vec<String>> = labels.as_ref().map(|l| {
                l.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            });

            let request = crate::models::issue::CreateIssueRequest {
                project: project.clone(),
                summary: summary.clone(),
                issuetype: issue_type.clone(),
                description_adf,
                priority: priority.clone(),
                assignee: assignee.clone(),
                labels: labels_vec,
                parent: parent.clone(),
                epic_link: epic_link.clone(),
            };

            let created = client.create_issue(&request).await?;
            Ok(serde_json::to_value(created)?)
        }
        IssueCmd::Edit {
            key,
            summary,
            description,
            priority,
            labels,
            add_label,
        } => {
            let mut fields_map: HashMap<String, serde_json::Value> = HashMap::new();

            if let Some(ref s) = summary {
                fields_map.insert("summary".into(), serde_json::Value::String(s.clone()));
            }
            if let Some(ref desc) = description {
                let md = if let Some(path) = desc.strip_prefix('@') {
                    std::fs::read_to_string(path).map_err(|e| {
                        JariError::Config(format!("Cannot read file '{}': {}", path, e))
                    })?
                } else {
                    desc.clone()
                };
                let adf = markdown_to_adf(&md)?;
                fields_map.insert("description".into(), adf);
            }
            if let Some(ref p) = priority {
                fields_map.insert("priority".into(), serde_json::json!({"name": p}));
            }
            if let Some(ref l) = labels {
                let label_vec: Vec<String> = l
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                fields_map.insert("labels".into(), serde_json::to_value(label_vec)?);
            }
            if !add_label.is_empty() {
                fields_map.insert("labels".into(), serde_json::json!([{"add": add_label.iter().map(|s| s.trim().to_string()).collect::<Vec<_>>()}]));
            }

            if fields_map.is_empty() {
                return Err(JariError::Cli(
                    "At least one field to edit is required".into(),
                ));
            }

            client.edit_issue(key, &fields_map).await?;

            let updated_fields: Vec<String> = fields_map.keys().cloned().collect();
            Ok(serde_json::json!({
                "key": key,
                "updated_fields": updated_fields
            }))
        }
        IssueCmd::Delete { key, force } => {
            if !force {
                return Err(JariError::Cli(
                    format!("Deletion requires confirmation. Use --force to delete {} without confirmation.", key)
                ));
            }
            client.delete_issue(key).await?;
            Ok(serde_json::json!({"deleted": key}))
        }
        IssueCmd::Assign { key, user } => {
            let resolved_user = resolve_user(user, client).await?;
            client.assign_issue(key, &resolved_user).await?;
            Ok(serde_json::json!({"key": key, "assignee": user}))
        }
        IssueCmd::Watch(watch_cmd) => dispatch_watch(watch_cmd, client).await,
    }
}

async fn resolve_user(user: &str, client: &JiraClient) -> Result<String, JariError> {
    match user.to_lowercase().as_str() {
        "me" | "currentuser()" => {
            let current = client.get_current_user().await?;
            current
                .account_id
                .ok_or_else(|| JariError::Config("Current user has no account ID".into()))
        }
        "unassigned" | "none" => Ok(String::new()),
        _ => Ok(user.to_string()),
    }
}

async fn dispatch_watch(
    cmd: &WatchCmd,
    client: &JiraClient,
) -> Result<serde_json::Value, JariError> {
    let current_user = client.get_current_user().await?;
    let account_id = current_user
        .account_id
        .ok_or_else(|| JariError::Config("Current user has no account ID".into()))?;

    match cmd {
        WatchCmd::Add { key } => {
            match client.add_watcher(key, &account_id).await {
                Ok(()) => {}
                Err(JariError::Validation { .. }) => {}
                Err(e) => return Err(e),
            }
            Ok(serde_json::json!({"key": key, "watching": true, "account_id": account_id}))
        }
        WatchCmd::Remove { key } => {
            match client.remove_watcher(key, &account_id).await {
                Ok(()) => {}
                Err(JariError::Validation { .. }) => {}
                Err(e) => return Err(e),
            }
            Ok(serde_json::json!({"key": key, "watching": false, "account_id": account_id}))
        }
    }
}

async fn dispatch_transition(
    cmd: &TransitionCmd,
    client: &JiraClient,
) -> Result<serde_json::Value, JariError> {
    match cmd {
        TransitionCmd::List { key } => {
            let transitions = client.list_transitions(key).await?;
            Ok(serde_json::to_value(transitions)?)
        }
        TransitionCmd::Do {
            key,
            transition,
            comment,
            resolution,
        } => {
            let result = client
                .do_transition(key, transition, comment.as_deref(), resolution.as_deref())
                .await?;
            Ok(serde_json::to_value(result)?)
        }
    }
}

async fn dispatch_comment(
    cmd: &CommentCmd,
    client: &JiraClient,
) -> Result<serde_json::Value, JariError> {
    match cmd {
        CommentCmd::List { key, max } => {
            let comments = client.list_comments(key, *max).await?;
            let comments_md: Vec<serde_json::Value> = comments
                .iter()
                .map(|c| {
                    let mut val = serde_json::to_value(c).unwrap_or(serde_json::Value::Null);
                    if let Some(ref raw_body) = c.body_raw {
                        if let Ok(md) = adf_to_markdown(raw_body) {
                            val["body_markdown"] = serde_json::Value::String(md);
                        }
                    }
                    val
                })
                .collect();
            Ok(serde_json::Value::Array(comments_md))
        }
        CommentCmd::Get { key, id } => {
            let comment = client.get_comment(key, id).await?;
            let mut val = serde_json::to_value(&comment)?;
            if let Some(ref raw_body) = comment.body_raw {
                if let Ok(md) = adf_to_markdown(raw_body) {
                    val["body_markdown"] = serde_json::Value::String(md);
                }
            }
            Ok(val)
        }
        CommentCmd::Add {
            key,
            body,
            visibility,
        } => {
            if body.trim().is_empty() {
                return Err(JariError::Cli("Comment body is required".into()));
            }
            let result = client.add_comment(key, body, visibility.as_deref()).await?;
            Ok(serde_json::to_value(result)?)
        }
    }
}

async fn dispatch_project(
    cmd: &ProjectCmd,
    client: &JiraClient,
) -> Result<serde_json::Value, JariError> {
    match cmd {
        ProjectCmd::List { project_type } => {
            let projects = client.list_projects(project_type.as_deref()).await?;
            Ok(serde_json::to_value(projects)?)
        }
        ProjectCmd::Get { key } => {
            let project = client.get_project(key).await?;
            Ok(serde_json::to_value(project)?)
        }
    }
}

#[derive(Serialize)]
struct ConfigShowData {
    url: String,
    email: String,
    token: String,
    project: Option<String>,
    max_results: usize,
    output_format: String,
    timezone: String,
    source_url: String,
    source_email: String,
    source_token: String,
    source_project: String,
    source_output: String,
}

fn config_show_data(config: &Config, _output: Option<&str>) -> ConfigShowData {
    let masked_token = mask_token(&config.token);
    ConfigShowData {
        url: config.url.clone(),
        email: config.email.clone(),
        token: masked_token,
        project: config.project.clone(),
        max_results: config.max_results,
        output_format: config.output_format.clone(),
        timezone: config.timezone.clone(),
        source_url: format!("{:?}", config.source_url),
        source_email: format!("{:?}", config.source_email),
        source_token: format!("{:?}", config.source_token),
        source_project: format!("{:?}", config.source_project),
        source_output: format!("{:?}", config.source_output),
    }
}

fn mask_token(token: &str) -> String {
    if token.len() <= 8 {
        "***".to_string()
    } else {
        format!("{}...{}", &token[..4], &token[token.len() - 4..])
    }
}

#[derive(Serialize)]
struct ConfigInitData {
    path: String,
    user: Option<serde_json::Value>,
}

async fn run_config_init() -> Result<ConfigInitData, JariError> {
    use std::io::{IsTerminal, Write};

    let config_path = Config::config_path(None);

    if !std::io::stdin().is_terminal() {
        return Err(JariError::Cli(
            "config init requires an interactive terminal".into(),
        ));
    }

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    eprint!("Jira Cloud URL (e.g., https://company.atlassian.net): ");
    stdout.flush().ok();
    let mut url = String::new();
    stdin
        .read_line(&mut url)
        .map_err(|e| JariError::Config(format!("Failed to read input: {}", e)))?;
    url = url.trim().to_string();

    if url.is_empty() {
        return Err(JariError::Config("URL is required".into()));
    }

    eprint!("Email: ");
    stdout.flush().ok();
    let mut email = String::new();
    stdin
        .read_line(&mut email)
        .map_err(|e| JariError::Config(format!("Failed to read input: {}", e)))?;
    email = email.trim().to_string();

    if email.is_empty() || !email.contains('@') {
        return Err(JariError::Config("Valid email is required".into()));
    }

    eprintln!("API token (input hidden):");
    eprintln!("Generate at https://id.atlassian.com/manage-profile/security/api-tokens");
    let token = rpassword::prompt_password("Token: ")
        .map_err(|e| JariError::Config(format!("Failed to read token: {}", e)))?;

    if token.is_empty() {
        return Err(JariError::Config("API token is required".into()));
    }

    // Validate connection
    let temp_config = Config {
        url: url.clone(),
        email: email.clone(),
        token: token.clone(),
        project: None,
        max_results: 100,
        output_format: "json".to_string(),
        timezone: "local".to_string(),
        source_url: crate::config::ConfigSource::Cli,
        source_email: crate::config::ConfigSource::Cli,
        source_token: crate::config::ConfigSource::Cli,
        source_project: crate::config::ConfigSource::None,
        source_output: crate::config::ConfigSource::None,
    };

    let client = JiraClient::new(&temp_config)?;
    let user_info = match client.get_current_user().await {
        Ok(user) => {
            eprintln!(
                "Connected as: {} ({})",
                user.display_name,
                user.email_address.as_deref().unwrap_or("no email")
            );
            Some(serde_json::to_value(user).unwrap_or(serde_json::Value::Null))
        }
        Err(e) => {
            eprintln!("Connection failed: {}. Saving config anyway...", e);
            None
        }
    };

    // Write config
    let parent = config_path.parent().unwrap();
    std::fs::create_dir_all(parent)
        .map_err(|e| JariError::Config(format!("Failed to create config directory: {}", e)))?;

    if config_path.exists() {
        eprint!("Config already exists. Overwrite? (y/N): ");
        stdout.flush().ok();
        let mut answer = String::new();
        stdin
            .read_line(&mut answer)
            .map_err(|e| JariError::Config(format!("Failed to read input: {}", e)))?;
        if answer.trim().to_lowercase() != "y" && answer.trim().to_lowercase() != "yes" {
            return Err(JariError::Cli("Config init cancelled".into()));
        }
    }

    let toml_content = format!(
        "[connection]\nurl = \"{}\"\nemail = \"{}\"\ntoken = \"{}\"\n\n[defaults]\nmax_results = 100\n\n[output]\nformat = \"json\"\ntimezone = \"local\"\n",
        url, email, token
    );
    std::fs::write(&config_path, toml_content)
        .map_err(|e| JariError::Config(format!("Failed to write config: {}", e)))?;

    // Set file permissions to 600 on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&config_path, std::fs::Permissions::from_mode(0o600)).ok();
    }

    eprintln!("Config saved to {}", config_path.display());

    Ok(ConfigInitData {
        path: config_path.to_string_lossy().to_string(),
        user: user_info,
    })
}
