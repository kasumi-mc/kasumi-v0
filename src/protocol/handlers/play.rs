// ClientboundSynchronizePlayerPositionPacket

use bytes::{BufMut, BytesMut};

use crate::{
    Packet,
    connection::Connection,
    handler_adapter,
    protocol::{
        BitSet, PrefixedArray, ProtocolState,
        packets::play::{
            ChunkData, ClientboundChunkDataAndLightPacket, ClientboundGameEventPacket, LightData,
            ServerboundConfirmTeleportationPacket,
        },
        registry::HandlersRegistry,
    },
    varint::VarInt,
    world::{Chunk, Heightmap},
};

/// Setups the registry for this handlers set and protocol state. Only handlers
/// for serverbound packets are registered, through.
pub fn setup_registry(registry: &mut HandlersRegistry) {
    registry.register(
        ProtocolState::Play,
        ServerboundConfirmTeleportationPacket::PACKET_ID,
        handler_adapter!(
            ServerboundConfirmTeleportationPacket,
            handle_confirm_teleportation
        ),
    );
}

fn encode_all_air_section() -> Vec<u8> {
    let mut buf = BytesMut::new();

    // Block count (u16) = 0
    buf.put_u16(0);

    // Bits per block (u8) = 0
    buf.put_u8(0);

    // Palette length (VarInt) = 1
    buf.put_u8(1);

    // Palette[0] (VarInt) = 0 (air)
    buf.put_u8(0);

    // Data array length (VarInt) = 0
    buf.put_u8(0);

    // Biome palette length (VarInt) = 1 (just plains)
    buf.put_u8(1);

    // Biome[0] (VarInt) = 0 (plains biome)
    buf.put_u8(0);

    // Biome data array length (VarInt) = 0 (since only one biome in palette)
    buf.put_u8(0);

    buf.to_vec()
}

pub fn handle_confirm_teleportation(
    connection: &mut Connection,
    packet: &ServerboundConfirmTeleportationPacket,
) {
    println!("{:?}", packet);

    let game_event_packet = ClientboundGameEventPacket {
        event: 13,
        value: 0.0,
    };
    connection.write_packet(Box::new(game_event_packet));

    let fake_heightmap_nbt = vec![
        0x0a, 0x00, 0x00, // TAG_Compound ""
        0x09, 0x00, 0x0f, // TAG_Long_Array "MOTION_BLOCKING"
        b'M', b'O', b'T', b'I', b'O', b'N', b'_', b'B', b'L', b'O', b'C', b'K', b'I', b'N', b'G',
        0x00, 0x00, 0x00, 0x10, // 16 entries
        // 16*8 bytes zero (fake, should be enough for your world height)
        // ... fill in with zeroes ...
        0x00, // TAG_End
    ];
    let heightmap = PrefixedArray(vec![fake_heightmap_nbt]);

    let mut all_sections = Vec::new();
    for _ in 0..16 {
        let section = encode_all_air_section();
        all_sections.extend_from_slice(&section);
    }

    let heightmaps = vec![Heightmap::new(VarInt(0)), Heightmap::new(VarInt(1))];
    let packet = ClientboundChunkDataAndLightPacket {
        chunk_x: 0,
        chunk_z: 0,
        chunk_data: ChunkData {
            heightmap: PrefixedArray(heightmaps),
            data: Chunk::new(0, 0),
            block_entities: PrefixedArray(vec![]),
        },
        light_data: LightData {
            sky_light_mask: BitSet::empty(),
            block_light_mask: BitSet::empty(),
            empty_sky_light_mask: BitSet::empty(),
            empty_block_light_mask: BitSet::empty(),
            sky_lights: PrefixedArray(vec![]),
            block_lights: PrefixedArray(vec![]),
        },
    };
    connection.write_packet(Box::new(packet));
}
