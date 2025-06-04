use bytes::BytesMut;

use crate::{
    define_packet,
    network::BufferReader,
    protocol::{
        BitSet, PrefixedArray, Readable, Writeable, identifier::Identifier,
        registry::PacketsRegistry,
    },
    register_packet,
    varint::VarInt,
    world::{Chunk, Heightmap},
};

/// Setups the registry for this packets set and protocol state. Only
/// serverbound packets are registered, through.
pub fn setup_registry(registry: &mut PacketsRegistry) {
    register_packet!(registry, ServerboundConfirmTeleportationPacket);
}

define_packet!(ServerboundConfirmTeleportationPacket, 0x00, Play, {
    teleport_id: VarInt
});

define_packet!(ClientboundPlayPacket, 0x2B, Play, {
    entity_id: i32,
    is_hardcore: bool,
    dimension_names: PrefixedArray<Identifier>,
    max_players: VarInt,
    view_distance: VarInt,
    simulation_distance: VarInt,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
    do_limited_crafting: bool,
    dimension_type: VarInt,
    dimension_name: Identifier,
    hashed_seed: i64,
    game_mode: u8, // TODO: enum type
    previous_game_mode: i8, // TODO: enum type
    is_debug: bool,
    is_flat: bool,
    has_death_location: bool,
    death_dimension_name: Option<Identifier>,
    death_location: Option<i64>,
    portal_cooldown: VarInt,
    sea_level: VarInt,
    enforces_secure_chat: bool,
});
define_packet!(ClientboundSynchronizePlayerPositionPacket, 0x41, Play, {
    teleport_id: VarInt,
    x: f64,
    y: f64,
    z: f64,
    velocity_x: f64,
    velocity_y: f64,
    velocity_z: f64,
    yaw: f32,
    pitch: f32,
    flags: i32,
});
define_packet!(ClientboundGameEventPacket, 0x22, Play, {
    event: u8,
    value: f32,
});

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub heightmap: PrefixedArray<Heightmap>,
    pub data: Chunk,
    pub block_entities: PrefixedArray<u8>,
}

impl Readable for ChunkData {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        let mut reader = BufferReader::new(buffer);
        let heightmap = reader.read(PrefixedArray::read)?;
        let data = reader.read(Chunk::read)?;
        let block_entities = reader.read(PrefixedArray::read)?;
        Ok((
            Self {
                heightmap,
                data,
                block_entities,
            },
            reader.consumed(),
        ))
    }
}

impl Writeable for ChunkData {
    fn write(&self) -> Result<bytes::Bytes, crate::protocol::WriteError> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.heightmap.write()?);
        buffer.extend_from_slice(&self.data.write()?);
        buffer.extend_from_slice(&self.block_entities.write()?);
        Ok(buffer.freeze())
    }
}

#[derive(Debug, Clone)]
pub struct LightData {
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_lights: PrefixedArray<Vec<u8>>,
    pub block_lights: PrefixedArray<Vec<u8>>,
}

impl Readable for LightData {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        let mut reader = BufferReader::new(buffer);
        let sky_light_mask = reader.read(BitSet::read)?;
        let block_light_mask = reader.read(BitSet::read)?;
        let empty_sky_light_mask = reader.read(BitSet::read)?;
        let empty_block_light_mask = reader.read(BitSet::read)?;
        let sky_lights = reader.read(PrefixedArray::read)?;
        let block_lights = reader.read(PrefixedArray::read)?;
        Ok((
            Self {
                sky_light_mask,
                block_light_mask,
                empty_sky_light_mask,
                empty_block_light_mask,
                sky_lights,
                block_lights,
            },
            reader.consumed(),
        ))
    }
}

impl Writeable for LightData {
    fn write(&self) -> Result<bytes::Bytes, crate::protocol::WriteError> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.sky_light_mask.write()?);
        buffer.extend_from_slice(&self.block_light_mask.write()?);
        buffer.extend_from_slice(&self.empty_sky_light_mask.write()?);
        buffer.extend_from_slice(&self.empty_block_light_mask.write()?);
        buffer.extend_from_slice(&self.sky_lights.write()?);
        buffer.extend_from_slice(&self.block_lights.write()?);
        Ok(buffer.freeze())
    }
}

define_packet!(ClientboundChunkDataAndLightPacket, 0x27, Play, {
    chunk_x: i32,
    chunk_z: i32,
    chunk_data: ChunkData,
    light_data: LightData,
});
