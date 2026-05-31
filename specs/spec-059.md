# Spec 059: CI Pipeline

- [ ] Not implemented

## Goal

Set up a GitHub Actions CI pipeline that runs on every PR and push to main.

## Requirements

### Workflow File

`.github/workflows/ci.yml`:

### Triggers

- `push` to `main` branch
- `pull_request` to `main` branch
- Manual trigger via `workflow_dispatch`

### Jobs

**Matrix build (macOS, Linux, Windows):**
- `cargo check` — verify compilation
- `cargo test` — full test suite
- `cargo clippy -- -D warnings` — strict linting
- `cargo fmt --check` — code formatting
- `cargo doc --no-deps` — verify docs compile

**Platform matrix:**
- `ubuntu-latest`
- `macos-latest`
- `windows-latest`

### Caching

- Cache `~/.cargo/registry/`
- Cache `~/.cargo/git/`
- Cache `target/` directory
- Use `actions/cache` or `Swatinem/rust-cache`

### Toolchain

- Use `dtolnay/rust-toolchain` with `stable`
- Install `clippy` and `rustfmt` components

### Failure Conditions

- Any test failure
- Any clippy warning (treated as error)
- Any formatting violation
- Any doc link broken

### Additional Checks

- **Security audit**: `cargo audit` for known vulnerabilities (optional)
- **MSRV check**: verify minimum supported Rust version (optional)
