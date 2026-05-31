# Spec 060: Release Pipeline

- [ ] Not implemented

## Goal

Set up a GitHub Actions release pipeline that builds cross-compiled binaries and publishes on git tag.

## Requirements

### Workflow File

`.github/workflows/release.yml`:

### Trigger

- Push of a tag matching `v*.*.*` (e.g., `v0.1.0`)

### Build Matrix (via `cargo-dist`)

| Target | OS | Arch |
|--------|----|----|
| `apple-darwin` | macOS | ARM64 (Apple Silicon) |
| `apple-darwin` | macOS | x86_64 (Intel) |
| `linux-musl` | Linux | x86_64 (static) |
| `linux-musl` | Linux | ARM64 (static) |

### Release Assets

1. **Pre-built binaries**: One per platform, named `jari-{platform}.tar.gz`
2. **SHA256 checksums**: `jari-{version}-checksums.txt`
3. **Homebrew formula**: Auto-generated and pushed to Homebrew tap repo
4. **crates.io publish**: `cargo publish` (with `--dry-run` first)

### Process

1. Checkout code at tag
2. Run full CI checks (clippy, fmt, test, doc)
3. Use `cargo-dist` to cross-compile for all targets
4. Create GitHub Release with:
   - Release notes from `CHANGELOG.md` or auto-generated from commits
   - All binary artifacts attached
   - Checksums file attached
5. Generate Homebrew formula:
   - Compute SHA256 of each binary
   - Generate Ruby formula
   - Push to `homebrew-jari` tap repository
6. Publish to crates.io:
   - `cargo login` (token from secrets)
   - `cargo publish`

### Secrets Required

- `CRATES_IO_TOKEN` — for crates.io publishing
- `HOMEBREW_TAP_TOKEN` — for pushing to Homebrew tap repo
- `GITHUB_TOKEN` — automatic for release creation

### Requirements

- All builds use `rustls-tls` (no OpenSSL dependency)
- Linux builds use `musl` for static binaries (no glibc dependency)
- macOS builds are universal or split by architecture
- Release is created as draft first for manual review, then published
