//! Implementations for all types used in the protocol definitions.

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{ReadError, Readable, Writable, WriteError, varint::VarInt};

/// Implements the `Readable` and `Writable` traits for a primitive integer type
/// using a corresponding `bytes::Buf` method.
///
/// This macro generates an implementation of the `Readable` and `Writable`
/// trait for the specified integer type, using the provided method from the
/// [`bytes::Buf`](https://docs.rs/bytes/latest/bytes/buf/trait.Buf.html) trait
/// to extract the value from the buffer.
///
/// It allows to reduce amount of code required for implementing
/// protocol-compatible primitives.
///
/// # Usage
///
/// ```rust
/// impl_integer_type!(u8, get_u8, put_u8);
/// impl_integer_type!(i16, get_i16, put_i16);
/// ```
#[macro_export]
macro_rules! impl_integer_type {
    ($type:ty, $read_function:ident, $write_function:ident) => {
        impl $crate::Readable for $type {
            fn read(buffer: &mut bytes::Bytes) -> std::result::Result<Self, $crate::ReadError> {
                use bytes::Buf;
                if buffer.remaining() < std::mem::size_of::<$type>() {
                    return Err($crate::ReadError::TooShort(
                        std::mem::size_of::<$type>(),
                        buffer.remaining(),
                    ));
                }
                Ok(buffer.$read_function())
            }
        }

        impl $crate::Writable for $type {
            fn write(
                &self,
                buffer: &mut bytes::BytesMut,
            ) -> std::result::Result<(), $crate::WriteError> {
                buffer.$write_function(*self);
                Ok(())
            }
        }
    };
}

/// Represents the maximum possible size for a protocol-compatible string.
const MAXIMUM_STRING_SIZE: usize = 32767;

impl Readable for String {
    fn read(buffer: &mut Bytes) -> Result<Self, ReadError> {
        let length = VarInt::read(buffer)?.0 as usize;

        if length > MAXIMUM_STRING_SIZE {
            return Err(ReadError::Overflow(MAXIMUM_STRING_SIZE));
        }

        if buffer.remaining() < length {
            return Err(ReadError::TooShort(length, buffer.remaining()));
        }

        let string_buffer = buffer.split_to(length);
        let string = std::str::from_utf8(&string_buffer)?;
        Ok(string.to_owned())
    }
}

impl Writable for String {
    fn write(&self, buffer: &mut BytesMut) -> Result<(), WriteError> {
        let string_buffer = self.as_bytes();
        VarInt(string_buffer.len() as i32).write(buffer)?;
        buffer.extend_from_slice(string_buffer);
        Ok(())
    }
}

impl_integer_type!(u16, get_u16, put_u16);
