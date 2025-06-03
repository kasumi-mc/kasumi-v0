use crate::{connection::Connection, protocol::packets::Packet};

pub mod configuration;
pub mod handshake;
pub mod login;
pub mod status;

/// Adapter for a each individual packet handler that converts a generic packet
/// type into a specific one (provided by the type).
///
/// # Examples
/// ```rust
/// handler_registry.register(
///     ProtocolState::Handshake,
///     0x00,
///     handler_adapter!(Handshake, handle_handshake), // `Handshake` here is the type, and `handle_handshake` is handler
/// );
/// ```
#[macro_export]
macro_rules! handler_adapter {
    ($ty:ty, $fn_name:ident) => {
        |connection: &mut Connection, packet: &Box<dyn Packet>| {
            if let Some(packet) = packet.as_any().downcast_ref::<$ty>() {
                $fn_name(connection, packet);
            }
        }
    };
}

/// Type for a single packet handler function.
pub type PacketHandlerFn = fn(&mut Connection, &Box<dyn Packet>);
