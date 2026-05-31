# Spec 039: CLI — Transition List & Do

- [ ] Not implemented

## Goal

Wire the `transition list` and `transition do` subcommands.

## Requirements

### transition list

```
jari transition list <KEY>
```

- Call `client.list_transitions(key)`
- Output: array of `Transition` with `id`, `name`, `to.status_category.name`

### transition do

```
jari transition do <KEY> <TRANSITION> [--comment <TEXT>] [--resolution <R>]
```

- `<TRANSITION>`: transition ID or name (supports fuzzy matching via client)
- `--comment <TEXT>`: optional comment to add during transition (markdown → ADF)
- `--resolution <R>`: optional resolution name (e.g., "Done", "Won't Do")
- Call `client.do_transition(key, transition, comment, resolution)`
- Output `TransitionResult`

### transition do Output Shape

```json
{
  "ok": true,
  "data": {
    "transition": "Done",
    "from_status": "In Progress",
    "to_status": "Done"
  },
  "meta": { "command": "jari transition do PROJ-123 Done", "duration_ms": 350 }
}
```

### Error Handling

- Issue not found: `NotFound`
- Invalid transition name: error with "No transition matching 'X'. Available: [list...]"
- Transition not allowed (workflow rules): `Validation` from Jira
- Missing required fields (e.g., resolution): `Validation` with details
