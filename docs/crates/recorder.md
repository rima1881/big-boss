# recorder

**Kind:** library
**Depends on:** `protocol` (later: `mcap`, `rtrb`)
**Used by:** `service` (feature `recorder`), `tools`

## Purpose
Records every packet with its arrival time, and replays logs at the original timing or another speed. This is how bugs get reproduced and how VR replay mode works.

## Modules
| Module | Contents |
|---|---|
| `format` | Channels and schemas used in log files |
| `writer` | Batched MCAP writer fed by a ring buffer |
| `reader` | Log reader and indexing |
| `replay` | Replay at original timing or a chosen speed |

## Rules
- Record raw packets, not decoded structs, so old logs survive type changes.
- The writer never blocks the sender: packets arrive through a lock-free queue.
