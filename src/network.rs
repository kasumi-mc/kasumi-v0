use bytes::{Buf, BytesMut};
use thiserror::Error;

use crate::{
    protocol::{ReadError, Readable},
    varint::{VarInt, VarIntError},
};

/// Indicates the maximum allowed buffer size for a connection at a time. The
/// buffer clears itself up after each packet, so this amount should be plenty
/// for a connection.
pub const BUFFER_CAPACITY: usize = 4096;

/// Errors that can occur when working with `PacketReader`.
#[derive(Debug, Error)]
pub enum PacketReaderError {
    /// Indicates that something went wrong while reading.
    #[error(transparent)]
    ReadError(#[from] ReadError),
    /// Indicates that the packet is malformed, i.e. its length is less than 0.
    #[error("the length of the received packet is less than 0 ({0})")]
    MalformedPacket(i32),
}

/// Implementation for a efficient reader for a packets sent over the wire.
pub struct PacketReader {
    buffer: BytesMut,
}

impl Default for PacketReader {
    fn default() -> Self {
        Self::with_capacity(BUFFER_CAPACITY)
    }
}

impl PacketReader {
    /// Creates a new `PacketReader` with the provided capacity for the
    /// underlying buffer.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: BytesMut::with_capacity(capacity),
        }
    }

    /// Extends the underlying buffer from a provided slice.
    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data)
    }

    /// Clears the underlying buffer and resets the cursor position.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Tries to read packet from the underlying buffer. Returns packet's ID
    /// and its body, if the read was successful and there were enough data.
    pub fn try_next_packet(&mut self) -> Result<Option<(VarInt, BytesMut)>, PacketReaderError> {
        let (packet_len, read_len) = match VarInt::read(&self.buffer) {
            Ok(lengths) => lengths,
            Err(ReadError::VarIntError(VarIntError::Incomplete)) => return Ok(None),
            Err(e) => return Err(PacketReaderError::ReadError(e)),
        };

        // packet is malformed
        if packet_len.0 < 0 {
            return Err(PacketReaderError::MalformedPacket(packet_len.0));
        }

        // wait for more data to be received
        if self.buffer.len() < read_len + (packet_len.0 as usize) {
            return Ok(None);
        }

        // TODO: check for maximum possible packet size

        // strip the `read_len` (header length) from the packet body
        self.buffer.advance(read_len);

        let mut packet_body = self.buffer.split_to(packet_len.0 as usize);
        let (packet_id, read_len) = match VarInt::read(&packet_body) {
            Ok(lengths) => lengths,
            Err(ReadError::VarIntError(VarIntError::Incomplete)) => return Ok(None),
            Err(e) => return Err(PacketReaderError::ReadError(e)),
        };

        packet_body.advance(read_len);
        Ok(Some((packet_id, packet_body)))
    }
}

pub struct BufferReader<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> BufferReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, offset: 0 }
    }

    pub fn read<T, F, E>(&mut self, reader: F) -> Result<T, E>
    where
        F: Fn(&[u8]) -> Result<(T, usize), E>,
    {
        let (value, consumed) = reader(&self.buffer[self.offset..])?;
        self.offset += consumed;
        Ok(value)
    }

    pub fn consumed(&self) -> usize {
        self.offset
    }
}
