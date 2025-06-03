use crate::{
    Packet,
    connection::Connection,
    handler_adapter,
    protocol::{
        PrefixedArray, ProtocolState, Writeable,
        identifier::Identifier,
        packets::configuration::{
            ClientboundFinishConfigurationPacket, ClientboundKnownPacksPacket,
            ClientboundRegistryDataPacket, KnownPack, ServerboundClientInformationPacket,
            ServerboundKnownPacksPacket,
        },
        registry::HandlersRegistry,
    },
    registry::build_registries_data,
};

/// Setups the registry for this handlers set and protocol state. Only handlers
/// for serverbound packets are registered, through.
pub fn setup_registry(registry: &mut HandlersRegistry) {
    registry.register(
        ProtocolState::Configuration,
        ServerboundClientInformationPacket::PACKET_ID,
        handler_adapter!(
            ServerboundClientInformationPacket,
            handle_client_information
        ),
    );
    registry.register(
        ProtocolState::Configuration,
        ServerboundKnownPacksPacket::PACKET_ID,
        handler_adapter!(ServerboundKnownPacksPacket, handle_known_packs),
    );
}

pub fn handle_client_information(
    connection: &mut Connection,
    _: &ServerboundClientInformationPacket,
) {
    let core_pack = KnownPack {
        namespace: Identifier::minecraft("core".to_string()),
        id: String::from("wtf"),
        version: String::from("1.21.5"),
    };
    connection.write_packet(Box::new(ClientboundKnownPacksPacket {
        packs: PrefixedArray(vec![core_pack]),
    }));
}

pub fn handle_known_packs(connection: &mut Connection, _: &ServerboundKnownPacksPacket) {
    let registries = build_registries_data().unwrap();
    for registry in registries {
        let packet = ClientboundRegistryDataPacket {
            registry_data: registry.write().unwrap().to_vec(),
        };
        connection.write_packet(Box::new(packet));
    }
    let finish_configuration_packet = ClientboundFinishConfigurationPacket {};
    connection.write_packet(Box::new(finish_configuration_packet));
}
