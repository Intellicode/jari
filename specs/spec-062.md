# Spec 062: Homebrew Formula & crates.io Publish

- [ ] Not implemented

## Goal

Set up Homebrew tap distribution and crates.io publication so users can install via `brew install jari` or `cargo install jari`.

## Requirements

### Homebrew Tap

- Repository: `github.com/anomalyco/homebrew-jari`
- Formula file: `Formula/jari.rb`
- Generated automatically by release pipeline (spec 060)
- Formula correctly computes SHA256 for each platform binary
- Formula includes shell completion installation

### Homebrew Formula Template

```ruby
class Jari < Formula
  desc "Jira Cloud CLI for LLM coding agents"
  homepage "https://github.com/anomalyco/jari"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/anomalyco/jari/releases/download/v0.1.0/jari-aarch64-apple-darwin.tar.gz"
      sha256 "..."
    end
    on_intel do
      url "https://github.com/anomalyco/jari/releases/download/v0.1.0/jari-x86_64-apple-darwin.tar.gz"
      sha256 "..."
    end
  end

  on_linux do
    url "https://github.com/anomalyco/jari/releases/download/v0.1.0/jari-x86_64-unknown-linux-musl.tar.gz"
    sha256 "..."
  end

  def install
    bin.install "jari"
    # Install completions
    bash_completion.install "completions/jari.bash"
    zsh_completion.install "completions/jari.zsh"
    fish_completion.install "completions/jari.fish"
  end

  test do
    system "#{bin}/jari", "--version"
  end
end
```

### crates.io Publication

- Package name: `jari`
- Metadata in `Cargo.toml`:
  - `description`: informative one-liner
  - `repository`: GitHub URL
  - `license`: MIT or Apache-2.0
  - `keywords`: `["jira", "cli", "llm", "ai", "agent"]`
  - `categories`: `["command-line-utilities", "development-tools"]`
- `README.md` included in crate
- `LICENSE` file present
- `cargo publish --dry-run` passes before actual publish
- Publish as part of release pipeline, after CI passes

### Requirements

- `brew install jari` works end-to-end
- `cargo install jari` works end-to-end
- Version in formula matches git tag
- SHA256 checksums match actual release artifacts
