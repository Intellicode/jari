# Jari &mdash; Jira Cloud CLI for LLM Coding Agents

[![Crates.io](https://img.shields.io/crates/v/jari)](https://crates.io/crates/jari)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**jari** is a command-line tool purpose-built for LLM coding agents (and humans) to interact with Jira Cloud. Every command emits parseable JSON to stdout (success or error) and logs to stderr. Descriptions and comments are auto-converted between Jira's ADF rich-text format and clean Markdown.

- **All reads**: descriptions/comments come back as Markdown.
- **All writes**: you supply Markdown, jari converts it to Jira's native format.
- **All results**: auto-paginated &mdash; the caller never worries about `startAt`/`maxResults`.
- **All errors**: stable JSON shape with `code`, `message`, `http_status`, and actionable `suggestion`.

---

## Quickstart (5&nbsp;minutes)

```bash
# 1. Install
brew install jari                                                   # macOS
# or: cargo install jari
# or: download a pre-built binary from GitHub Releases

# 2. Interactive configuration wizard
jari config init

# 3. Verify it works
jari me

# 4. Search your open tasks
jari search "assignee = currentUser() AND status != Done ORDER BY priority DESC"

# 5. Read an issue with Markdown description
jari issue get PROJ-123
```

---

## Installation

### Homebrew (macOS)

```bash
brew install jari
```

### Cargo (any platform with Rust)

```bash
cargo install jari
```

### Pre-built binaries

Download the latest binary for your platform from [GitHub Releases](https://github.com/anomalyco/jari/releases):

| Platform | Binary |
|----------|--------|
| macOS ARM64 (Apple Silicon) | `jari-aarch64-apple-darwin.tar.gz` |
| macOS x86_64 (Intel) | `jari-x86_64-apple-darwin.tar.gz` |
| Linux x86_64 (static musl) | `jari-x86_64-unknown-linux-musl.tar.gz` |
| Linux ARM64 (static musl) | `jari-aarch64-unknown-linux-musl.tar.gz` |

### Shell completions

```bash
# Bash
echo 'source <(jari completions bash)' >> ~/.bashrc

# Zsh
echo 'source <(jari completions zsh)' >> ~/.zshrc

# Fish
jari completions fish > ~/.config/fish/completions/jari.fish
```

Pre-built completion files are also available in the [completions/](completions/) directory.

---

## Authentication

### 1. Generate an API token

Go to [Atlassian API tokens](https://id.atlassian.com/manage-profile/security/api-tokens) and click **Create API token**. Copy the token &mdash; it is only shown once.

### 2. Configure jari

**Option A &mdash; Interactive wizard (recommended):**

```bash
jari config init
```

Prompts for URL, email, and token. Validates the connection by calling `jira.me`. Saves to the config file.

**Option B &mdash; Write the config file manually:**

```toml
# ~/.config/jari/config.toml
[connection]
url = "https://your-company.atlassian.net"
email = "you@company.com"
token = "ATATT3xFf..."

[defaults]
project = "PROJ"        # optional — default project
max_results = 100       # page size for list/search

[output]
format = "json"         # json | json-pretty
timezone = "local"      # local | utc
```

On macOS/Linux the file is `~/.config/jari/config.toml`.  
On Windows it is `%APPDATA%\jari\config.toml`.

**Option C &mdash; Environment variables (no config file needed):**

```bash
export JARI_URL="https://your-company.atlassian.net"
export JARI_EMAIL="you@company.com"
export JARI_TOKEN="ATATT3xFf..."
export JARI_PROJECT="PROJ"          # optional
export JARI_OUTPUT="json-pretty"    # optional
```

**Option D &mdash; CLI flags (temporary, not persisted):**

```bash
jari --url "https://..." --email "you@..." --token "ATATT..." me
```

### Resolution order

```
CLI flags  >  environment variables  >  config file
```

---

## Command Reference

All commands output JSON. Use `--output json-pretty` for human-readable formatting.

### Global Options

| Flag | Env var | Description |
|------|---------|-------------|
| `--config <PATH>` | &mdash; | Alternate config file path |
| `--url <URL>` | `JARI_URL` | Jira Cloud base URL |
| `--email <EMAIL>` | `JARI_EMAIL` | Your Atlassian account email |
| `--token <TOKEN>` | `JARI_TOKEN` | API token |
| `--project <KEY>` | `JARI_PROJECT` | Default project key |
| `--output <FORMAT>` | `JARI_OUTPUT` | `json` (default) or `json-pretty` |
| `-v, --verbose` | &mdash; | Enable debug-level logging to stderr |
| `-h, --help` | &mdash; | Show help |

---

### `jari issue get <KEY>`

Get full details of a single issue. Description is rendered as Markdown.

```
jari issue get PROJ-123
```

**Options:**

| Flag | Description |
|------|-------------|
| `-f, --fields <FIELDS>` | Comma-separated extra fields (e.g. `customfield_10001,duedate`) |
| `--raw` | Return raw ADF description instead of converting to Markdown |

**Example output:**

```json
{
  "ok": true,
  "data": {
    "id": "10042",
    "key": "PROJ-123",
    "fields": {
      "summary": "Fix login page timeout",
      "description_markdown": "## Problem\nThe login page times out after 30s...",
      "status": {"name": "In Progress", "status_category": {"key": "indeterminate"}},
      "priority": {"name": "High"},
      "assignee": {"display_name": "Jane Smith", "email_address": "jane@company.com"},
      "issuetype": {"name": "Bug", "subtask": false},
      "created": "2026-05-15T10:30:00.000+0000",
      "updated": "2026-05-28T14:22:00.000+0000"
    }
  },
  "meta": {"command": "jari issue get PROJ-123", "duration_ms": 234}
}
```

---

### `jari issue create`

Create a new issue. Description is supplied as Markdown and auto-converted to Jira's rich text.

```
jari issue create -p PROJ -s "Add dark mode support" -t Story \
  -d "## Problem\nUsers want dark mode.\n\n## AC\n- [ ] Toggle in settings" \
  -P High -l "frontend,ux"
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `-p, --project <KEY>` | Project key **(required)** | &mdash; |
| `-s, --summary <TEXT>` | Issue title/summary **(required)** | &mdash; |
| `-t, --type <TYPE>` | Issue type: `Story`, `Bug`, `Task`, `Subtask` | `Task` |
| `-d, --description <TEXT>` | Description in Markdown. Prefix with `@` to read from file: `@path/to/desc.md` | &mdash; |
| `-P, --priority <PRI>` | `Highest`, `High`, `Medium`, `Low`, `Lowest` | &mdash; |
| `-a, --assignee <USER>` | Assignee account ID or email | &mdash; |
| `-l, --labels <LABELS>` | Comma-separated labels | &mdash; |
| `--parent <KEY>` | Parent issue key (for subtasks) | &mdash; |
| `--epic-link <KEY>` | Epic issue key to link | &mdash; |

---

### `jari issue edit <KEY>`

Edit fields on an existing issue. All fields are optional; at least one must be supplied.

```
jari issue edit PROJ-123 -s "Updated summary" -P Low
```

**Options:**

| Flag | Description |
|------|-------------|
| `-s, --summary <TEXT>` | New summary |
| `-d, --description <TEXT>` | New description in Markdown (`@file.md` supported) |
| `-P, --priority <PRI>` | New priority |
| `-l, --labels <LABELS>` | Replace all labels (comma-separated) |
| `--add-label <LBL>` | Add a single label (repeatable) |

---

### `jari issue delete <KEY>`

Delete an issue. Requires `--force` confirmation.

```bash
jari issue delete PROJ-123 --force
```

| Flag | Description |
|------|-------------|
| `-F, --force` | Skip confirmation prompt |

---

### `jari issue assign <KEY> <USER>`

Assign an issue to a user.

```bash
jari issue assign PROJ-123 me           # assign to yourself
jari issue assign PROJ-123 jane@co.com  # assign by email
jari issue assign PROJ-123 unassigned   # remove assignment
```

---

### `jari issue watch add <KEY>` / `jari issue watch remove <KEY>`

Start or stop watching an issue.

```bash
jari issue watch add PROJ-123
jari issue watch remove PROJ-123
```

---

### `jari search <JQL>`

Search issues with JQL. **Auto-paginates** &mdash; you get all matching results in one response.

```bash
jari search "project = PROJ AND status = 'In Progress' ORDER BY priority DESC"
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --max <N>` | Limit results | All (capped at 1000) |
| `-f, --fields <FIELDS>` | Comma-separated fields | `summary,status,assignee,priority,issuetype` |

---

### `jari transition list <KEY>`

List available workflow transitions for an issue.

```bash
jari transition list PROJ-123
```

Each transition includes `id`, `name`, and destination status.

---

### `jari transition do <KEY> <TRANSITION>`

Execute a workflow transition. `<TRANSITION>` can be an ID or a name (partial match supported).

```bash
jari transition do PROJ-123 "In Progress"
jari transition do PROJ-123 "Done" -m "PR merged, deployed to staging"
```

**Options:**

| Flag | Description |
|------|-------------|
| `-m, --comment <TEXT>` | Comment to add during transition (Markdown) |
| `--resolution <NAME>` | Resolution to set (e.g. `Done`, `Won't Do`) |

---

### `jari comment list <KEY>`

List comments on an issue. Newest first. Body is converted to Markdown.

```bash
jari comment list PROJ-123
```

| Flag | Description | Default |
|------|-------------|---------|
| `-n, --max <N>` | Limit results | All |

---

### `jari comment get <KEY> <ID>`

Get a specific comment by ID.

```bash
jari comment get PROJ-123 10001
```

---

### `jari comment add <KEY> <BODY>`

Add a comment. Body is Markdown, auto-converted to Jira's rich text.

```bash
jari comment add PROJ-123 "## Progress update\n- Implemented auth flow\n- Added tests"
```

| Flag | Description |
|------|-------------|
| `--visibility <V>` | Comment visibility: `group:<name>` or `role:<id>` |

---

### `jari project list`

List all accessible projects.

```bash
jari project list
jari project list --type software
```

| Flag | Description |
|------|-------------|
| `--type <TYPE>` | Filter: `software`, `service_desk`, `business` |

---

### `jari project get <KEY>`

Get project details: name, description, lead, versions, components, and issue types.

```bash
jari project get PROJ
```

---

### `jari field list`

List all fields (system + custom) with IDs, names, and metadata.

```bash
jari field list
```

Use this to discover custom field IDs before creating or editing issues.

---

### `jari me`

Get the currently authenticated user's information.

```bash
jari me
```

Output includes `account_id`, `email_address`, `display_name`, and `active` status.

---

### `jari config show`

Print the current effective configuration. The API token is masked.

```bash
jari config show
```

---

### `jari config path`

Print the path to the config file jari uses.

```bash
jari config path
```

---

### `jari config init`

Interactive setup wizard. Prompts for URL, email, and token; validates the connection; writes the config file with restrictive permissions (0600 on Unix).

```bash
jari config init
```

---

### `jari schema`

Output JSON Schema of all jari commands for LLM tool-use discovery.

```bash
jari schema                     # Default verbose format with command templates
jari schema --openai            # OpenAI function-calling format
jari schema --anthropic         # Anthropic tool-use format
```

---

### `jari completions <SHELL>`

Generate shell completion scripts.

```bash
jari completions bash
jari completions zsh
jari completions fish
```

---

## JSON Output Format

Every jari command writes a single JSON object to stdout. The shape is consistent across all commands.

### Success

```json
{
  "ok": true,
  "data": { /* command-specific payload */ },
  "meta": {
    "command": "jari issue get PROJ-123",
    "duration_ms": 234
  }
}
```

### Error

```json
{
  "ok": false,
  "error": {
    "code": "not_found",
    "message": "Not found",
    "http_status": 404,
    "jira_errors": {
      "errorMessages": ["Issue does not exist or you do not have permission to view it."],
      "errors": {}
    },
    "suggestion": "The requested resource was not found. Verify the key/ID."
  },
  "meta": {
    "command": "jari issue get NOPE-999",
    "duration_ms": 312
  }
}
```

### Error codes

| `code` | HTTP | Meaning |
|--------|------|---------|
| `auth_failed` | 401 | Invalid credentials or expired token |
| `permission_denied` | 403 | Valid auth but insufficient permissions |
| `not_found` | 404 | Resource doesn't exist |
| `validation_error` | 400/422 | Invalid input (missing fields, bad values) |
| `rate_limited` | 429 | Too many requests (includes `retry_after` field) |
| `server_error` | 5xx | Jira-side error (retryable) |
| `network_error` | &mdash; | DNS, connection refused, timeout |
| `config_error` | &mdash; | Missing or invalid configuration |
| `adf_error` | &mdash; | Failed to parse or convert ADF content |
| `cli_error` | &mdash; | Invalid CLI arguments or usage |

**Exit codes:** `0` success, `1` error, `2` CLI usage error.

---

## LLM Agent Usage Guide

### Discovering jari's capabilities

LLMs should start by discovering the available tools:

```bash
# For OpenAI-compatible agents
jari schema --openai

# For Anthropic-compatible agents
jari schema --anthropic

# Verbose format (includes command templates)
jari schema
```

The schema output includes every command, its parameters, required fields, default values, and its CLI template. From this, the LLM can construct correct invocations.

### Design principles for LLM agents

1. **Branch on `ok`** &mdash; Every response has `"ok": true` or `"ok": false`. Check this first.
2. **Machine-readable errors** &mdash; `error.code` is a stable enum string. Handle known codes programmatically.
3. **Actionable suggestions** &mdash; `error.suggestion` contains human-readable fix guidance you can relay to users.
4. **Never paginate** &mdash; `jari search` and `jari comment list` auto-fetch all pages. You get the complete result set.
5. **Markdown everywhere** &mdash; Descriptions and comments arrive as Markdown. Write them as Markdown too.

### Common LLM workflows

#### Find and read my tasks

```bash
# Discover tools (do once or on first use)
jari schema --openai

# Find open tasks assigned to me
jari search "assignee = currentUser() AND status != Done ORDER BY priority DESC"

# Get full details of a specific task
jari issue get PROJ-123

# Read comments for context
jari comment list PROJ-123
```

#### Start working on a task

```bash
# See what transitions are available
jari transition list PROJ-123

# Move to In Progress
jari transition do PROJ-123 "In Progress"

# Assign to yourself
jari issue assign PROJ-123 me

# Add a comment about your approach
jari comment add PROJ-123 "Starting implementation. Using approach A with the new library."
```

#### Complete a task

```bash
# Add a completion comment with details
jari comment add PROJ-123 "## Done\n- Implemented feature X\n- Added tests\n- Updated docs\n\nPR: https://github.com/org/repo/pull/42"

# Move to Done
jari transition do PROJ-123 "Done"
```

#### Create a story

```bash
# First, check available project info and fields
jari project get PROJ
jari field list

# Create the story
jari issue create \
  -p PROJ \
  -t Story \
  -s "Add dark mode support to dashboard" \
  -d "## Problem
Users have requested dark mode for the analytics dashboard.

## Acceptance Criteria
- [ ] Toggle in settings to switch between light/dark/auto
- [ ] All existing components support dark theme
- [ ] Respects system preference by default
- [ ] No visual regressions in light mode

## Technical Notes
- Use CSS variables for theming
- Follow design tokens from the design system" \
  -P High \
  -l "frontend,ux,settings"

# Optionally assign to yourself
jari issue assign PROJ-456 me
```

#### Respond to a bug report

```bash
# Read the bug
jari issue get PROJ-789

# Ask clarifying questions via comment
jari comment add PROJ-789 "## Clarification\n- What browser version?\n- Any console errors?\n- Screenshot available?"
```

---

## JQL Cheatsheet for LLMs

JQL (Jira Query Language) is the query syntax used by `jari search`. Reference the [Atlassian JQL docs](https://support.atlassian.com/jira-software-cloud/docs/what-is-advanced-searching-in-jira/) for full details.

### Workload & assignment

| Pattern | JQL |
|---------|-----|
| My open tasks | `assignee = currentUser() AND status != Done` |
| My high-priority open tasks | `assignee = currentUser() AND priority = High AND resolution = Unresolved` |
| Unassigned issues | `assignee IS EMPTY AND resolution = Unresolved` |
| Someone else's bugs | `assignee = jane@co.com AND issuetype = Bug` |

### Status & progress

| Pattern | JQL |
|---------|-----|
| In progress across a project | `project = PROJ AND status = "In Progress"` |
| Blocked items | `status = Blocked OR labels = blocked` |
| Done this week | `status = Done AND resolutiondate >= -1w` |
| Stale (not updated in 7 days) | `updated <= -7d AND status != Done` |

### Time-based

| Pattern | JQL |
|---------|-----|
| Recently created | `created >= -7d` |
| Recently updated | `updated >= -24h` |
| Due this week | `duedate >= startOfWeek() AND duedate <= endOfWeek()` |
| Overdue | `duedate < now() AND resolution = Unresolved` |

### By issue metadata

| Pattern | JQL |
|---------|-----|
| By label | `labels in (frontend, bug)` |
| By priority | `priority = High` |
| By multiple priorities | `priority in (Highest, High)` |
| By type | `issuetype = Bug` |
| By type(s) | `issuetype in (Bug, Story)` |
| Subtasks of an issue | `parent = PROJ-123` |
| Epic children | `"Epic Link" = PROJ-100` |

### Full-text search

| Pattern | JQL |
|---------|-----|
| Contains a word | `text ~ "login"` |
| Contains a phrase | `text ~ "\"login page\""` |
| Summary contains | `summary ~ "timeout"` |
| Description contains | `description ~ "timeout"` |

### Ordering & limiting

| Pattern | JQL |
|---------|-----|
| By priority (highest first) | `ORDER BY priority DESC` |
| By creation (newest first) | `ORDER BY created DESC` |
| By last updated (most recent first) | `ORDER BY updated DESC` |
| Combined | `ORDER BY priority DESC, created DESC` |
| Limit to N results | Use `jari search -n 50 "..."` instead of MAXRESULTS in JQL |

### Common full queries

```bash
# My open bugs sorted by priority
jari search "assignee = currentUser() AND issuetype = Bug AND status != Done ORDER BY priority DESC"

# All in-progress work in my project
jari search "project = PROJ AND status = 'In Progress' ORDER BY updated ASC"

# High priority unresolved items across all projects
jari search "priority = High AND resolution = Unresolved ORDER BY created DESC"

# Issues I commented on recently
jari search "issue IN commentsAuthoredByUser(-7d)"

# Items I've reported
jari search "reporter = currentUser() ORDER BY created DESC"
```

---

## Troubleshooting

### Command not found: jari

Ensure jari is installed and on your `PATH`. Try:
- `brew install jari` and restart your terminal.
- `cargo install jari` &mdash; ensure `~/.cargo/bin` is in your `PATH`.
- Downloaded a binary? Make it executable: `chmod +x jari && mv jari /usr/local/bin/`.

### Authentication failed (401)

```json
{"ok":false,"error":{"code":"auth_failed","suggestion":"Check your email and API token..."}}
```

1. Verify your email matches your Atlassian account.
2. Regenerate your API token at https://id.atlassian.com/manage-profile/security/api-tokens
3. Confirm the URL is `https://your-company.atlassian.net` (not `http://`).
4. Test with `jari config show` to see the effective configuration.

### Permission denied (403)

Your account does not have the required permissions for the action. Contact your Jira administrator to:
- Grant **Browse Projects** permission for reading issues.
- Grant **Transition Issues**, **Create Issues**, etc. for write operations.

### Rate limited (429)

```json
{"ok":false,"error":{"code":"rate_limited","retry_after":15,"suggestion":"Rate limited. Wait before retrying."}}
```

Jari's HTTP client retries automatically up to 3 times with exponential backoff. If you still see this error, wait the number of seconds indicated by `retry_after` before retrying.

### Config errors

```json
{"ok":false,"error":{"code":"config_error","message":"Configuration error: URL must start with https://..."}}
```

- Use `jari config init` to reconfigure interactively.
- Check `jari config path` to confirm which file jari is reading.
- Ensure environment variables don't conflict with your config file. CLI flags take highest precedence.
- On Unix, the config file permissions should be `0600` (`chmod 600 ~/.config/jari/config.toml`).

### Markdown/ADF conversion issues

```json
{"ok":false,"error":{"code":"adf_error","message":"ADF conversion error: ..."}}
```

- Some ADF features (custom panels, macros, Jira-specific widgets) may not convert cleanly to Markdown.
- If a description is garbled, use `jari issue get PROJ-123 --raw` to see the raw ADF JSON.
- For writes, stick to CommonMark Markdown with GFM tables and task lists for best results.
- Markdown `@mentions` without a user ID are rendered as plain text.

### Verbose logging for debugging

Enable debug-level logging to stderr to see HTTP requests/responses:

```bash
jari -v issue get PROJ-123
# or
jari --verbose search "project = PROJ"
```

This outputs detailed request/response information to stderr (sanitized of auth headers) without affecting the JSON on stdout.

### Reporting bugs

File an issue at [github.com/anomalyco/jari/issues](https://github.com/anomalyco/jari/issues). Include:
- The command you ran (with sensitive values redacted).
- The JSON output (full error response).
- The output of `jari config show` (token is automatically masked).
- Verbose logs: `jari -v <your-command>` output from stderr.
- Your OS and jari version (`jari --version`).

---

## License

MIT &mdash; see [LICENSE](LICENSE).
