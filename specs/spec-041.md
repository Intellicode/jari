# Spec 041: Input Validation for Write Operations

- [ ] Not implemented

## Goal

Add client-side validation for all write operations to produce clear, actionable errors before hitting the API.

## Requirements

### Validation Rules

**Issue Type validation:**
- Validate `--type` against project's available issue types (requires fetching project metadata)
- If invalid, suggest valid types: "Unknown issue type 'Feature'. Available: Story, Bug, Task, Subtask"

**Priority validation:**
- Validate `--priority` against standard Jira priorities
- If invalid: "Unknown priority 'Critical'. Expected: Highest, High, Medium, Low, Lowest"

**Transition name validation:**
- Already handled by transition fuzzy matching (spec 031)
- "Did you mean?" suggestions when multiple partial matches exist: "Multiple transitions match 'Progress': 'In Progress', 'In Progress Review'. Use the exact name or ID."

**Label validation:**
- Validate labels don't contain spaces (Jira doesn't allow spaces in labels)
- Auto-strip whitespace from comma-separated labels

**Required fields:**
- `issue create`: `--project` and `--summary` required (enforced by clap)
- `issue edit`: at least one field flag required
- `comment add`: body must be non-empty

### Validation Architecture

- Validation functions in a dedicated module or inline in CLI handlers
- Validation runs BEFORE API calls to provide fast feedback
- Validation errors use `Cli` error variant with clear messages
- Optional: pre-fetch project metadata for type validation (fail gracefully if fetch fails)
