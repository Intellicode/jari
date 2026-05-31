# Spec 057: Integration Tests — CLI Arg Parsing

- [ ] Not implemented

## Goal

Tests verifying that all CLI subcommands, arguments, and flags parse correctly with proper defaults.

## Requirements

### Test Scenarios

**All subcommands parse:**
- Test each subcommand with minimal required args parses without error
- Test each subcommand with all optional flags parses without error

**Required args detection:**
- `issue get` without KEY → clap error
- `issue create` without `--project` → clap error
- `issue create` without `--summary` → clap error
- `search` without JQL → clap error
- `comment add` without KEY and BODY → clap error

**Default values:**
- `--output` defaults to `"json"`
- `--type` defaults to `"Task"` (issue create)
- `--fields` defaults to specific set (search)
- `--max` has no default (means "all")

**Short flags:**
- `-v` → `--verbose`
- `-p PROJ` → `--project PROJ`
- `-s "Title"` → `--summary "Title"`
- `-P High` → `--priority High`
- Test all short flags map correctly

**`@file.md` syntax:**
- `--description @testfile.md` parsed as file reference
- `--description "Just text"` parsed as inline text
- No `@` prefix → inline text

**`--output` validation:**
- `json` → valid
- `json-pretty` → valid
- `json-schema` → valid
- `xml` → clap error (invalid variant)

### Test Approach

- Use `clap::Command::try_get_matches_from()` for programmatic parsing tests
- Test from `Vec<&str>` simulating argv
- No network calls needed — pure arg parsing
