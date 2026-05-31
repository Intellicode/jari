# Spec 049: Shell Completions

- [ ] Not implemented

## Goal

Generate and ship shell completion scripts for bash, zsh, and fish.

## Requirements

### CLI Interface

```
jari completions <SHELL>
```

- `<SHELL>`: `bash`, `zsh`, or `fish`
- Uses `clap_complete` to generate script
- Output to stdout

### Pre-built Files

- Generate completion files at build time and commit to `completions/` directory:
  - `completions/jari.bash`
  - `completions/jari.zsh`
  - `completions/jari.fish`

### Installation Instructions (README)

- **bash**: `source <(jari completions bash)` or copy to `/etc/bash_completion.d/`
- **zsh**: `source <(jari completions zsh)` or copy to `/usr/local/share/zsh/site-functions/`
- **fish**: `jari completions fish | source` or copy to `~/.config/fish/completions/`

### Requirements

- Generated via `clap_complete` crate at compile time
- Pre-built files shipped in the repository
- Homebrew formula installs completions automatically
- Shell completions include all subcommands, flags, and options
