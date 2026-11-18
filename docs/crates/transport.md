# transport

**Kind:** library
**Depends on:** `protocol` (later: `tokio`, `tokio-seqpacket`, `serialport`, `nix`)
**Used by:** `service`, `desktop`, `tools`

## Purpose
Moves bytes. Knows how to open sockets and serial ports, frame packets and check their integrity, but not what the packets mean.

## Modules
| Module | Contents |
|---|---|
| `framing::cobs` | COBS encode/decode for UART byte streams |
| `framing::crc` | CRC-16 over header + payload |
| `udp` | UDP sockets for device links and VR output |
| `serial` | UART / RS-485 links |
| `unix::seqpacket` | Control channel (link 5) |
| `unix::dgram` | Telemetry channel (link 6) |
| `unix::fd_passing` | `SCM_RIGHTS` to hand the video memfd to the desktop |
| `tcp` | Length-prefixed TCP fallback for remote debugging |
| `control` | `ControlTransport` trait, implemented by seqpacket and TCP |

## Rules
- No business logic and no knowledge of the supervisor or UI.
- The control channel is selected by config (`unix` or `tcp`) through the `ControlTransport` trait.
- Keep UDP payloads under ~1400 bytes to avoid IP fragmentation.
