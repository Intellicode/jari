use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "jari", about = "Jira Cloud CLI for LLM coding agents", version)]
pub struct Cli {
    #[arg(long, help = "Config file path [default: ~/.config/jari/config.toml]")]
    pub config: Option<PathBuf>,

    #[arg(long, help = "Jira base URL (overrides config/env)")]
    pub url: Option<String>,

    #[arg(long, help = "Jira email (overrides config/env)")]
    pub email: Option<String>,

    #[arg(long, help = "API token (overrides config/env)")]
    pub token: Option<String>,

    #[arg(
        long,
        default_value = "json",
        help = "Output format: json, json-pretty"
    )]
    pub output: Option<String>,

    #[arg(long, help = "Default project key (overrides config/env)")]
    pub project: Option<String>,

    #[arg(short, long, help = "Enable verbose logging (DEBUG level)")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Issue(IssueCmd),

    Search {
        #[arg(help = "JQL query")]
        jql: String,

        #[arg(short = 'n', long, help = "Maximum results (default: all)")]
        max: Option<usize>,

        #[arg(
            short = 'f',
            long,
            default_value = "summary,status,assignee,priority,issuetype,created,updated",
            help = "Comma-separated fields"
        )]
        fields: Option<String>,
    },

    #[command(subcommand)]
    Transition(TransitionCmd),

    #[command(subcommand)]
    Comment(CommentCmd),

    #[command(subcommand)]
    Project(ProjectCmd),

    Field {
        #[command(subcommand)]
        cmd: FieldCmd,
    },

    Me,

    #[command(subcommand)]
    Config(ConfigCmd),

    Schema {
        #[arg(long, help = "Output in OpenAI function-calling format")]
        openai: bool,

        #[arg(long, help = "Output in Anthropic tool-use format")]
        anthropic: bool,
    },

    Completions {
        #[arg(help = "Shell: bash, zsh, or fish")]
        shell: String,
    },
}

#[derive(Subcommand)]
pub enum IssueCmd {
    Get {
        #[arg(help = "Issue key (e.g., PROJ-123)")]
        key: String,

        #[arg(short = 'f', long, help = "Comma-separated additional fields")]
        fields: Option<String>,

        #[arg(long, help = "Return raw ADF description instead of markdown")]
        raw: bool,
    },

    Create {
        #[arg(short = 'p', long, help = "Project key (required)")]
        project: String,

        #[arg(short = 's', long, help = "Issue summary/title (required)")]
        summary: String,

        #[arg(
            short = 't',
            long,
            default_value = "Task",
            help = "Issue type: Story, Bug, Task, etc."
        )]
        issue_type: String,

        #[arg(
            short = 'd',
            long,
            help = "Description in markdown (use @file.md for file input)"
        )]
        description: Option<String>,

        #[arg(
            short = 'P',
            long,
            help = "Priority: Highest, High, Medium, Low, Lowest"
        )]
        priority: Option<String>,

        #[arg(short = 'a', long, help = "Assignee account ID or email")]
        assignee: Option<String>,

        #[arg(short = 'l', long, help = "Comma-separated labels")]
        labels: Option<String>,

        #[arg(long, help = "Parent issue key (for subtasks)")]
        parent: Option<String>,

        #[arg(long, help = "Epic link key")]
        epic_link: Option<String>,
    },

    Edit {
        #[arg(help = "Issue key")]
        key: String,

        #[arg(short = 's', long, help = "New summary")]
        summary: Option<String>,

        #[arg(short = 'd', long, help = "New description in markdown")]
        description: Option<String>,

        #[arg(short = 'P', long, help = "New priority")]
        priority: Option<String>,

        #[arg(short = 'l', long, help = "New labels (replaces existing)")]
        labels: Option<String>,

        #[arg(long = "add-label", help = "Add a label (can repeat)")]
        add_label: Vec<String>,
    },

    Delete {
        #[arg(help = "Issue key")]
        key: String,

        #[arg(short = 'F', long, help = "Skip confirmation")]
        force: bool,
    },

    Assign {
        #[arg(help = "Issue key")]
        key: String,

        #[arg(help = "Account ID, email, 'me', or 'unassigned'")]
        user: String,
    },

    #[command(subcommand)]
    Watch(WatchCmd),
}

#[derive(Subcommand)]
pub enum WatchCmd {
    Add {
        #[arg(help = "Issue key")]
        key: String,
    },
    Remove {
        #[arg(help = "Issue key")]
        key: String,
    },
}

#[derive(Subcommand)]
pub enum TransitionCmd {
    List {
        #[arg(help = "Issue key")]
        key: String,
    },
    Do {
        #[arg(help = "Issue key")]
        key: String,

        #[arg(help = "Transition name or ID")]
        transition: String,

        #[arg(short = 'm', long, help = "Comment to add during transition")]
        comment: Option<String>,

        #[arg(long, help = "Resolution name (e.g., Done)")]
        resolution: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum CommentCmd {
    List {
        #[arg(help = "Issue key")]
        key: String,

        #[arg(short = 'n', long, help = "Maximum comments (default: all)")]
        max: Option<usize>,
    },
    Get {
        #[arg(help = "Issue key")]
        key: String,

        #[arg(help = "Comment ID")]
        id: String,
    },
    Add {
        #[arg(help = "Issue key")]
        key: String,

        #[arg(help = "Comment text in markdown")]
        body: String,

        #[arg(long, help = "Visibility: group:<name> or role:<id>")]
        visibility: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ProjectCmd {
    List {
        #[arg(long, help = "Filter: software, service_desk, business")]
        project_type: Option<String>,
    },
    Get {
        #[arg(help = "Project key")]
        key: String,
    },
}

#[derive(Subcommand)]
pub enum FieldCmd {
    List,
}

#[derive(Subcommand)]
pub enum ConfigCmd {
    Show,
    Path,
    Init,
}
