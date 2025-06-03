use crate::{
    Packet,
    connection::Connection,
    handler_adapter,
    protocol::{
        ProtocolState,
        packets::status::{
            ClientboundPingResponsePacket, ClientboundStatusResponsePacket,
            ServerboundPingRequestPacket, ServerboundStatusRequestPacket,
        },
        registry::HandlersRegistry,
        server_list_ping::{ServerListPing, ServerListPingPlayers, ServerListPingVersion},
        text::{Color, NamedColor, TextComponent, TextComponentKind},
    },
};

/// Setups the registry for this handlers set and protocol state. Only handlers
/// for serverbound packets are registered, through.
pub fn setup_registry(registry: &mut HandlersRegistry) {
    registry.register(
        ProtocolState::Status,
        ServerboundStatusRequestPacket::PACKET_ID,
        handler_adapter!(ServerboundStatusRequestPacket, handle_status_request),
    );
    registry.register(
        ProtocolState::Status,
        ServerboundPingRequestPacket::PACKET_ID,
        handler_adapter!(ServerboundPingRequestPacket, handle_ping_request),
    );
}

/// Handles the `StatusRequest` packet sent by a client.
pub fn handle_status_request(connection: &mut Connection, _: &ServerboundStatusRequestPacket) {
    // TODO: this packet should be taken from server's configuration
    // TODO: handle Legacy Server Ping
    let packet = ClientboundStatusResponsePacket {
        response: ServerListPing {
            version: ServerListPingVersion {
                name: "1.21.5".to_owned(),
                protocol: 770,
            },
            players: Some(ServerListPingPlayers {
                max: 1337,
                online: -1,
                sample: vec![],
            }),
            description: Some(TextComponent {
            kind: TextComponentKind::Text {
                text: "Hello, World! ".to_owned(),
            },
            extra: Some(vec![TextComponent {
                kind: TextComponentKind::Text {
                    text: "This is Kasumi".to_owned(),
                },
                extra: None,
                color: Some(Color::Named(NamedColor::Gold)),
                font: None,
                bold: None,
                italic: Some(true),
                underlined: Some(true),
                strikethrough: None,
                obfuscated: None,
                shadow_color: None,
                insertion: None,
                click_event: None,
                hover_event: None,
            }]),
            color: Some(Color::Named(NamedColor::DarkGreen)),
            font: None,
            bold: Some(true),
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            shadow_color: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        }),
            favicon: Some("data:image/png;base64,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".to_string()),
            enforces_secure_chat: Some(false),
        },
    };

    connection.write_packet(Box::new(packet));
}

pub fn handle_ping_request(connection: &mut Connection, packet: &ServerboundPingRequestPacket) {
    connection.write_packet(Box::new(ClientboundPingResponsePacket {
        value: packet.value,
    }));
}
