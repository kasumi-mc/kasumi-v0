#[macro_export]
macro_rules! register_packet {
    ($registry: expr, $packet: ty) => {
        $registry.register(
            <$packet>::PACKET_STATE,
            <$packet>::PACKET_ID,
            Box::new(|buffer| {
                $crate::packets::registry_adapter::<$packet>(
                    buffer,
                    <$packet as $crate::protocol::Readable>::read,
                )
            }),
        )
    };
}

#[macro_export]
macro_rules! define_packet {
    ($name: ident, $id: expr, $state: ident, { $($field: ident : $type: ty$(,)?)* }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $field: $type,)*
        }

        impl $name {
            /// ID of this packet. May not be unique between multiple protocol
            /// states.
            pub const PACKET_ID: crate::varint::VarInt = crate::varint::VarInt($id);

            /// State that this packet is designed for.
            pub const PACKET_STATE: $crate::protocol::ProtocolState = $crate::protocol::ProtocolState::$state;
        }

        impl $crate::protocol::Readable for $name {
            fn read(_buffer: &[u8]) -> std::result::Result<(Self, usize), $crate::protocol::ReadError> {
                let mut total_read_length: usize = 0;
                $(
                    let ($field, read_length) = <$type as $crate::protocol::Readable>::read(&_buffer[total_read_length..])?;
                    total_read_length += read_length;
                )*
                Ok((Self { $($field,)* }, total_read_length))
            }
        }

        impl $crate::protocol::Writeable for $name {
            fn write(&self) -> std::result::Result<bytes::Bytes, $crate::protocol::WriteError> {
                let mut buffer = bytes::BytesMut::new(); // TODO: detect the maximum allocation size
                $(
                    buffer.extend_from_slice(&self.$field.write()?);
                )*
                Ok(buffer.freeze())
            }
        }

        impl $crate::protocol::packets::Packet for $name {
            fn id(&self) -> crate::varint::VarInt {
                crate::varint::VarInt($id)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

#[macro_export]
macro_rules! define_varint_enum {
    ($name: ident, { $($variant_name: ident = $variant_value: expr$(,)?)* }) => {
        #[repr(i32)]
        #[derive(Debug, Clone)]
        pub enum $name {
            $($variant_name = $variant_value,)*
        }

        impl $crate::protocol::Readable for $name {
            fn read(_buffer: &[u8]) -> std::result::Result<(Self, usize), $crate::protocol::ReadError> {
                let (value, value_length) = crate::varint::VarInt::read(_buffer)?;
                let variant = match value.0 {
                    $(
                        $variant_value => $name::$variant_name,
                    )*
                    _ => return Err($crate::protocol::ReadError::MalformedBuffer) // TODO: better error type
                };
                Ok((variant, value_length))
            }
        }

        impl $crate::protocol::Writeable for $name {
            fn write(&self) -> std::result::Result<bytes::Bytes, $crate::protocol::WriteError> {
                let mut buffer = bytes::BytesMut::with_capacity(5); // TODO: constant
                match self {
                    $(
                        $name::$variant_name => buffer.extend_from_slice(&crate::varint::VarInt($variant_value).write()?),
                    )*
                };
                Ok(buffer.freeze())
            }
        }
    };
}
