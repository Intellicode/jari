# Spec 011: main.rs — Entry Point & Command Dispatch

- [ ] Not implemented

## Goal

Wire together config loading, client creation, command dispatch, timing, and JSON output in the `main` function.

## Requirements

### Startup Flow

1. Initialize tracing/logging (spec 002)
2. Parse CLI args via `clap` (spec 010)
3. Load config from file + env vars + CLI overrides (spec 003)
4. Validate config — exit with JSON error if invalid
5. Create `JiraClient` from config (spec 006)
6. Record start time
7. Match on `Cli.command` and dispatch to appropriate handler
8. Measure elapsed time
9. Wrap result in `Output<T>` envelope (spec 005)
10. Serialize to JSON and print to stdout
11. Exit with appropriate code (0 success, 1 error, 2 CLI error)

### Error Handling

- All errors caught at the top level and serialized to JSON
- Never panic — all failures become JSON error output
- `Cli` variant errors exit with code 2

### Initial Dispatch

- Phase 1 minimum: only `issue get` wired up
- Other subcommands return "not yet implemented" error
- Stderr receives log output only

### File

- `src/main.rs` — `#[tokio::main]` async entry point
