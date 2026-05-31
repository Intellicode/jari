# Spec 044: Schemars Derives on All Output Types

- [ ] Not implemented

## Goal

Derive `JsonSchema` on all model types so the schema generator can include precise output schemas for each command.

## Requirements

### Types Requiring `JsonSchema`

- `Issue`, `IssueFields`, `IssueSummary`, `IssueSummaryFields`
- `Transition`, `TransitionDestination`, `TransitionResult`
- `Comment`, `CreatedComment`
- `Project`, `ProjectSummary`
- `User`
- `Status`, `StatusCategory`
- `Priority`, `IssueType`
- `SearchResults`
- `Field`, `FieldSchema`
- `Output<T>` — the envelope itself
- `CreateIssueRequest`, `CreatedIssue`
- All common types: `Version`, `Component`, `Resolution`, `IssueLink`, `Votes`, `Watches`, `TimeTracking`, `Worklog`

### Requirements

- `use schemars::JsonSchema;` on all models
- `#[derive(JsonSchema)]` alongside existing derives
- `#[schemars(title = "...", description = "...")]` for key types to aid LLM understanding
- `serde(rename)` attributes respected by schemars automatically
- `Option<T>` maps to nullable in schema
- Generate a root `Definitions` struct that references all types

### Integration with Spec 043

- Each `ToolDefinition.output_schema` is populated by calling `schemars::schema_for::<ReturnType>()`
- Define a mapping: command → return type (e.g., `issue_get` → `Issue`, `search` → `Vec<IssueSummary>`)
- Schema references use `$ref` to avoid duplication

### File

- Derives on existing model files in `src/models/`
- Schema root definitions in `src/schema/mod.rs`
