//! Video: decode (core) and memfd frame ring (writer in core, reader in desktop).

#[cfg(feature = "decode")]
pub mod decode;
pub mod ring;
pub mod writer;
pub mod reader;
