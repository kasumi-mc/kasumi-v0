use crate::{
    Packet,
    connection::Connection,
    handler_adapter,
    protocol::{
        ProtocolState, packets::handshake::ServerboundHandshakePacket, registry::HandlersRegistry,
    },
};

/// Setups the registry for this handlers set and protocol state. Only handlers
/// for serverbound packets are registered, through.
pub fn setup_registry(registry: &mut HandlersRegistry) {
    registry.register(
        ProtocolState::Handshake,
        ServerboundHandshakePacket::PACKET_ID,
        handler_adapter!(ServerboundHandshakePacket, handle_handshake),
    );
}

/// Handles the received `Handshake` packet.
pub fn handle_handshake(connection: &mut Connection, packet: &ServerboundHandshakePacket) {
    match packet.intent {
        crate::protocol::packets::handshake::HandshakeIntent::Status => {
            connection.set_state(ProtocolState::Status)
        }
        crate::protocol::packets::handshake::HandshakeIntent::Login => {
            connection.set_state(ProtocolState::Login)
        }
        _ => {} // TODO: handle other cases
    };
    println!("Connection state now is: {:?}", connection.state);
    println!("Client is connecting with {}", packet.protocol_version);
}
