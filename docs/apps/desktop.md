# desktop

**Kind:** binary (Slint)
**Depends on:** `protocol`, `transport`, `video` (reader only). Never `service`.

## Purpose
The operator's control panel. Shows what the core reports and sends requests. Holds no authority: closing or crashing it never affects the robot.

## Modules
| Module | Contents |
|---|---|
| `client` | Link 5: requests out, Ack / Nack and events in, heartbeat |
| `telemetry_rx` | Link 6: keeps only the latest snapshot |
| `video_reader` | Link 7: newest frame from the memfd ring → `slint::Image` |
| `app_state` | View models and pending requests; the UI reads only from here |

## UI (`ui/`)
`app.slint` plus views: `status_estop`, `scaling`, `sim_panel`, `calibrator`, `logger_panel`, `surgical_viewer`.

## Rules
- Views never touch sockets. They read `app_state` and fire callbacks.
- No optimistic updates: a value changes on screen only after the core acks it or telemetry shows it.
