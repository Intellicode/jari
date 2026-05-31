# Spec 061: README.md

- [ ] Not implemented

## Goal

Write a comprehensive README covering installation, configuration, command reference, and LLM agent usage.

## Requirements

### Sections

**1. Quickstart (5-minute setup)**
- Install (brew, cargo, or download binary)
- Configure (`jari config init`)
- First command (`jari me`)

**2. Installation**
- Homebrew: `brew install jari`
- Cargo: `cargo install jari`
- Pre-built binaries: link to GitHub Releases
- Shell completions setup

**3. Authentication**
- How to generate an API token at Atlassian
- Config file location and format
- Environment variable overrides
- `jari config init` wizard

**4. Command Reference**
- Complete list of all commands with:
  - Syntax
  - Description
  - Required and optional arguments
  - Output format
  - 1-2 examples each

**5. LLM Agent Usage Guide**
- How to discover tools: `jari schema --openai` / `jari schema --anthropic`
- JSON output format explanation
- Common LLM workflows:
  - Find my tasks: `jari search "assignee = currentUser() AND status != Done"`
  - Get issue details: `jari issue get PROJ-123`
  - Start working: `jari transition do PROJ-123 "In Progress"`
  - Complete task: `jari transition do PROJ-123 "Done"`
  - Create story: `jari issue create --project PROJ --type Story --summary "..." --description "..."`
  - Add comment: `jari comment add PROJ-123 "Fixed in PR #42"`

**6. JQL Cheatsheet for LLMs**
- Common JQL patterns:
  - `assignee = currentUser()` — my tasks
  - `project = PROJ AND status = "In Progress"` — project progress
  - `created >= -7d` — recently created
  - `priority = High AND resolution = Unresolved` — high priority open
  - `labels in (frontend, bug)` — by label
  - `text ~ "login"` — full-text search

**7. Troubleshooting**
- Common errors and solutions
- How to enable verbose logging
- How to report bugs

### Requirements

- Markdown format, readable on GitHub and crates.io
- All code examples are copy-pasteable
- Links to relevant Atlassian docs
- Badges: CI status, crates.io version, license
