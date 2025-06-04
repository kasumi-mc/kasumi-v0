use std::any::Any;

use thiserror::Error;

use crate::{
    protocol::{ReadError, WriteError, Writeable},
    varint::{VarInt, VarIntError},
};

pub mod configuration;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

/// Errors that can occur while decoding the packet.
#[derive(Debug, Error)]
pub enum PacketDecodeError {
    /// Indicates that something went wrong while parsing `VarInt`s.
    #[error(transparent)]
    VarIntError(#[from] VarIntError),
    /// Indicates that something went wrong while performing basic reading.
    #[error(transparent)]
    ReadError(#[from] ReadError),
    /// Indicates that the received enum variant is unknown to us.
    #[error("unknown enum variant ID: {0}")]
    UnknownEnumVariant(i32),
    /// Indicates that the received packet ID is unknown to us.
    #[error("unknown packet ID: {0}")]
    UnknownPacketId(i32),
}

/// Errors that can occur while encoding the packet.
#[derive(Debug, Error)]
pub enum PacketEncodeError {
    /// Indicates that the packet is not encodable (i.e. it can be send only
    /// in one direction).
    #[error("this packet is not encodable")]
    NotEncodable,
    /// Indicates that something went wrong while writing `VarInt`s.
    #[error(transparent)]
    VarIntError(#[from] VarIntError),
    /// Indicates that something went wrong while performing writing.
    #[error(transparent)]
    WriteError(#[from] WriteError),
}

/// Set of methods that each packet should implement to be treated as a packet.
pub trait Packet: Writeable + Send + Sync {
    /// ID of the this packet as `VarInt` value.
    fn id(&self) -> VarInt;
    /// Converts this packet into `Any` type. Useful for casting the trait to
    /// an actual packet struct.
    fn as_any(&self) -> &dyn Any;
}
/// The adapter for the `PacketsRegistry` that converts the provided packet
/// decoder function into a registry-compatible function.
pub fn registry_adapter<P: Packet + 'static>(
    buffer: &[u8],
    read_fn: fn(&[u8]) -> Result<(P, usize), ReadError>,
) -> Result<(Box<dyn Packet>, usize), ReadError> {
    let (packet, len) = read_fn(buffer)?;
    Ok((Box::new(packet), len))
}
