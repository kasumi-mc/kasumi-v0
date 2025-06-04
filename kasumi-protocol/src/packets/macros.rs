//! Defines and implements useful QoL macros for protocol.

/// Declares an enum representing a protocol VarInt enum and implements
/// protocol (de)serialization traits.
///
/// This macro generates an `enum` whose discriminants correspond to explicit
/// integer values, and provides implementations for conversion, serialization,
/// and deserialization as required.
///
/// # Usage
///
/// ```rust
/// define_varint_enum! {
///     /// Represents the protocol's direction.
///     pub Direction, {
///         Serverbound = 0,
///         Clientbound = 1,
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! define_varint_enum {
    (
        $(#[$meta:meta])*
        $enum_name:ident, {
            $( $variant_name:ident = $variant_value:literal$(,)? )*
        }
    ) => {
        $(#[$meta])*
        #[repr(i32)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $enum_name {
            $(
                $variant_name = $variant_value,
            )*
        }

        impl $enum_name {
            /// Returns the value of the this variant.
            pub fn as_i32(&self) -> i32 {
                match self {
                    $(
                        Self::$variant_name => $variant_value,
                    )*
                }
            }

            /// Returns the value of this variant as `VarInt`.
            pub fn as_varint(&self) -> $crate::varint::VarInt {
                VarInt(self.as_i32())
            }
        }

        impl From<$enum_name> for i32 {
            fn from(value: $enum_name) -> Self {
                match value {
                    $(
                        $enum_name::$variant_name => $variant_value,
                    )*
                }
            }
        }

        impl $crate::Readable for $enum_name {
            fn read(buffer: &mut bytes::Bytes) -> std::result::Result<Self, $crate::ReadError> {
                let value = $crate::varint::VarInt::read(buffer)?.0;
                match value {
                    $(
                        $variant_value => Ok(Self::$variant_name),
                    )*
                    _ => Err($crate::ReadError::UnknownVariant(value))
                }
            }
        }

        impl $crate::Writable for $enum_name {
            fn write(&self, buffer: &mut bytes::BytesMut) -> std::result::Result<(), $crate::WriteError> {
                self.as_varint().write(buffer)?;
                Ok(())
            }
        }
    };
}

/// Defines a protocol packet struct and auto-implements `Readable` and
/// `Writable` traits.
///
/// This macro generates a packet struct with the specified fields, ID, and
/// connection state. It also implements the `Readable` and `Writable` traits
/// for the struct, making it easy to use for serialization/deserialization.
///
/// See documentation for `Readable` and `Writable` for more information about
/// reading and writing each individual field and packets in general. Requires
/// each field of the packet to implement both `Readable` and `Writable`. Also
/// because of this the order of fields must match the on-wire protocol order.
///
/// # Usage
///
/// ```rust
/// define_packet!(
///     /// Packet that is sent from the client to the server with handshake data.
///     ServerboundHandshakePacket, 0x00, Handshake, { protocol_version: VarInt }
/// );
///
/// define_packet!(
///     /// Packet that client is sending to request the Server List Ping.
///     ServerboundStatusRequestPacket, 0x00, Status, {}
/// );
/// ```
#[macro_export]
macro_rules! define_packet {
    (
        $(#[$meta:meta])*
        $packet_name:ident, $packet_id:literal, $state:ident, {
            $( $field_name:ident : $field_type:ty$(,)? )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $packet_name {
            $(
                $field_name: $field_type,
            )*
        }

        impl $packet_name {
            /// ID of this individual packet. Can be not unique across the
            /// whole protocol. You should also compare `STATE` constant to
            /// identify the packet.
            pub const ID: $crate::varint::VarInt = $crate::varint::VarInt($packet_id);

            /// The connection state that this packet is sent within. If you
            /// combine this with the ID of the packet, you can surely identify
            /// the packet.
            pub const STATE: $crate::state::ConnectionState = $crate::state::ConnectionState::$state;
        }

        impl $crate::Readable for $packet_name {
            fn read(buffer: &mut bytes::Bytes) -> std::result::Result<Self, $crate::ReadError> {
                $(
                    let $field_name = <$field_type as $crate::Readable>::read(buffer)?;
                )*
                Ok(Self { $($field_name,)* })
            }
        }

        impl $crate::Writable for $packet_name {
            fn write(&self, buffer: &mut bytes::BytesMut) -> std::result::Result<(), $crate::WriteError> {
                $(
                    self.$field_name.write(buffer)?;
                )*
                Ok(())
            }
        }
    };
}
