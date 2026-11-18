# vision

**Kind:** library
**Depends on:** `protocol` (later: `opencv`, `ort`, `nalgebra`, `rustfft`)
**Used by:** `service` (feature `vision`)

## Purpose
All computer-vision algorithms: camera and hand-eye calibration, and the camera-mode analysis that feeds VR overlays.

## Modules
| Module | Contents |
|---|---|
| `calibration::board` | ChArUco / ArUco board detection |
| `calibration::intrinsics` | Camera intrinsics, stereo rectification |
| `calibration::hand_eye` | Hand-eye solve: camera ↔ robot transform |
| `analysis::tool_tracking` | Tool / needle tip tracking (2D, stereo 3D) |
| `analysis::segmentation` | ML instrument and tissue segmentation (ONNX) |
| `analysis::tremor` | Tremor spectrum (FFT) from tip motion |
| `analysis::distance` | Distance from tool tip to targets and safety zones |

## Rules
- Pure algorithms: frames and poses in, results out. No sockets, no threads of its own.
- The workflow (when to capture, accept or cancel) lives in `service::calibration`, not here.
