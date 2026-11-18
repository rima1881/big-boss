//! Core service: owns hardware links, state and safety. Runs without a GUI.

pub mod config;
pub mod bus;
pub mod supervisor;
pub mod links;
pub mod control_server;
pub mod telemetry;
pub mod vr_output;
pub mod calibration;
#[cfg(feature = "sim")]
pub mod simulator;
#[cfg(feature = "video")]
pub mod video_ingest;
#[cfg(feature = "vision")]
pub mod analysis;
#[cfg(feature = "recorder")]
pub mod logger;
