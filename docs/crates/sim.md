# sim

**Kind:** library
**Depends on:** `protocol` (later: `rapier3d` or MuJoCo bindings, `urdf-rs`)
**Used by:** `service` (feature `sim`)

## Purpose
A stand-in for the real manipulator. It takes `JointCmd` and answers with `RobotState`, exactly like the hardware, so the processing system can't tell the difference. It also runs training scenarios and scores them.

## Modules
| Module | Contents |
|---|---|
| `robot` | `SimRobot`: kinematic model loaded from URDF |
| `scenario` | Scenario files (TOML / RON): start pose, objects, targets |
| `physics` | Physics world stepping |
| `faults` | Injected faults: latency, packet loss, tremor, sensor noise |
| `scoring` | `SimScore`: target error, time, path length, tremor |

## Rules
- Speaks the same wire protocol as the real robot (sim port on link 2).
- Runs headless, so scenarios work in automated tests.
- No rendering: drawing the scene is the VR app's job, using `SimScene` from `VrFrame`.
