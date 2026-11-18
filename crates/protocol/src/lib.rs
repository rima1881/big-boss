//! Wire data types shared by every program. Builds without std for firmware.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod msg_type;
pub mod header;
pub mod enums;
pub mod tracker;
pub mod processing;
pub mod robot;
pub mod safety;
pub mod control;
pub mod telemetry;
pub mod video;
pub mod vr;
