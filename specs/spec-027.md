# Spec 027: CLI — Project List & Get

- [ ] Not implemented

## Goal

Wire the `project list` and `project get` subcommands.

## Requirements

### project list

```
jari project list [--type <TYPE>]
```

- `--type <TYPE>`: filter by `software`, `service_desk`, `business`
- Calls `client.list_projects(type_filter)`
- Output: array of `Project`

### project get

```
jari project get <KEY>
```

- `<KEY>`: positional, required (project key)
- Calls `client.get_project(key)`
- Output: single `Project` with full details (lead, versions, components, issue types)

### Output Shape

List:
```json
{
  "ok": true,
  "data": [
    { "key": "PROJ", "name": "My Project", ... },
    ...
  ],
  "meta": { "command": "jari project list", "duration_ms": 200 }
}
```

### Error Handling

- Project not found: `NotFound`
- No projects: empty array (not an error)
