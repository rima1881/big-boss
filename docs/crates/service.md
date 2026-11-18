# service

**Kind:** library (the core service logic)
**Depends on:** `protocol`, `transport`; optional `sim`, `vision`, `video`, `recorder` (features of the same names, all on by default)
**Used by:** `apps/core-service` only

## Purpose
The headless brain of the desktop system. Owns every hardware link, all system state and all safety logic. Runs with no GUI attached; the desktop and VR are clients.

Named `service` because a crate called `core` would clash with Rust's built-in `core` library.

## Modules
| Module | Contents |
|---|---|
| `config` | Loads `config/core.toml` |
| `bus` | Channels between subsystems; all events flow to the supervisor |
| `supervisor::state_machine` | Idle → Ready → Engaged → Paused → EStopped |
| `supervisor::watchdog` | Heartbeat timeouts for desktop, processing, manipulator |
| `links::tracker` | Link 1 |
| `links::processing` | Link 2 (and the sim port) |
| `links::manipulator` | Link 3 |
| `control_server` | Link 5: validates commands, replies Ack / Nack, pushes events |
| `telemetry` | Link 6 |
| `vr_output` | Link 8: builds `VrFrame` each tick, sends `VrPacket` / `VrCalib` |
| `calibration` | Calibration workflow; math in `vision` |
| `simulator` (feature `sim`) | Runs `sim::SimRobot` on the sim port |
| `video_ingest` (feature `video`) | Links 4 and 7 |
| `analysis` (feature `vision`) | Feeds frames to `vision::analysis`, results to `vr_output` |
| `logger` (feature `recorder`) | Copies every packet to `recorder` |

## Rules
- **Only the supervisor changes state.** Other modules send it events over `bus`.
- No module shares mutable state through mutexes; message passing only.
- The control loop stays in the processing system. The core watches, configures and logs; it is never in the joint-command path.
- If the desktop disconnects, pause (disengage and hold). The hardwired E-stop works regardless.
