# Spec 010: CLI Definition — clap Derive

- [ ] Not implemented

## Goal

Define the full CLI command tree using `clap` derive macros, covering all subcommands, flags, and options described in the plan.

## Requirements

### Top-Level Struct

```rust
#[derive(Parser)]
#[command(name = "jari", about = "Jira Cloud CLI for LLM coding agents")]
pub struct Cli {
    #[arg(long, help = "Config file path")]
    pub config: Option<PathBuf>,

    #[arg(long, help = "Jira base URL")]
    pub url: Option<String>,

    #[arg(long, help = "Jira email")]
    pub email: Option<String>,

    #[arg(long, help = "API token")]
    pub token: Option<String>,

    #[arg(long, default_value = "json", help = "Output format: json, json-pretty, json-schema")]
    pub output: Option<String>,

    #[arg(short, long, help = "Enable verbose logging")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}
```

### Subcommand Enum

Must include all variants:
- `Issue` (subcommands: `Get`, `Create`, `Edit`, `Delete`, `Assign`, `Watch`)
- `Search` (search with JQL arg and flags)
- `Transition` (subcommands: `List`, `Do`)
- `Comment` (subcommands: `List`, `Get`, `Add`)
- `Project` (subcommands: `List`, `Get`)
- `Field { List }`
- `Me`
- `Config` (subcommands: `Show`, `Path`, `Init`)
- `Schema` (flags: `--openai`, `--anthropic`)
- `Completions` (arg: `Shell`)

### Requirements

- All arguments use `#[arg(help = "...")]` with human-readable descriptions
- Long and short flags where sensible (`-p` for `--project`, `-s` for `--summary`)
- `IssueCreate` supports `--description` accepting both inline text and `@file.md` syntax
- `Search` accepts positional JQL string
- `TransitionDo` accepts positional key and transition name/id
- `CommentAdd` accepts positional key and body text
- `Watch` is a subcommand group with `Add` and `Remove` variants
- Defined in `src/cli.rs`
