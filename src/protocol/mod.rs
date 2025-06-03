use std::str::Utf8Error;

use bytes::{Bytes, BytesMut};
use thiserror::Error;
use uuid::Uuid;

use crate::varint::{VarInt, VarIntError};

pub mod identifier;
pub mod macros;
pub mod packets;
pub mod server_list_ping;

pub mod handlers;
pub mod registry;
pub mod text;

/// Represents the current state of the protocol for a connection.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

/// Maximum length of a string as per Minecraft protocol.
const MAX_STRING_LENGTH: u32 = 32767;

/// Errors that can occur while reading from a protocol.
#[derive(Debug, Error)]
pub enum ReadError {
    /// Indicates that something went wrong while reading `VarInt`s.
    #[error(transparent)]
    VarIntError(#[from] VarIntError),
    /// Indicates that something went wrong while serializing JSON.
    #[error("failed to serialize the JSON: {0}")]
    JsonSerializationError(#[from] serde_json::Error),
    /// Indicates that the UUID parsing process has failed.
    #[error("failed to read UUID: {0}")]
    UuidError(#[from] uuid::Error),

    /// Indicates that the received buffer isn't enough for reading the whole
    /// value of a type.
    #[error("there weren't enough bytes to read the whole value")]
    Incomplete,
    /// Indicates that there were too many bytes provided for this type to be
    /// read.
    #[error("there were too many bytes in the buffer")]
    TooManyBytes,
    /// Indicates that the provided buffer is malformed for this type.
    #[error("the provided buffer is malformed")]
    MalformedBuffer,

    /// Represents an error occurred while converting raw read bytes into the
    /// string.
    #[error(transparent)]
    StringError(#[from] Utf8Error),
}

/// Represents a prefixed array. The simplest container out there.
#[derive(Debug, Clone)]
pub struct PrefixedArray<T: Readable + Writeable>(pub Vec<T>);

/// Implementation of a generic types for a Minecraft protocol as per
/// specification.
pub trait Readable: Sized {
    /// Reads from the provided buffer into the implementing type as per
    /// protocol specification. Returns the value in the type, as well as
    /// amount of read bytes.
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError>;
}

impl Readable for String {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        let mut read_bytes: usize = 0;

        let (string_length, read_len) = VarInt::read(buffer)?;
        read_bytes += read_len;

        // string's length cannot be less than 0
        if string_length.0 < 0 {
            return Err(ReadError::MalformedBuffer);
        }

        let string_length = string_length.0 as usize;
        if string_length > MAX_STRING_LENGTH as usize {
            return Err(ReadError::TooManyBytes);
        }

        if buffer.len() < read_bytes + string_length {
            return Err(ReadError::Incomplete);
        }

        let string = str::from_utf8(&buffer[read_bytes..(read_bytes + string_length)])?;
        read_bytes += string_length;

        Ok((string.to_owned(), read_bytes))
    }
}

impl Readable for u16 {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        if buffer.len() < 2 {
            return Err(ReadError::Incomplete);
        }
        let value = u16::from_be_bytes([buffer[0], buffer[1]]);
        Ok((value, 2))
    }
}

impl Readable for i64 {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        let i64_buffer = buffer.get(..8).ok_or(ReadError::Incomplete)?;
        let array: [u8; 8] = i64_buffer.try_into().unwrap(); // safe: `i64_buffer` is always 8 bytes
        Ok((i64::from_be_bytes(array), 8))
    }
}

impl Readable for Uuid {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        let uuid_buffer = buffer.get(..16).ok_or(ReadError::Incomplete)?;
        Ok((Uuid::from_slice(uuid_buffer)?, 16))
    }
}

impl<T: Readable + Writeable> Readable for PrefixedArray<T> {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        let mut total_length = 0;

        let (length, read_length) = VarInt::read(buffer)?;
        total_length += read_length;

        if length.0 < 0 {
            return Err(ReadError::MalformedBuffer);
        }

        if buffer.len() < length.0 as usize {
            return Err(ReadError::MalformedBuffer);
        }

        let mut elements = Vec::with_capacity(length.0 as usize);
        for _ in 0..length.0 {
            let (element, read_len) = T::read(&buffer[total_length..])?;
            elements.push(element);
            total_length += read_len;
        }

        Ok((PrefixedArray(elements), total_length))
    }
}

impl Readable for u8 {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        Ok((buffer[0], 1))
    }
}

impl Readable for bool {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        Ok((buffer[0] == 0x01, 1))
    }
}

impl Readable for i8 {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        Ok((buffer[0] as i8, 1))
    }
}

impl Readable for Vec<u8> {
    fn read(buffer: &[u8]) -> Result<(Self, usize), ReadError> {
        Ok((Vec::from(buffer), buffer.len()))
    }
}

/// Errors that can occur while writing writing to a buffer.
#[derive(Debug, Error)]
pub enum WriteError {
    /// Indicates that something went wrong while writing `VarInt`s.
    #[error(transparent)]
    VarIntError(#[from] VarIntError),
    /// Indicates that something went wrong while deserializing JSON.
    #[error("failed to deserialize JSON: {0}")]
    JsonDeserializationError(#[from] serde_json::Error),
}

/// Implementation of a generic types that can be written over the wire per
/// Minecraft protocol specification.
pub trait Writeable: Send + Sync {
    /// Writes self value into the `Bytes` instance as per Minecraft protocol
    /// specification.
    fn write(&self) -> Result<Bytes, WriteError>;
}

impl Writeable for String {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::with_capacity(5 + self.len()); // VarInt can take up at most 5 bytes
        buffer.extend_from_slice(&VarInt(self.len() as i32).write()?);
        buffer.extend_from_slice(self.as_bytes());
        Ok(buffer.freeze())
    }
}

impl Writeable for u16 {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::with_capacity(2);
        buffer.extend_from_slice(&self.to_be_bytes());
        Ok(buffer.freeze())
    }
}

impl Writeable for i64 {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::with_capacity(8);
        buffer.extend_from_slice(&self.to_be_bytes());
        Ok(buffer.freeze())
    }
}

impl Writeable for Uuid {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::with_capacity(16);
        buffer.extend_from_slice(self.as_bytes());
        Ok(buffer.freeze())
    }
}

impl<T: Readable + Writeable> Writeable for PrefixedArray<T> {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&VarInt(self.0.len() as i32).write()?);

        for element in self.0.iter() {
            let element_buffer = element.write()?;
            buffer.extend_from_slice(&element_buffer);
        }

        Ok(buffer.freeze())
    }
}

impl Writeable for u8 {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::with_capacity(1);
        buffer.extend_from_slice(&[*self]);
        Ok(buffer.freeze())
    }
}

impl Writeable for bool {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::with_capacity(1);
        buffer.extend_from_slice(&[if *self { 0x01 } else { 0x00 }]);
        Ok(buffer.freeze())
    }
}

impl Writeable for i8 {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::with_capacity(1);
        buffer.extend_from_slice(&[*self as u8]);
        Ok(buffer.freeze())
    }
}

impl Writeable for Vec<u8> {
    fn write(&self) -> Result<Bytes, WriteError> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(self);
        Ok(buffer.freeze())
    }
}
