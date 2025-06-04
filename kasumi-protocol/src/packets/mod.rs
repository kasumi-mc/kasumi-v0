//! Contains definitions, as well as implementations for different packets
//! inside the Minecraft protocol.

pub(crate) mod macros;

pub mod handshake;

/// Represents the protocol version that is currently implemented by this crate.
pub const PROTOCOL_VERSION: u16 = 770;
