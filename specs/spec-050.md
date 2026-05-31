# Spec 050: CLI Polish — Help Text, Examples & Short Flags

- [ ] Not implemented

## Goal

Polish the CLI with consistent help text, usage examples, and convenient short flags.

## Requirements

### Help Text Standards

- Every subcommand has a one-line `about` description
- Every subcommand has a multi-line `long_about` with:
  - What the command does
  - When to use it
  - 1-2 usage examples
- Every argument has `help` text describing the value format and purpose
- Global `after_help` with link to docs/repo

### Example Help Output

```
jari issue get
  Get full details of a single Jira issue. Returns description in markdown,
  status, assignee, priority, and all standard fields.

  Usage: jari issue get <KEY> [--fields <FIELDS>]

  Arguments:
    <KEY>         The issue key (e.g., 'PROJ-123')
    --fields      Comma-separated additional fields to include

  Examples:
    jari issue get PROJ-123
    jari issue get PROJ-123 --fields customfield_10010,components
```

### Short Flags

| Flag | Short | Long |
|------|-------|------|
| `--verbose` | `-v` | `--verbose` |
| `--project` | `-p` | `--project` |
| `--summary` | `-s` | `--summary` |
| `--type` | `-t` | `--type` |
| `--description` | `-d` | `--description` |
| `--priority` | `-P` | `--priority` |
| `--assignee` | `-a` | `--assignee` |
| `--labels` | `-l` | `--labels` |
| `--fields` | `-f` | `--fields` |
| `--max` | `-n` | `--max` |
| `--force` | `-F` | `--force` |
| `--comment` | `-m` | `--comment` |

### Additional Polish

- `--help` output uses consistent indentation and spacing
- Color output via `clap` when stdout is a TTY
- Version info: `jari --version` prints version + commit hash + build date
- `jari --help` shows high-level command categories
- All error messages reference the relevant `--help` command
