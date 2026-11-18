# protocol

**Kind:** library, `no_std` capable (`default-features = false` for firmware)
**Depends on:** nothing internal (later: `serde`, `postcard`, `zerocopy`)
**Used by:** every other crate, every app, firmware

## Purpose
The single source of truth for every byte that crosses a process or device boundary. If a struct is on the wire, it is defined here and nowhere else.

## Modules
| Module | Contents |
|---|---|
| `msg_type` | msg_type code constants, grouped by range |
| `header` | `PktHeader` |
| `enums` | `Mode`, `SafetyState`, `SimAction`, `CalibAction`, `LogAction`, `EStopSource`, `ObjectKind` |
| `tracker` | `Pose`, `EngageEvent` |
| `processing` | `JointCmd`, `FilteredPose`, `ProcConfig`, `CalibTransform` |
| `robot` | `RobotState` |
| `safety` | `EStop`, `Heartbeat` |
| `control` | `Request`, `Command`, `Reply`, `Event` |
| `telemetry` | `Telemetry` |
| `video` | `FrameHeader` |
| `vr` | `VrPacket`, `VrCalib`, `VrFrame`, `SimScene`, `SceneObject`, `Contact`, `SimScore`, `AnalysisResult` |

## Rules
- Types and encoding only: no sockets, threads, allocation-heavy logic or I/O.
- Device packets (links 1–3) are fixed-size packed structs; desktop and VR messages use postcard.
- Bump `PktHeader.version` on any breaking layout change.
- Every type gets an encode → decode round-trip test.
