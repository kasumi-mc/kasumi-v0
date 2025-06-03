use thiserror::Error;

use crate::protocol::{Readable, Writeable};

/// The bitmask to extract the lower 7 bits of each VarInt byte. These bits
/// contain the actual encoded data.
const SEGMENT_BITS: u8 = 0x7F;

/// The bitmask that indicates that additional VarInt bytes follow.
const CONTINUE_BIT: u8 = 0x80;

/// The number to shift for each subsequent VarInt byte.
const SHIFT_VALUE: u32 = 7;

/// The maximum number of bytes a single VarInt can occupy in the buffer.
const VARINT_BITS_SIZE: usize = 5;

/// Errors that can occur when encoding or decoding a Minecraft protocol VarInt.
#[derive(Error, Debug)]
pub enum VarIntError {
    /// The VarInt is too long - more than the allowed number of bytes were
    /// read before completion.
    #[error("the received VarInt is too big")]
    TooManyBytes,
    /// The VarInt is incomplete - there weren't enough bytes in the buffer to
    /// finish decoding.
    #[error("the received VarInt is incomplete")]
    Incomplete,
}

/// Reads Minecraft protocol VarInt from a provided buffer slice. It returns
/// the read VarInt and the number of read bytes.
fn read_varint(buffer: &[u8]) -> Result<(i32, usize), VarIntError> {
    let mut result: i32 = 0;
    let mut shift = 0;
    let mut read_bytes = 0;

    for byte in buffer.iter().take(VARINT_BITS_SIZE) {
        let value = (byte & SEGMENT_BITS) as i32;

        let shifted = value.checked_shl(shift).ok_or(VarIntError::TooManyBytes)?;
        result = result
            .checked_add(shifted)
            .ok_or(VarIntError::TooManyBytes)?;

        read_bytes += 1;
        if (byte & CONTINUE_BIT) == 0 {
            return Ok((result, read_bytes));
        }
        shift += SHIFT_VALUE;
    }

    if buffer.len() < VARINT_BITS_SIZE {
        Err(VarIntError::Incomplete)
    } else {
        Err(VarIntError::TooManyBytes)
    }
}

/// Writes the provided number as a Minecraft protocol VarInt. Returns the
/// buffer and the number of bytes written.
fn write_varint(value: i32) -> Result<([u8; VARINT_BITS_SIZE], usize), VarIntError> {
    let mut buffer = [0u8; VARINT_BITS_SIZE];
    let mut value = value;
    let mut i = 0;

    loop {
        let mut temp = (value as u8) & SEGMENT_BITS;
        value = ((value as u32) >> SHIFT_VALUE) as i32;

        if value != 0 {
            temp |= CONTINUE_BIT;
        }

        buffer[i] = temp;
        i += 1;

        if value == 0 {
            break;
        }
    }

    Ok((buffer, i))
}

/// Representation of a `VarInt` - the smallest possible number in Minecraft
/// protocol.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct VarInt(pub i32);

impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Readable for VarInt {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        let (value, read_length) = read_varint(buffer)?;
        Ok((Self(value), read_length))
    }
}

impl Writeable for VarInt {
    fn write(&self) -> Result<bytes::Bytes, crate::protocol::WriteError> {
        let (buffer, written_bytes) = write_varint(self.0)?;
        Ok(bytes::Bytes::copy_from_slice(&buffer[..written_bytes]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_zero() {
        let (buf, len) = write_varint(0).unwrap();
        assert_eq!(&buf[..len], &[0x00]);
        assert_eq!(read_varint(&buf[..len]).unwrap().0, 0);
    }

    #[test]
    fn test_varint_positive() {
        let (buf, len) = write_varint(300).unwrap();
        assert_eq!(&buf[..len], &[0xAC, 0x02]);
        assert_eq!(read_varint(&buf[..len]).unwrap().0, 300);
    }

    #[test]
    fn test_varint_negative() {
        let (buf, len) = write_varint(-1).unwrap();
        assert_eq!(&buf[..len], &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
        assert_eq!(read_varint(&buf[..len]).unwrap().0, -1);
    }

    #[test]
    fn test_varint_too_many_bytes() {
        // 6 bytes, all with CONTINUE_BIT set
        let buf = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80];
        assert!(matches!(read_varint(&buf), Err(VarIntError::TooManyBytes)));
    }

    #[test]
    fn test_varint_incomplete() {
        // Not enough bytes to complete the VarInt (stops after 2)
        let buf = [0x80, 0x80];
        assert!(matches!(read_varint(&buf), Err(VarIntError::Incomplete)));
    }
}
