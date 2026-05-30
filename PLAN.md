# Jari — Jira CLI for LLM Coding Agents

## Summary of Requirements

| Axis | Decision |
|------|----------|
| **Target** | Jira Cloud only (`*.atlassian.net` / `api.atlassian.com`) |
| **Auth** | API Token (Basic Auth) + env vars + config file |
| **Output** | JSON (machine-parseable) |
| **ADF** | Auto-convert to/from Markdown |
| **Language** | Rust |
| **MCP** | No — CLI only (shell-invocable) |
| **Schema** | Generate JSON Schema for LLM tool-use discovery |
| **Config** | TOML file (`~/.config/jari/config.toml`) + env var overrides |
| **Distribute** | Homebrew + `cargo install` + pre-built binaries (GitHub Releases) |

## Primary Workflows (Priority Order)

1. **Retrieve & read tasks** — Search issues, get issue details, list assigned work
2. **Execute work & update** — Transition issues through workflow, add comments, assign users
3. **Create issues** — Create new issues (stories, bugs, subtasks) with markdown descriptions

## Non-Goals (v1)

- MCP server protocol support
- Jira Data Center / Server
- OAuth 2.0 (3LO) authentication flow
- TUI / interactive mode
- Webhook listeners / real-time sync
- Admin/configuration operations (manage users, workflows, custom fields)
- Agile boards / sprints / velocity reports

---

## 1. Tech Stack

| Layer | Crate | Rationale |
|-------|-------|-----------|
| **CLI parsing** | `clap` v4 (derive) | Industry standard. Derive macros keep code minimal. Built-in shell completions via `clap_complete`. |
| **Async runtime** | `tokio` (full features) | Needed for `reqwest` HTTP. Single-threaded runtime is fine for a CLI. |
| **HTTP client** | `reqwest` (rustls-tls) | Mature, supports HTTP/2, connection pooling, retries. No OpenSSL dependency for easy cross-compilation. |
| **Serialization** | `serde` + `serde_json` | Defines all API models. Single source of truth for HTTP deserialization and CLI output. |
| **JSON Schema** | `schemars` | Derives JSON Schema from the same `serde` structs — zero duplication. |
| **Error handling** | `thiserror` + `miette` | `thiserror` for library error types. `miette` for pretty CLI diagnostics + JSON error serialization for LLMs. |
| **Config** | `toml` + `directories` | `directories` finds `~/.config/jari/` cross-platform. `toml` parses config. |
| **Logging** | `tracing` + `tracing-subscriber` | Structured, level-controlled. JSON log output when needed. |
| **Markdown parse** | `pulldown-cmark` | For Markdown → ADF conversion. Well-maintained, accurate CommonMark parser. |
| **Testing** | `rstest` + `wiremock` + `assert-json-diff` | `wiremock` for HTTP mocking. `assert-json-diff` for snapshot-style JSON tests. |
| **Build/release** | `cargo-dist` + GitHub Actions | Automated cross-compiled binary builds + Homebrew formula generation. |

### Why NOT alternatives

- **Not `native-tls`**: Avoids OpenSSL linking pains on cross-compilation (Linux builds, macOS ARM).
- **Not existing Jira SDK**: No well-maintained Rust Jira SDK exists. Building our own client around `reqwest` + `serde` gives full control over the exact API surface we need for LLM workflows.
- **Not workspace multi-crate**: Single crate is simpler. Multi-crate adds build complexity without benefit for a project of this scale.

---

## 2. Project Structure

```
jari/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── PLAN.md
├── .github/
│   └── workflows/
│       ├── ci.yml                  # Test, lint, format on PR
│       └── release.yml             # Build + publish binaries, Homebrew formula
├── src/
│   ├── main.rs                     # Entry point, tokio main
│   ├── cli.rs                      # Entire CLI tree via clap derive
│   ├── output.rs                   # JSON envelope formatting
│   ├── error.rs                    # Error types, JSON error serialization
│   ├── config.rs                   # Config loading (file + env vars)
│   ├── client/
│   │   ├── mod.rs                  # JiraClient struct, auth, request helpers
│   │   ├── issues.rs               # Issue CRUD API methods
│   │   ├── search.rs               # JQL search with auto-pagination
│   │   ├── transitions.rs          # Get/execute transitions
│   │   ├── comments.rs             # Comment CRUD
│   │   ├── projects.rs             # Project listing
│   │   ├── users.rs                # User lookup, current user
│   │   └── fields.rs               # Field metadata
│   ├── models/
│   │   ├── mod.rs
│   │   ├── issue.rs                # Issue, IssueFields, IssueSummary
│   │   ├── transition.rs           # Transition, TransitionResult
│   │   ├── comment.rs              # Comment
│   │   ├── project.rs              # Project
│   │   ├── user.rs                 # User
│   │   ├── search.rs               # SearchRequest, SearchResults, JQL types
│   │   └── common.rs               # Pagination, ErrorCollection, Status
│   ├── adf/
│   │   ├── mod.rs                  # ADF type definitions (serde)
│   │   ├── to_markdown.rs          # ADF JSON → Markdown string
│   │   └── from_markdown.rs        # Markdown string → ADF JSON
│   └── schema/
│       ├── mod.rs
│       └── generate.rs             # JSON Schema generation from clap + serde types
├── tests/
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── issues.rs
│   │   ├── search.rs
│   │   ├── transitions.rs
│   │   └── comments.rs
│   └── fixtures/
│       ├── issue_EX-1.json
│       ├── search_results.json
│       ├── transitions.json
│       └── comments.json
└── completions/
    ├── jari.bash
    ├── jari.zsh
    └── jari.fish
```

---

## 3. CLI Command Tree

```
jari [OPTIONS] <COMMAND>

OPTIONS:
  --config <PATH>          Config file path [default: ~/.config/jari/config.toml]
  --url <URL>              Jira base URL (overrides config/env)
  --email <EMAIL>          Jira email (overrides config/env)
  --token <TOKEN>          API token (overrides config/env)
  --output <FORMAT>        Output format: json, json-schema, json-pretty [default: json]
  --verbose, -v            Enable verbose logging to stderr
  --help, -h               Print help

COMMANDS:
  issue
    get <KEY>              Get issue details (description rendered as markdown)
        --fields <FIELDS>  Comma-separated extra fields to include

    create                 Create a new issue (description provided as markdown, auto-converted to ADF)
        --project <KEY>    Project key (required)
        --summary <TEXT>   Issue summary/title (required)
        --type <TYPE>      Issue type: Story, Bug, Task, etc. [default: Task]
        --description <TEXT|@FILE>  Description in markdown (use @file.md for file input)
        --priority <PRI>   Priority: Highest, High, Medium, Low, Lowest
        --assignee <USER>  Assignee account ID or email
        --labels <LABELS>  Comma-separated labels
        --parent <KEY>     Parent issue key (for subtasks)
        --epic-link <KEY>  Epic link key

    edit <KEY>             Edit issue fields
        --summary <TEXT>   New summary
        --description <TEXT|@FILE>  New description (markdown)
        --priority <PRI>   New priority
        --labels <LABELS>  New labels (replaces existing)
        --add-label <LBL>  Add a label (can repeat)

    delete <KEY>           Delete an issue (requires confirmation unless --force)
        --force

    assign <KEY> <USER>    Assign issue to user (account ID or email)

    watch add <KEY>        Start watching an issue
    watch remove <KEY>     Stop watching an issue

  search <JQL>             Search issues with JQL. Auto-paginates — returns ALL matching issues.
        --max <N>          Limit results [default: fetch all]
        --fields <FIELDS>  Comma-separated fields [default: summary,status,assignee,priority,issuetype]

  transition
    list <KEY>             List available transitions for an issue
    do <KEY> <TRANSITION>  Execute a transition (by ID or name match)
        --comment <TEXT>   Optional comment to add during transition
        --resolution <R>   Resolution to set (if required by workflow)

  comment
    list <KEY>             List comments on an issue (newest first)
        --max <N>          Limit comments [default: all]
    get <KEY> <ID>         Get a specific comment by ID
    add <KEY> <BODY>       Add a comment (markdown → ADF)
        --visibility <V>   Comment visibility: group:<name> or role:<id>

  project
    list                   List all accessible projects
        --type <TYPE>      Filter: software, service_desk, business
    get <KEY>              Get project details (name, description, lead, versions, components)

  field
    list                   List all fields (system + custom) with IDs

  me                       Get current authenticated user info (accountId, name, email)

  config
    show                   Print current config (secrets masked)
    path                   Print config file path
    init                   Interactive setup wizard (prompts for URL, email, token)

  schema                   Output JSON Schema of all commands for LLM tool-use discovery
        --openai           Output in OpenAI function-calling format
        --anthropic        Output in Anthropic tool-use format

  completions <SHELL>      Generate shell completion script (bash, zsh, fish)
```

---

## 4. Configuration Design

### Config File

**Location**: `~/.config/jari/config.toml` (resolved via `directories` crate)

```toml
[connection]
url = "https://your-company.atlassian.net"
email = "you@company.com"
token = "your-api-token"

[defaults]
project = "PROJ"              # Default project key (used when --project omitted)
max_results = 100             # Page size for search/list operations

[output]
format = "json"               # json | json-pretty
timezone = "local"            # local | utc
```

### Environment Variable Overrides

| Env var | Overrides | Example |
|---------|-----------|---------|
| `JARI_URL` | `connection.url` | `https://company.atlassian.net` |
| `JARI_EMAIL` | `connection.email` | `bot@company.com` |
| `JARI_TOKEN` | `connection.token` | `ATATT3xFf...` |
| `JARI_PROJECT` | `defaults.project` | `PROJ` |
| `JARI_OUTPUT` | `output.format` | `json-pretty` |

### Resolution Order

1. CLI flags (`--url`, `--email`, `--token`)
2. Environment variables
3. Config file
4. No defaults for credentials — must be explicitly configured

### Config Validation on Load

- URL must be a valid HTTPS URL matching `https://*.atlassian.net` or `https://*.jira.com`
- Email must contain `@`
- Token must be non-empty
- Invalid config produces a clear JSON error with actionable fix suggestions

---

## 5. ADF ↔ Markdown Conversion

### ADF → Markdown (read direction)

| ADF Node Type | Markdown Output |
|---------------|-----------------|
| `doc` | Root container (no output, traverse children) |
| `paragraph` | Text content + `\n\n` |
| `heading` (level 1-6) | `#` × level + ` ` + text + `\n\n` |
| `text` (plain) | Literal text content |
| `text` + `strong` mark | `**text**` |
| `text` + `em` mark | `*text*` |
| `text` + `code` mark | `` `text` `` |
| `text` + `link` mark | `[text](href)` (uses `attrs.href`) |
| `text` + `strike` mark | `~~text~~` |
| `text` + `underline` mark | `<u>text</u>` (no native markdown) |
| `text` + `subsup` mark | `<sub>text</sub>` / `<sup>text</sup>` |
| `text` + `textColor` mark | Plain text (color lost in markdown) |
| `bulletList` | `- ` per `listItem` child |
| `orderedList` | `1. ` per `listItem` child (sequential numbering) |
| `listItem` | Content indented + `\n` |
| `codeBlock` | ` ``` ` + language (from `attrs.language`) + `\n` + code + `\n` + ` ``` ` + `\n\n` |
| `blockquote` | `> ` prefixed per line |
| `rule` | `---\n\n` |
| `mention` | `@` + `attrs.displayName` (fallback: `@` + `attrs.id`) |
| `emoji` | `:` + `attrs.shortName` + `:` |
| `hardBreak` | `\n` (single newline, no blank line) |
| `table` | GitHub-Flavored Markdown table (header separator row) |
| `tableRow` | `| cell1 | cell2 |` |
| `tableCell` | Cell text |
| `tableHeader` | Like `tableCell` but triggers separator row |
| `mediaSingle` / `media` | `![alt](url)` (uses `attrs.alt` or filename) |
| `inlineCard` | `[url](url)` (uses `attrs.url`) |
| `blockCard` | `[title](url)` (uses `attrs.data.title`) |
| `panel` | `> **` + `attrs.panelType` + `:** content` (admonition-style) |
| `taskList` | `- [x] ` or `- [ ] ` per `taskItem` |
| `taskItem` | Checkbox + content |
| `date` | ISO date string from `attrs.timestamp` |
| `status` | `[STATUS: text]` (color info lost) |
| `expand` | `<details><summary>title</summary>content</details>` (HTML passthrough) |
| `placeholder` | `[PLACEHOLDER: text]` |
| Unknown node | JSON inline as `<pre><code>...</code></pre>` (debug passthrough) |

### Markdown → ADF (write direction)

Parsed with `pulldown-cmark`. Reverse mapping:

| Markdown | ADF Output |
|----------|------------|
| Document | `doc` with version 1, type `"doc"` |
| Paragraph | `paragraph` with `text` children |
| Heading (level N) | `heading` with `attrs.level = N` |
| `**bold**` | `text` with `marks: [{type: "strong"}]` |
| `*italic*` | `text` with `marks: [{type: "em"}]` |
| `` `code` `` | `text` with `marks: [{type: "code"}]` |
| `[text](url)` | `text` with `marks: [{type: "link", attrs: {href: "url"}}]` |
| `~~strike~~` | `text` with `marks: [{type: "strike"}]` |
| Unordered list | `bulletList` > `listItem` > `paragraph` |
| Ordered list | `orderedList` > `listItem` > `paragraph` |
| Code block (fenced) | `codeBlock` with `attrs.language` |
| Blockquote | `blockquote` |
| `---` (horizontal rule) | `rule` |
| GF Markdown table | `table` > `tableRow` > `tableCell` / `tableHeader` |
| `![alt](url)` image | `mediaSingle` > `media` with `attrs` |
| Task list `- [ ]` / `- [x]` | `taskList` > `taskItem` |
| HTML `<details>` | `expand` node |
| Unknown HTML | Stripped or `text` passthrough |

### Edge Cases

- **Nested marks**: Bold + italic = `[{type: "strong"}, {type: "em"}]` in marks array
- **Empty document**: Returns minimal ADF: `{"type": "doc", "version": 1, "content": []}`
- **Custom fields containing ADF**: Pass through as-is (identified by field schema type = `"doc"`)
- **Mentions from markdown**: Cannot create true mentions without user ID; treat `@name` as plain text in v1
- **Line breaks**: Single `\n` → `hardBreak`; double `\n\n` → new paragraph
- **Nested lists**: ADF lists auto-nest via indentation level

---

## 6. API Client Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌──────────────┐
│  CLI Args    │────▶│   Config    │────▶│  JiraClient  │────▶│  Jira Cloud   │
│ (clap)       │     │ (file+env)  │     │              │     │  REST API v3  │
└─────────────┘     └─────────────┘     │ ┌──────────┐ │     └──────────────┘
                                        │ │ reqwest   │ │
                                        │ │ Client    │─┼──── HTTP ──────▶
                                        │ └──────────┘ │
                                        │ ┌──────────┐ │
                                        │ │ Auth      │ │  Basic auth header
                                        │ │ Middleware│ │
                                        │ └──────────┘ │
                                        │ ┌──────────┐ │
                                        │ │ Paginator │ │  Auto-fetch loop
                                        │ └──────────┘ │
                                        │ ┌──────────┐ │
                                        │ │ Retry     │ │  Exponential backoff
                                        │ │ Layer     │ │
                                        │ └──────────┘ │
                                        │ ┌──────────┐ │
                                        │ │ ADF Conv. │ │  Convert on read/write
                                        │ └──────────┘ │
                                        │ ┌──────────┐ │
                                        │ │ Rate      │ │  Pre-request sleep
                                        │ │ Limiter   │ │  if bucket low
                                        │ └──────────┘ │
                                        └──────┬───────┘
                                               │
                                        ┌──────▼───────┐
                                        │   Output     │
                                        │  JSON (stdout)│
                                        └──────────────┘
```

### JiraClient

```rust
pub struct JiraClient {
    base_url: String,          // https://company.atlassian.net
    http: reqwest::Client,     // Reusable connection pool
    auth_header: String,       // "Basic base64(user:token)"
    max_page_size: usize,      // Default 100
    max_retries: u32,          // Default 3
    retry_base_ms: u64,        // Default 1000ms
}
```

### Key Behaviors

- **Auto-pagination**: All list/search endpoints automatically follow pagination. The LLM never sees `startAt`/`maxResults`/`isLast` — it gets the full result set in one response.
- **Retry with jitter**: On 429 (rate limit) or 5xx, retry up to 3 times with exponential backoff + random jitter. Respects `Retry-After` header if present.
- **Timeout**: 30s connect, 60s read. Configurable.
- **User-Agent**: `jari/{version} (Rust CLI; +https://github.com/...)` — Atlassian requires a meaningful UA.
- **Concurrent page fetches**: During auto-pagination, fetch pages sequentially to avoid rate limit issues.
- **Request logging**: At `trace` level, log full request/response (sanitized of auth headers).

---

## 7. JSON Output Format (LLM-Optimized)

### Success Response

```json
{
  "ok": true,
  "data": {
    "key": "PROJ-123",
    "summary": "Fix login page timeout",
    "description": "## Problem\nThe login page times out after 30s when...\n\n## Acceptance Criteria\n- [ ] Increase timeout to 60s\n- [ ] Add loading spinner",
    "status": "In Progress",
    "assignee": {"name": "Jane Smith", "email": "jane@company.com"},
    "priority": "High",
    "issuetype": "Bug",
    "created": "2026-05-15T10:30:00Z",
    "updated": "2026-05-28T14:22:00Z"
  },
  "meta": {
    "command": "jari issue get PROJ-123",
    "duration_ms": 234
  }
}
```

### Error Response

```json
{
  "ok": false,
  "error": {
    "code": "not_found",
    "message": "Issue does not exist or you do not have permission to view it.",
    "http_status": 404,
    "jira_errors": {
      "errorMessages": ["Issue does not exist or you do not have permission to view it."],
      "errors": {}
    },
    "suggestion": "Verify the issue key is correct and you have Browse Projects permission."
  },
  "meta": {
    "command": "jari issue get NOPE-999",
    "duration_ms": 312
  }
}
```

### Error Codes (Machine-Readable)

| Code | HTTP | Meaning |
|------|------|---------|
| `auth_failed` | 401 | Invalid credentials or expired token |
| `permission_denied` | 403 | Valid auth but insufficient permissions |
| `not_found` | 404 | Resource doesn't exist |
| `validation_error` | 400/422 | Invalid input (missing fields, bad values) |
| `rate_limited` | 429 | Too many requests |
| `server_error` | 5xx | Jira-side error (retryable) |
| `network_error` | — | DNS, connection refused, timeout |
| `config_error` | — | Missing or invalid configuration |
| `adf_error` | — | Failed to parse or convert ADF content |
| `cli_error` | — | Invalid CLI arguments or usage |

### Design Principles for LLMs

- `ok: bool` at top level — LLM can branch immediately on success/failure
- `data` shape is stable and predictable per command (documented via JSON Schema)
- `error.code` is a stable enum string — LLM can handle specific error types
- `error.suggestion` gives actionable human-readable fix guidance
- `meta.command` reproduces the exact invocation for debugging
- **stdout**: Always valid JSON (success or error)
- **stderr**: Log messages only (LLM ignores)
- **Exit codes**: 0 on success, 1 on error, 2 on CLI usage error

---

## 8. JSON Schema Generation (`jari schema`)

The `jari schema` command outputs a complete tool-use schema document. This is derived programmatically from the clap command definitions and serde model types, so it never drifts from reality.

### Output Format Options

- `--openai`: OpenAI function-calling format
- `--anthropic`: Anthropic tool-use format
- Default: Custom verbose format that includes both command templates and output schemas

### Example (OpenAI-style)

```json
{
  "name": "jari",
  "description": "Jira Cloud CLI for retrieving tasks, updating status, and creating issues. All commands output JSON.",
  "version": "0.1.0",
  "functions": [
    {
      "name": "issue_get",
      "description": "Get full details of a single Jira issue. Returns description in markdown, status, assignee, priority, and all standard fields.",
      "parameters": {
        "type": "object",
        "properties": {
          "key": {
            "type": "string",
            "description": "The issue key (e.g., 'PROJ-123')"
          },
          "fields": {
            "type": "string",
            "description": "Comma-separated additional fields to include"
          }
        },
        "required": ["key"]
      },
      "command": "jari issue get {key}",
      "output_schema": {
        "type": "object",
        "properties": {
          "ok": {"type": "boolean"},
          "data": {"$ref": "#/definitions/Issue"},
          "meta": {"type": "object"}
        }
      }
    },
    {
      "name": "search",
      "description": "Search Jira issues using JQL. Automatically fetches ALL pages — no manual pagination needed. Returns up to 1000 issues.",
      "parameters": {
        "type": "object",
        "properties": {
          "jql": {
            "type": "string",
            "description": "JQL query. Examples: 'project = PROJ AND assignee = currentUser()', 'status = \"In Progress\" ORDER BY priority DESC'"
          },
          "max": {
            "type": "integer",
            "description": "Maximum results (default: all, capped at 1000)"
          },
          "fields": {
            "type": "string",
            "description": "Comma-separated fields (default: summary,status,assignee,priority,issuetype)"
          }
        },
        "required": ["jql"]
      },
      "command": "jari search '{jql}'",
      "output_schema": {
        "type": "object",
        "properties": {
          "ok": {"type": "boolean"},
          "data": {
            "type": "array",
            "items": {"$ref": "#/definitions/IssueSummary"},
            "description": "All matching issues (auto-paginated)"
          },
          "meta": {"type": "object"}
        }
      }
    },
    {
      "name": "transition_list",
      "description": "List all available workflow transitions for an issue. Returns transition IDs and names.",
      "parameters": {
        "type": "object",
        "properties": {
          "key": {
            "type": "string",
            "description": "The issue key (e.g., 'PROJ-123')"
          }
        },
        "required": ["key"]
      },
      "command": "jari transition list {key}",
      "output_schema": {
        "type": "object",
        "properties": {
          "ok": {"type": "boolean"},
          "data": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "id": {"type": "string"},
                "name": {"type": "string"},
                "to": {"type": "object", "properties": {"name": {"type": "string"}}}
              }
            }
          }
        }
      }
    },
    {
      "name": "transition_do",
      "description": "Execute a workflow transition on an issue (e.g., move to 'In Progress', 'Done'). Use transition_list first to see available transitions.",
      "parameters": {
        "type": "object",
        "properties": {
          "key": {
            "type": "string",
            "description": "The issue key (e.g., 'PROJ-123')"
          },
          "transition": {
            "type": "string",
            "description": "Transition ID or name (partial match supported, e.g., 'In Progress', 'Done')"
          },
          "comment": {
            "type": "string",
            "description": "Optional comment to add during transition (markdown)"
          },
          "resolution": {
            "type": "string",
            "description": "Optional resolution name to set (e.g., 'Done', 'Won\\'t Do')"
          }
        },
        "required": ["key", "transition"]
      },
      "command": "jari transition do {key} {transition}",
      "output_schema": {
        "type": "object",
        "properties": {
          "ok": {"type": "boolean"},
          "data": {
            "type": "object",
            "properties": {
              "transition": {"type": "string"},
              "from_status": {"type": "string"},
              "to_status": {"type": "string"}
            }
          }
        }
      }
    },
    {
      "name": "issue_create",
      "description": "Create a new Jira issue (story, bug, task, subtask). Description accepts markdown which is auto-converted to Jira's rich text format.",
      "parameters": {
        "type": "object",
        "properties": {
          "project": {"type": "string", "description": "Project key"},
          "summary": {"type": "string", "description": "Issue title/summary"},
          "type": {"type": "string", "description": "Issue type: Story, Bug, Task, Subtask (default: Task)"},
          "description": {"type": "string", "description": "Issue description in markdown"},
          "priority": {"type": "string", "description": "Priority: Highest, High, Medium, Low, Lowest"},
          "assignee": {"type": "string", "description": "Assignee account ID or email"},
          "labels": {"type": "string", "description": "Comma-separated labels"},
          "parent": {"type": "string", "description": "Parent issue key (for subtasks)"},
          "epic_link": {"type": "string", "description": "Epic issue key to link"}
        },
        "required": ["project", "summary"]
      },
      "command": "jari issue create --project {project} --summary '{summary}'",
      "output_schema": {
        "type": "object",
        "properties": {
          "ok": {"type": "boolean"},
          "data": {
            "type": "object",
            "properties": {
              "key": {"type": "string", "description": "New issue key (e.g., PROJ-456)"},
              "id": {"type": "string"},
              "url": {"type": "string", "description": "Direct link to the issue"}
            }
          }
        }
      }
    },
    {
      "name": "comment_add",
      "description": "Add a comment to an issue. Comment text is markdown auto-converted to Jira's rich text.",
      "parameters": {
        "type": "object",
        "properties": {
          "key": {"type": "string", "description": "Issue key (e.g., 'PROJ-123')"},
          "body": {"type": "string", "description": "Comment text in markdown"}
        },
        "required": ["key", "body"]
      },
      "command": "jari comment add {key} '{body}'",
      "output_schema": {
        "type": "object",
        "properties": {
          "ok": {"type": "boolean"},
          "data": {
            "type": "object",
            "properties": {
              "id": {"type": "string"},
              "created": {"type": "string"}
            }
          }
        }
      }
    },
    {
      "name": "issue_assign",
      "description": "Assign an issue to a user.",
      "parameters": {
        "type": "object",
        "properties": {
          "key": {"type": "string", "description": "Issue key (e.g., 'PROJ-123')"},
          "user": {"type": "string", "description": "User account ID or email"}
        },
        "required": ["key", "user"]
      },
      "command": "jari issue assign {key} {user}"
    },
    {
      "name": "project_list",
      "description": "List all accessible Jira projects.",
      "parameters": {
        "type": "object",
        "properties": {},
        "required": []
      },
      "command": "jari project list"
    },
    {
      "name": "me",
      "description": "Get the currently authenticated Jira user's info.",
      "parameters": {
        "type": "object",
        "properties": {},
        "required": []
      },
      "command": "jari me"
    }
  ]
}
```

---

## 9. Error Handling Model

```
JariError (enum)
├── Config(String)           — Missing/invalid config
├── Auth                     — 401 Unauthorized
│   ├── status: 401
│   └── suggestion: "Check your email and API token. Generate a token at https://id.atlassian.com/manage-profile/security/api-tokens"
├── Permission               — 403 Forbidden
│   ├── status: 403
│   └── suggestion: "You don't have permission for this action. Contact your Jira admin."
├── NotFound                 — 404 Not Found
│   ├── status: 404
│   └── suggestion: "The requested resource was not found. Verify the key/ID."
├── Validation               — 400/422 Bad Request
│   ├── status: 400/422
│   ├── jira_errors: HashMap<String, String>  (raw Jira error details)
│   └── suggestion: "Check the required fields and values. Use 'jari field list' to see valid fields."
├── RateLimit                — 429 Too Many Requests
│   ├── status: 429
│   ├── retry_after: Option<Duration>
│   └── suggestion: "Rate limited. Wait before retrying."
├── ServerError              — 5xx
│   ├── status: 5xx
│   └── suggestion: "Jira server error. This is usually temporary. Retry in a moment."
├── Network(reqwest::Error)  — DNS, timeout, connection refused
│   └── suggestion: "Network error. Check your connection and the Jira URL."
├── AdfConversion(String)    — Failed to parse/convert ADF
│   └── suggestion: "Failed to convert rich text. The content may contain unsupported formatting."
└── Cli(String)              — Invalid CLI arguments
    └── suggestion: "Check the command syntax. Use --help for usage."
```

Each variant maps to:
- A stable `error.code` string
- An HTTP status code (when applicable)
- Raw Jira error details (when applicable)
- A human-readable `suggestion` string
- Exits with code 1 (except Cli which exits with code 2)

---

## 10. API Endpoints Used

All endpoints are Jira Cloud REST API v3 (`/rest/api/3/`).

| CLI Command | HTTP Method | API Path | Notes |
|-------------|-------------|----------|-------|
| `issue get` | `GET` | `/issue/{key}` | Expand `renderedFields` for markdown conversion |
| `issue create` | `POST` | `/issue` | Body includes ADF-converted description |
| `issue edit` | `PUT` | `/issue/{key}` | Partial update with fields hash |
| `issue delete` | `DELETE` | `/issue/{key}` | Requires confirmation |
| `issue assign` | `PUT` | `/issue/{key}/assignee` | Body: `{accountId}` or `{name}` |
| `issue watch add` | `POST` | `/issue/{key}/watchers` | Body: accountId |
| `issue watch remove` | `DELETE` | `/issue/{key}/watchers?accountId=...` | |
| `search` | `POST` | `/search` | Body: `{jql, startAt, maxResults, fields}` |
| `search` (alt) | `GET` | `/search?jql=...` | Simple queries, GET is simpler but limited length |
| `transition list` | `GET` | `/issue/{key}/transitions` | |
| `transition do` | `POST` | `/issue/{key}/transitions` | Body: `{transition: {id}}` |
| `comment list` | `GET` | `/issue/{key}/comment` | Paginated |
| `comment get` | `GET` | `/issue/{key}/comment/{id}` | |
| `comment add` | `POST` | `/issue/{key}/comment` | Body: `{body: ADF}` |
| `project list` | `GET` | `/project/search` | |
| `project get` | `GET` | `/project/{key}` | |
| `field list` | `GET` | `/field` | |
| `me` | `GET` | `/myself` | |

---

## 11. Implementation Phases

### Phase 1: Foundation — CLI Skeleton & Auth (Days 1-3)

**Goal**: CLI compiles, config loads, auth works, can GET a single issue.

**Tasks**:

1. `cargo init` with all dependencies in `Cargo.toml`
2. Set up `tracing-subscriber` (stderr logging, JSON mode when `--verbose`)
3. Implement `config.rs`:
   - Load TOML from `~/.config/jari/config.toml` via `directories` crate
   - Override with env vars (`JARI_URL`, `JARI_EMAIL`, `JARI_TOKEN`)
   - Override with CLI flags
   - Validation (URL format, email format, token non-empty)
   - `Config` struct with `Default` for optional fields
4. Implement `error.rs`:
   - Full `JariError` enum with `thiserror`
   - `Serialize` impl for JSON error output
   - `From<reqwest::Error>`, `From<serde_json::Error>` conversions
5. Implement `output.rs`:
   - `Output<T>` struct: `{ok, data, meta}`
   - `Output::success(data)` and `Output::error(err)` constructors
   - `meta.command` and `meta.duration_ms` auto-populated
6. Implement `client/mod.rs`:
   - `JiraClient::new(config)` — build `reqwest::Client` with auth header
   - `JiraClient::get<T>(path)` — generic GET helper
   - `JiraClient::post<T, B>(path, body)` — generic POST helper
   - `JiraClient::put<T, B>(path, body)` — generic PUT helper
   - `JiraClient::delete(path)` — generic DELETE helper
   - Auth header: `Basic base64(email:token)`
   - User-Agent header
7. Implement `models/common.rs`:
   - `ErrorCollection` (Jira error response)
   - `Status` (category, name, id)
8. Implement `models/issue.rs`:
   - `Issue` struct with all standard fields
   - `IssueFields` embedded struct
   - `IssueSummary` for search results (subset of fields)
9. Implement `client/issues.rs`:
   - `get_issue(key, fields)` → `Issue`
10. Implement `cli.rs`:
    - `Cli` struct with `#[derive(Parser)]`
    - `issue get <KEY>` subcommand
    - Top-level flags: `--config`, `--url`, `--email`, `--token`, `--verbose`
11. Implement `main.rs`:
    - Parse args → load config → create client → dispatch command → print JSON
    - Measure and include `duration_ms` in meta
12. Write first integration test:
    - `wiremock` server serving a mock issue JSON
    - Assert output JSON shape

**Deliverable**: `jari issue get PROJ-123` returns JSON with issue data.

### Phase 2: ADF Converter (Days 4-6)

**Goal**: Descriptions and comments render as clean markdown.

**Tasks**:

1. Implement `adf/mod.rs`:
   - ADF document types: `Doc`, `Node`, `Mark` enums (serde)
   - `Node::parse(json)` — deserialize ADF JSON
2. Implement `adf/to_markdown.rs`:
   - Recursive walker over ADF nodes
   - Handle all node types from the mapping table above
   - `fn adf_to_markdown(json: &Value) -> Result<String>`
   - Soft error handling: unknown nodes become `[UNKNOWN: type]`
3. Implement `adf/from_markdown.rs`:
   - Parse markdown with `pulldown-cmark` → ADF node tree
   - `fn markdown_to_adf(md: &str) -> Result<Value>`
   - Handle code blocks with language, tables, lists, tasks
4. Wire into issue get: `description` and `comment.body` auto-converted to markdown in output
5. Wire into issue create: `--description` markdown auto-converted to ADF in request body
6. Unit tests:
   - Round-trip tests: markdown → ADF → markdown (should be semantically equivalent)
   - Specific ADF fixtures from real Jira data
   - Edge cases: empty doc, deeply nested lists, code blocks, tables

**Deliverable**: `jari issue get PROJ-123` shows human-readable markdown description.

### Phase 3: Search & Read Operations (Days 7-10)

**Goal**: Search, list issues, read comments, list projects, look up fields.

**Tasks**:

1. Implement `models/search.rs`:
   - `SearchRequest`: `{jql, startAt, maxResults, fields, fieldsByKeys}`
   - `SearchResults`: `{total, issues: Vec<IssueSummary>}`
2. Implement `client/search.rs`:
   - `search(jql, fields, max_override)` — auto-paginates internally:
     - Loop: fetch page, append to results, increment `startAt`
     - Stop when `startAt >= total` or hit `--max` limit
     - Soft cap at 1000 results (Jira API limit without deep pagination)
3. Implement `client/comments.rs`:
   - `list_comments(key, max)` — auto-paginates
   - `get_comment(key, id)`
4. Implement `client/projects.rs`:
   - `list_projects(type_filter)`
   - `get_project(key)`
5. Implement `client/users.rs`:
   - `get_current_user()` → `/myself`
6. Implement `client/fields.rs`:
   - `list_fields()`
7. Add CLI subcommands:
   - `search <JQL>` with `--max`, `--fields` flags
   - `comment list <KEY>` with `--max`
   - `comment get <KEY> <ID>`
   - `project list` with `--type`
   - `project get <KEY>`
   - `field list`
   - `me`
8. Integration tests for search pagination, comment listing

**Deliverable**: Full read-side CLI: search, browse, inspect.

### Phase 4: Write Operations (Days 11-14)

**Goal**: Create issues, transition status, add comments, assign users.

**Tasks**:

1. Implement `client/transitions.rs`:
   - `list_transitions(key)`
   - `do_transition(key, transition_id_or_name, comment, resolution)`
   - Transition name fuzzy matching: if user passes a name like "In Progress", resolve to the actual transition ID
2. Wire up `client/issues.rs` write methods:
   - `create_issue(request)` — POST with ADF-converted description
   - `edit_issue(key, fields)` — PUT partial update
   - `delete_issue(key)` — DELETE
   - `assign_issue(key, user)` — PUT assignee
   - `add_watcher(key, account_id)` / `remove_watcher`
3. Implement `client/comments.rs`:
   - `add_comment(key, body_md)` — POST with markdown→ADF converted body
4. Add CLI subcommands:
   - `issue create` with `--project`, `--summary`, `--type`, `--description`, `--priority`, `--assignee`, `--labels`, `--parent`, `--epic-link`
   - `issue edit <KEY>` with editable fields
   - `issue delete <KEY>` with `--force`
   - `issue assign <KEY> <USER>`
   - `issue watch add <KEY>` / `issue watch remove <KEY>`
   - `transition list <KEY>`
   - `transition do <KEY> <TRANSITION>` with `--comment`, `--resolution`
   - `comment add <KEY> <BODY>` with `--visibility`
5. Input validation:
   - Issue type validation against project's available types
   - Priority validation against available priorities
   - Transition name fuzzy match with "did you mean?" suggestions

**Deliverable**: Full CRUD CLI: agents can create, update, transition, and comment on issues.

### Phase 5: JSON Schema & LLM Tool Integration (Days 15-17)

**Goal**: `jari schema` outputs accurate tool definitions for LLM consumption.

**Tasks**:

1. Implement `schema/generate.rs`:
   - Walk clap command tree programmatically
   - Map each subcommand to a tool definition with:
     - `name`: snake_case command identifier
     - `description`: from clap doc comments
     - `parameters`: from clap argument definitions
     - `command_template`: string with `{placeholder}` substitution
     - `output_schema`: derived from serde return type (via `schemars`)
   - Support `--openai` and `--anthropic` format flags
2. Generate `schemars::JsonSchema` for all output types:
   - `Issue`, `IssueSummary`, `Transition`, `Comment`, `Project`, `User`
3. Add `config show` and `config path` subcommands:
   - `config show` masks the token value
4. Add `config init` interactive setup:
   - Prompt for URL, email, token
   - Validate connection by calling `/myself`
   - Write config file
5. Generate shell completion scripts:
   - `jari completions bash|zsh|fish` via `clap_complete`
   - Pre-build and commit completion files
6. Final CLI polish:
   - Consistent help text across all subcommands
   - Examples in help text
   - Short flags for common options (`-p` for `--project`, `-s` for `--summary`)

**Deliverable**: LLM agents can do `jari schema` to discover all capabilities, then invoke any command.

### Phase 6: Testing, CI/CD, Distribution (Days 18-21)

**Goal**: Production-ready with automated tests, builds, and distribution.

**Tasks**:

1. **Test coverage**:
   - Unit tests for ADF converter (round-trip + edge cases)
   - Unit tests for config loading (file, env, flag precedence)
   - Unit tests for error serialization
   - Integration tests with `wiremock` for all API operations:
     - Happy paths (success responses)
     - Error paths (401, 403, 404, 422, 429, 500, network error)
     - Auto-pagination (multi-page responses)
   - Integration test for CLI arg parsing (all subcommands, required fields)
   - Snapshot tests for JSON output format stability

2. **CI pipeline** (`.github/workflows/ci.yml`):
   - `cargo check` on all platforms (macOS, Linux, Windows)
   - `cargo test` with full test suite
   - `cargo clippy` (strict, deny warnings)
   - `cargo fmt --check`
   - `cargo doc --no-deps` (check docs don't have broken links)

3. **Release pipeline** (`.github/workflows/release.yml`):
   - Triggered by git tag `v*.*.*`
   - `cargo-dist` builds:
     - macOS ARM64 (Apple Silicon)
     - macOS x86_64 (Intel)
     - Linux x86_64 (musl static)
     - Linux ARM64 (musl static)
   - GitHub Release with binaries + SHA256 checksums
   - Generate Homebrew formula, push to homebrew tap repo
   - Publish to crates.io (`cargo publish`)

4. **README.md**:
   - Quickstart: install → config → first command
   - Complete command reference with examples
   - LLM agent usage guide:
     - How to use `jari schema` to discover tools
     - Common LLM workflows (find my tasks, update status, create story)
     - JQL cheatsheet for LLMs
   - Troubleshooting section

**Deliverable**: `brew install jari` or `cargo install jari` works. CI green. Binary releases auto-built.

---

## 12. Model Type Definitions (Key Types)

### Issue (full — `issue get`)

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct IssueFields {
    pub summary: String,
    #[serde(default)]
    pub description_markdown: Option<String>,   // ADF converted
    pub issuetype: IssueType,
    pub status: Status,
    pub priority: Option<Priority>,
    pub assignee: Option<User>,
    pub reporter: Option<User>,
    pub created: String,       // ISO 8601
    pub updated: String,       // ISO 8601
    pub duedate: Option<String>,
    pub resolution: Option<Resolution>,
    pub labels: Vec<String>,
    pub components: Vec<Component>,
    pub fix_versions: Vec<Version>,
    pub versions: Vec<Version>,
    pub parent: Option<IssueLink>,
    pub subtasks: Vec<IssueLink>,
    pub issuelinks: Vec<IssueLinkType>,
    pub timetracking: Option<TimeTracking>,
    pub votes: Option<Votes>,
    pub watches: Option<Watches>,
    pub worklog: Option<Worklog>,
    pub comment: Option<CommentsPage>,
    pub project: ProjectSummary,
}
```

### IssueSummary (search results)

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct IssueSummary {
    pub id: String,
    pub key: String,
    pub fields: IssueSummaryFields,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct IssueSummaryFields {
    pub summary: String,
    pub issuetype: IssueType,
    pub status: Status,
    pub priority: Option<Priority>,
    pub assignee: Option<User>,
    pub created: String,
    pub updated: String,
    pub duedate: Option<String>,
    pub labels: Vec<String>,
}
```

### Transition

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Transition {
    pub id: String,
    pub name: String,
    pub to: TransitionDestination,
    pub has_screen: Option<bool>,
    pub is_conditional: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TransitionDestination {
    pub name: String,
    pub id: String,
    pub status_category: Option<StatusCategory>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TransitionResult {
    pub transition: String,        // Transition name
    pub from_status: String,
    pub to_status: String,
}
```

### Common Types

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct User {
    pub account_id: Option<String>,
    pub email_address: Option<String>,
    pub display_name: String,
    pub active: bool,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Status {
    pub id: String,
    pub name: String,
    pub status_category: Option<StatusCategory>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StatusCategory {
    pub id: u32,
    pub key: String,           // "indeterminate", "new", "done"
    pub name: String,          // "In Progress", "To Do", "Done"
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Priority {
    pub id: String,
    pub name: String,
    pub icon_url: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct IssueType {
    pub id: String,
    pub name: String,
    pub subtask: bool,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Comment {
    pub id: String,
    pub author: User,
    pub body_markdown: Option<String>,   // ADF converted
    pub created: String,
    pub updated: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub lead: Option<User>,
    pub project_type_key: Option<String>,  // "software", "service_desk", "business"
    pub simplified: bool,                   // next-gen project
    pub style: Option<String>,             // "classic" or "next-gen"
    pub versions: Vec<Version>,
    pub components: Vec<Component>,
    pub issue_types: Option<Vec<IssueType>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Pagination {
    pub start_at: usize,
    pub max_results: usize,
    pub total: usize,
    pub is_last: bool,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ErrorCollection {
    #[serde(default)]
    pub error_messages: Vec<String>,
    #[serde(default)]
    pub errors: HashMap<String, String>,
}
```

---

## 13. LLM Agent Workflow Examples

### Workflow 1: Find and Read My Tasks

```bash
# LLM discovers tools
jari schema --openai

# Find tasks assigned to me
jari search "assignee = currentUser() AND status != Done ORDER BY priority DESC"

# Get full details of specific task
jari issue get PROJ-123

# Read comments for context
jari comment list PROJ-123
```

### Workflow 2: Start Working on a Task

```bash
# See available transitions
jari transition list PROJ-123

# Move to In Progress
jari transition do PROJ-123 "In Progress"

# Add a comment about what I'm doing
jari comment add PROJ-123 "Starting implementation. Using approach A with the new library."
```

### Workflow 3: Complete a Task

```bash
# Add completion comment
jari comment add PROJ-123 "## Done\n- Implemented feature X\n- Added tests\n- Updated docs\n\nPR: https://github.com/org/repo/pull/42"

# Move to Done (or Ready for Review depending on workflow)
jari transition do PROJ-123 "Done"
```

### Workflow 4: Create a Story for PO

```bash
# First, check the project and available fields
jari project get PROJ

# Create the story
jari issue create \
  --project PROJ \
  --type Story \
  --summary "Add dark mode support to dashboard" \
  --description "## Problem
Users have requested dark mode for the analytics dashboard.

## Acceptance Criteria
- [ ] Toggle in settings to switch between light/dark/auto
- [ ] All existing components support dark theme
- [ ] Respects system preference by default
- [ ] No visual regressions in light mode

## Technical Notes
- Use CSS variables for theming
- Follow design tokens from the design system" \
  --priority High \
  --labels "frontend,ux,settings"

# Optionally assign to yourself
jari issue assign PROJ-456 "me"
```

---

## 14. Future Roadmap (Post-v1)

These are explicitly out of scope for v1 but should inform architecture decisions today:

| Feature | Why Not Now | Architecture Prep Needed |
|---------|------------|--------------------------|
| MCP server mode | Explicitly descoped | Keep CLI logic decoupled from output layer so it can be wrapped in MCP handlers later |
| Agile boards & sprints | Not in priority workflows | Jira Agile API is a separate REST resource — add as new client module |
| Worklog tracking | Not in priority workflows | `client/worklogs.rs` module, new CLI subcommands |
| OAuth 2.0 support | Only API token auth requested | Auth layer is an enum — add `OAuth(oauth_client)` variant later |
| Batch/bulk operations | Performance optimization for later | API client already handles arrays; CLI syntax is the main design question |
| Webhook listener mode | Real-time not needed for v1 | Separate binary or subcommand; doesn't affect core architecture |
| Custom field type detection | Need field metadata to know which fields are ADF | `field list` already returns this; add to conversion pipeline |
| --format table (human output) | JSON-only was the explicit request | Output layer is pluggable — add `TableFormatter` alongside `JsonFormatter` |
| Jira Data Center/Server | Cloud-only was the explicit request | Config already has `base_url` — just needs alternate auth methods |

---

## 15. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| ADF conversion is lossy | Medium | High — descriptions may lose formatting | Extensive round-trip tests. Document known lossy conversions. Keep original ADF accessible via `--raw` flag. |
| Jira API changes break models | Low | Medium | Use `#[serde(default)]` for all optional fields. Integration tests catch schema drift. |
| Rate limiting on auto-pagination | Medium | Medium | Sequential page fetches. Configurable max results. Retry with backoff. |
| `pulldown-cmark` doesn't handle all GFM extensions | Low | Medium | Tables and task lists are the main GFM features needed. Test thoroughly. |
| Cross-compilation issues (musl) | Medium | Low | Use `cargo-dist` which handles cross-compilation toolchains. CI catches this. |
| Large issue descriptions (100KB+) cause performance issues | Low | Low | Streaming ADF conversion. Configurable trim. Not common for typical Jira usage. |

---

## 16. Success Criteria

- [ ] `cargo install jari` succeeds on macOS and Linux
- [ ] `brew install jari` works on macOS
- [ ] `jari config init` sets up auth in under 30 seconds
- [ ] `jari issue get PROJ-123` returns JSON with markdown description in <500ms
- [ ] `jari search "project = PROJ"` returns all issues (auto-paginated) in <5s for 200 issues
- [ ] `jari schema` outputs valid OpenAI-compatible tool definitions
- [ ] All 3 priority workflows are executable end-to-end:
  1. Fetch and read assigned issues
  2. Transition an issue and add a comment
  3. Create a new story with markdown description
- [ ] All errors return JSON with `ok: false`, stable `error.code`, and actionable `suggestion`
- [ ] 80%+ test coverage on client and ADF modules
- [ ] CI passes on macOS, Linux, and Windows
- [ ] No panics — all errors are handled gracefully
