use crate::{
    Packet,
    connection::Connection,
    handler_adapter,
    protocol::{
        PrefixedArray, ProtocolState, Writeable,
        identifier::Identifier,
        packets::{
            configuration::{
                ClientboundFinishConfigurationPacket, ClientboundKnownPacksPacket,
                ClientboundRegistryDataPacket, KnownPack, ServerboundAcknowledgeFinishPacket,
                ServerboundClientInformationPacket, ServerboundKnownPacksPacket,
            },
            play::{ClientboundPlayPacket, ClientboundSynchronizePlayerPositionPacket},
        },
        registry::HandlersRegistry,
    },
    registry::build_registries_data,
    varint::VarInt,
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
    registry.register(
        ProtocolState::Configuration,
        ServerboundAcknowledgeFinishPacket::PACKET_ID,
        handler_adapter!(
            ServerboundAcknowledgeFinishPacket,
            handle_acknowledge_finish_configuration
        ),
    );
}

pub fn handle_client_information(
    connection: &mut Connection,
    packet: &ServerboundClientInformationPacket,
) {
    println!("Received client information: {packet:?}");
    let core_pack = KnownPack {
        namespace: Identifier::minecraft("core"),
        id: String::from("wtf"),
        version: String::from("1.21.5"),
    };
    connection.write_packet(Box::new(ClientboundKnownPacksPacket {
        packs: PrefixedArray(vec![core_pack]),
    }));
    println!("Sent ClientboundKnownPacksPacket");
}

pub fn handle_known_packs(connection: &mut Connection, _: &ServerboundKnownPacksPacket) {
    println!("Received ServerboundKnownPacksPacket");
    let registries = build_registries_data().unwrap();
    for registry in registries {
        println!("Written registry: {}", registry.registry_id);
        let packet = ClientboundRegistryDataPacket {
            registry_data: registry.write().unwrap().to_vec(),
        };
        connection.write_packet(Box::new(packet));
    }
    let finish_configuration_packet = ClientboundFinishConfigurationPacket {};
    connection.write_packet(Box::new(finish_configuration_packet));
    println!("Sent ClientboundFinishConfigurationPacket");
}

pub fn handle_acknowledge_finish_configuration(
    connection: &mut Connection,
    _: &ServerboundAcknowledgeFinishPacket,
) {
    println!("State -> Play");
    connection.set_state(ProtocolState::Play);
    let dimension_names = vec![Identifier::minecraft("overworld")];
    let play_packet = ClientboundPlayPacket {
        entity_id: 0_i32,
        is_hardcore: false,
        dimension_names: PrefixedArray(dimension_names.clone()),
        max_players: VarInt(1337),
        view_distance: VarInt(16),
        simulation_distance: VarInt(12),
        reduced_debug_info: false,
        enable_respawn_screen: true,
        do_limited_crafting: false,
        dimension_type: VarInt(0),
        dimension_name: dimension_names[0].clone(),
        hashed_seed: 0_i64,
        game_mode: 0_u8,
        previous_game_mode: 0_i8,
        is_debug: true,
        is_flat: false,
        has_death_location: false,
        death_dimension_name: None,
        death_location: None,
        portal_cooldown: VarInt(20),
        sea_level: VarInt(100),
        enforces_secure_chat: false,
    };
    connection.write_packet(Box::new(play_packet));

    let synchronize_player_position_packet = ClientboundSynchronizePlayerPositionPacket {
        teleport_id: VarInt(0),
        x: 0.0,
        y: 0.0,
        z: 0.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        velocity_z: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        flags: 0,
    };
    connection.write_packet(Box::new(synchronize_player_position_packet));
}
