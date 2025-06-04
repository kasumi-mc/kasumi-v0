//! # Kasumi Protocol
//!
//! Crate that defines and implements the Minecraft protocol.

use bytes::{Bytes, BytesMut};
use thiserror::Error;

pub mod packets;
pub mod state;
pub mod types;
pub mod varint;

/// Errors that can occur during reading operations.
#[non_exhaustive]
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ReadError {
    /// Indicates that the received buffer is empty, but the reader wanted to
    /// get more out of it.
    #[error("wanted to read {0} bytes, but got only {1}")]
    TooShort(usize, usize),
    /// Indicates that the reader tried to parse more than allowed by the type.
    #[error("attempted to parse a value larger than {0} bytes")]
    Overflow(usize),
    /// Indicates that the received string is invalid UTF-8 and cannot be parsed.
    #[error("invalid UTF-8 string: {0}")]
    InvalidString(#[from] std::str::Utf8Error),
    /// Indicates that the received enum variant is unknown to protocol parser.
    #[error("unknown enum variant: {0}")]
    UnknownVariant(i32),
}

/// Errors that can occur during writing operations.
#[non_exhaustive]
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WriteError {
    /// Indicates that the writer tried deserializing value that is too big.
    #[error("value exceeds the maximum allowed size")]
    TooBig,
}

/// Defines the set of methods to read the type from the provided buffer.
pub trait Readable: Sized {
    /// Reads the type from the provided buffer, while also advancing the
    /// current position of the internal cursor.
    fn read(buffer: &mut Bytes) -> Result<Self, ReadError>;
}

/// Defines the set of methods to write the type to the provided buffer.
pub trait Writable: Sized {
    /// Writes the type into the provided buffer.
    fn write(&self, buffer: &mut BytesMut) -> Result<(), WriteError>;
}
