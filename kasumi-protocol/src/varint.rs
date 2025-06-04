//! Contains the representation and implementation for a VarInt.

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{ReadError, Readable, Writable, WriteError};

/// The maximum number of bytes allowed when parsing a VarInt, as defined by
/// the Minecraft protocol. VarInts are at most 5 bytes (35 bits) long.
const MAXIMUM_SIZE: usize = 5;

/// Bitmask used to extract the lower 7 data bits from each VarInt byte segment.
/// In VarInt encoding, each byte uses the lower 7 bits for data and the
/// highest bit as a continuation flag.
const SEGMENT_BITS: u8 = 0x7F;

/// The bit flag in VarInt encoding indicating that more bytes follow. If this
/// bit is set in a VarInt byte, the next byte is also part of the value.
const CONTINUE_BIT: u8 = 0x80;

/// Representation of a single variable-length integer used by Minecraft
/// protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl Readable for VarInt {
    #[inline(always)]
    fn read(buffer: &mut Bytes) -> Result<Self, ReadError> {
        let mut shift = 0;
        let mut result: i32 = 0;

        for _ in 0..MAXIMUM_SIZE {
            if buffer.is_empty() {
                return Err(ReadError::TooShort(1, buffer.len()));
            }

            let current_byte = buffer.get_u8();
            result |= ((current_byte & SEGMENT_BITS) as i32) << shift;

            if (current_byte & CONTINUE_BIT) == 0 {
                return Ok(Self(result));
            }
            shift += 7;
        }

        Err(ReadError::Overflow(MAXIMUM_SIZE))
    }
}

impl Writable for VarInt {
    #[inline(always)]
    fn write(&self, buffer: &mut BytesMut) -> Result<(), WriteError> {
        let mut value = self.0 as u32;
        let mut bytes_written = 0;

        loop {
            if bytes_written >= MAXIMUM_SIZE {
                return Err(WriteError::TooBig);
            }

            let mut temp_value = (value as u8) & SEGMENT_BITS;
            value >>= 7;

            if value != 0 {
                temp_value |= CONTINUE_BIT;
            }

            buffer.put_u8(temp_value);
            bytes_written += 1;

            if value == 0 {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Bytes, BytesMut};

    /// Asserts that a `VarInt` value can be written to a buffer and then read
    /// back to produce the original value, verifying round-trip correctness of
    /// both encoding and decoding.
    ///
    /// Also checks that the encoded representation does not exceed the
    /// protocol's maximum allowed length, and that the buffer is fully
    /// consumed after reading.
    fn round_trip(value: i32) {
        let varint = VarInt(value);
        let mut buf = BytesMut::new();
        varint.write(&mut buf).unwrap();

        let mut bytes = buf.freeze();
        let parsed = VarInt::read(&mut bytes).unwrap();

        assert_eq!(parsed, varint, "Round-trip failed for value: {value}");
        assert!(bytes.is_empty(), "Buffer not fully consumed after read");
    }

    #[test]
    fn test_varint_zero() {
        round_trip(0);
    }

    #[test]
    fn test_varint_positive_values() {
        for &val in &[1, 127, 128, 255, 300, 16384, i32::MAX] {
            round_trip(val);
        }
    }

    #[test]
    fn test_varint_negative_values() {
        for &val in &[-1, -127, -128, -255, -300, -16384, i32::MIN] {
            round_trip(val);
        }
    }

    #[test]
    fn test_malformed_input_too_short() {
        // Simulate EOF in the middle of a VarInt
        let mut bytes = Bytes::from_static(&[0x81]);
        let err = VarInt::read(&mut bytes).unwrap_err();
        assert!(matches!(err, ReadError::TooShort(_, _)));
    }

    #[test]
    fn test_malformed_input_too_long() {
        // More than 5 bytes with all continuation bits set
        let mut bytes = Bytes::from_static(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]);
        let err = VarInt::read(&mut bytes).unwrap_err();
        assert!(matches!(err, ReadError::Overflow(_)));
    }
}
