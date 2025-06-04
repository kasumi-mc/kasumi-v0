use crate::{
    define_packet,
    protocol::{registry::PacketsRegistry, server_list_ping::ServerListPing},
    register_packet,
};

/// Setups the registry for this packets set and protocol state. Only
/// serverbound packets are registered, through.
pub fn setup_registry(registry: &mut PacketsRegistry) {
    register_packet!(registry, ServerboundStatusRequestPacket);
    register_packet!(registry, ServerboundPingRequestPacket);
}

define_packet!(ServerboundStatusRequestPacket, 0x00, Status, {});
define_packet!(ServerboundPingRequestPacket, 0x01, Status, {
    value: i64,
});

define_packet!(ClientboundStatusResponsePacket, 0x00, Status, {
    response: ServerListPing,
});
define_packet!(ClientboundPingResponsePacket, 0x01, Status, {
    value: i64,
});
