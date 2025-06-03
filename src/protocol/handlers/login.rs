use crate::{
    Packet,
    connection::Connection,
    handler_adapter,
    protocol::{
        PrefixedArray, ProtocolState,
        packets::login::{
            ClientboundLoginSuccessPacket, ServerboundLoginAcknowledgedPacket,
            ServerboundLoginStartPacket,
        },
        registry::HandlersRegistry,
    },
};

/// Setups the registry for this handlers set and protocol state. Only handlers
/// for serverbound packets are registered, through.
pub fn setup_registry(registry: &mut HandlersRegistry) {
    registry.register(
        ProtocolState::Login,
        ServerboundLoginStartPacket::PACKET_ID,
        handler_adapter!(ServerboundLoginStartPacket, handle_login_start),
    );
    registry.register(
        ProtocolState::Login,
        ServerboundLoginAcknowledgedPacket::PACKET_ID,
        handler_adapter!(
            ServerboundLoginAcknowledgedPacket,
            handle_login_acknowledged
        ),
    );
}

/// Handles the incoming `LoginStart` packet.
pub fn handle_login_start(connection: &mut Connection, packet: &ServerboundLoginStartPacket) {
    // for now, just send the whole LoginSuccess packet
    let packet = ClientboundLoginSuccessPacket {
        id: packet.id,
        name: packet.name.to_owned(),
        properties: PrefixedArray(vec![]),
    };
    connection.write_packet(Box::new(packet));
}

/// Handles the incoming `LoginAcknowledged` packet.
pub fn handle_login_acknowledged(
    connection: &mut Connection,
    _: &ServerboundLoginAcknowledgedPacket,
) {
    connection.set_state(ProtocolState::Configuration);
}
