# Spec 001: Project Scaffold & Dependencies

- [ ] Not implemented

## Goal

Initialize the Rust project with all required dependencies in `Cargo.toml`, set up the source tree, and ensure `cargo check` passes.

## Requirements

- Single-crate project: `jari/` with `Cargo.toml` at root
- All dependencies declared with appropriate feature flags:

| Crate | Features | Purpose |
|-------|----------|---------|
| `clap` v4 | `derive` | CLI arg parsing |
| `tokio` | `full` | Async runtime |
| `reqwest` | `rustls-tls`, `json` | HTTP client (no OpenSSL) |
| `serde` | `derive` | Serialization |
| `serde_json` | — | JSON handling |
| `schemars` | — | JSON Schema generation |
| `thiserror` | — | Error type derives |
| `miette` | `fancy` | Pretty CLI diagnostics |
| `toml` | — | Config file parsing |
| `directories` | — | XDG config path resolution |
| `tracing` | — | Structured logging |
| `tracing-subscriber` | `env-filter`, `json` | Log output |
| `pulldown-cmark` | — | Markdown to ADF parsing |

- Dev dependencies: `rstest`, `wiremock`, `assert-json-diff`
- Source tree matches structure in PLAN.md section 2
- `cargo check` succeeds with zero warnings
