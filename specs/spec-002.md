# Spec 002: Logging Infrastructure

- [ ] Not implemented

## Goal

Set up `tracing-subscriber` so all log output goes to stderr in a structured, controllable format.

## Requirements

- Logs go to **stderr** only (never stdout — stdout is JSON output)
- Default log level: `WARN` (quiet by default)
- `--verbose` / `-v` flag sets log level to `DEBUG`
- `RUST_LOG` env var overrides log level (standard tracing behavior)
- JSON log format when `JARI_LOG_FORMAT=json` env var is set
- Plain text log format by default (single-line, timestamped)
- Logs are initialized before any CLI dispatch in `main.rs`
- Sensitive data (auth headers) must never appear in logs
