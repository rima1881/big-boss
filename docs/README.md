# big-boss docs

| Doc | What it covers |
|---|---|
| [architecture.md](architecture.md) | The core service data map (the whiteboard): subsystems, links, messages, data types, enums, VR inputs |
| [crates/](crates/) | One page per library crate: purpose, modules, dependencies, rules |
| [apps/](apps/) | One page per binary: core-service, desktop, tools |

Live whiteboard: https://claude.ai/artifact/63oVAYJcxERLT3pBpSZNbc
`architecture.md` is a snapshot of it. When one changes, update the other.

## Crates and apps at a glance

| Path | Kind | One line |
|---|---|---|
| [crates/protocol](crates/protocol.md) | lib, no_std | Every wire data type |
| [crates/transport](crates/transport.md) | lib | UDP, UART, Unix sockets, framing |
| [crates/video](crates/video.md) | lib | Camera decode and the memfd frame ring |
| [crates/sim](crates/sim.md) | lib | SimRobot, scenarios, physics, scoring |
| [crates/vision](crates/vision.md) | lib | Hand-eye calibration and camera-mode analysis |
| [crates/recorder](crates/recorder.md) | lib | Packet logging and replay |
| [crates/service](crates/service.md) | lib | The core service logic |
| [apps/core-service](apps/core-service.md) | bin | Headless core service |
| [apps/desktop](apps/desktop.md) | bin | Slint desktop client |
| [apps/tools](apps/tools.md) | bins | Inspector, replay, fake devices |
| firmware/ | separate workspace | MCU firmware sharing `protocol` |

## Dependency rules

```
protocol  ← everyone
transport ← service, desktop, tools
video     ← service (writer + decode), desktop (reader)
sim, vision ← service only
recorder  ← service, tools
service   ← core-service only
desktop   → protocol + transport + video   (never service)
```
