# video

**Kind:** library
**Depends on:** `protocol` (later: `gstreamer`, `gstreamer-app`, `nix`)
**Used by:** `service` (with feature `decode`), `desktop` (reader only)

## Purpose
Gets camera frames from the camera into shared memory once, so the desktop viewer, analysis and VR can all read them without copying.

## Modules
| Module | Contents |
|---|---|
| `decode` (feature `decode`) | GStreamer pipeline: RTP / UVC → raw frames, hardware decode |
| `ring` | memfd ring buffer layout: slots, `FrameHeader`, latest-frame index |
| `writer` | Ring writer used by the core's video ingest |
| `reader` | Ring reader used by the desktop's surgical viewer |

## Rules
- The desktop builds without GStreamer: decoding sits behind the `decode` feature.
- Readers always take the newest frame and never block the writer.
- Target under ~100 ms from camera to display.
