# big-boss

Teleoperated surgical robot: core service + Slint desktop.

Design docs: [docs/](docs/README.md) (architecture data map and one page per crate).

| Path | Kind | Role |
|---|---|---|
| crates/protocol | lib (no_std) | Every wire data type: packets, control, telemetry, VR |
| crates/transport | lib | UDP, UART, Unix sockets, COBS + CRC framing |
| crates/video | lib | GStreamer decode, memfd ring (writer + reader) |
| crates/sim | lib | SimRobot, scenarios, physics, scoring |
| crates/vision | lib | Hand-eye calibration, camera-mode analysis |
| crates/recorder | lib | MCAP logging and replay |
| crates/service | lib | Core service logic (supervisor, links, servers, VR output) |
| apps/core-service | bin | Headless core service |
| apps/desktop | bin | Slint desktop client |
| apps/tools | bins | Packet inspector, replay CLI, fake tracker, fake robot |
| firmware | separate workspace | MCU firmware, shares crates/protocol |

Dependency rules:
- `protocol` depends on nothing internal.
- `desktop` uses protocol + transport + video, never `service`.
- `sim`, `vision`, `recorder` are used by `service` (and tools); heavy ones are optional features.
