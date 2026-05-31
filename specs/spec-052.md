# Spec 052: Unit Tests — Config Loading

- [ ] Not implemented

## Goal

Unit tests for configuration loading, resolution precedence, and validation.

## Requirements

### Test Scenarios

**File loading:**
- Load valid TOML config file
- Load config from custom path (`--config`)
- Missing config file: no error when env vars provide credentials
- Malformed TOML: error with clear message

**Resolution precedence:**
1. CLI flag overrides env var: set both, assert CLI wins
2. Env var overrides config file: set in file and env, assert env wins
3. Config file used when no overrides
4. Default values used for optional fields (`defaults.project`, `output.*`)

**Validation:**
- Valid URL (`https://company.atlassian.net`): passes
- Invalid URL (`http://company.atlassian.net` — no TLS): fails
- Invalid URL (`https://evil.com` — wrong domain): fails
- Invalid URL (not HTTPS): fails
- Valid email (`user@company.com`): passes
- Invalid email (no `@`): fails
- Empty token: fails
- Missing token from all sources: fails with "API token is required"

**Source tracking:**
- Each config field tracks its origin (file, env, CLI, default)
- `config show` correctly reports sources

### Test Helpers

- Temp TOML file creation in temp dir
- Env var set/restore (use `temp_env` or manual set/remove)
- CLI args simulation

### Test Files

- `src/config.rs` — `#[cfg(test)] mod tests`
- Separate config test fixtures if needed
