use crate::{
    define_packet, define_varint_enum, protocol::registry::PacketsRegistry, register_packet,
    varint::VarInt,
};

/// Setups the registry for this packets set and protocol state. Only
/// serverbound packets are registered, through.
pub fn setup_registry(registry: &mut PacketsRegistry) {
    register_packet!(registry, ServerboundHandshakePacket);
}

define_varint_enum!(HandshakeIntent, {
    Status = 0x01,
    Login = 0x02,
    Transfer = 0x03,
});

define_packet!(ServerboundHandshakePacket, 0x00, Handshake, {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    intent: HandshakeIntent,
});
