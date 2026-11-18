# tools

**Kind:** several binaries (`src/bin/`)
**Depends on:** `protocol`, `transport`, `recorder`

## Purpose
Development and test helpers, so the whole system can run end to end before hardware exists.

| Binary | Does |
|---|---|
| `packet_inspector` | Decodes and prints live packets on any link |
| `replay` | Replays a recorded log into the core or VR |
| `fake_tracker` | Sends `Pose` and `EngageEvent` like the real tracker |
| `fake_robot` | Answers `JointCmd` with `RobotState` like the real manipulator |

## Rules
- Use the same `protocol` and `transport` code as production, so the tools test the real wire format.
