use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    protocol::{Readable, Writeable},
    varint::VarInt,
};

/// Converts the provided entries into the data array with the specified bits per entry size.
fn pack_data_array(entries: &[VarInt], bits_per_entry: usize) -> Vec<u64> {
    // TODO: constants
    let mut output = Vec::with_capacity((entries.len() * bits_per_entry + 63) / 64);
    let mut buffer = 0u64; // accumulator
    let mut bits_in_buffer = 0; // amount of bits written to the buffer

    for entry in entries.iter() {
        let value = entry.0 as u64;

        // if we've reached 64 bits...
        if bits_in_buffer >= 64 {
            output.push(buffer); // push current buffer
            buffer = value >> (bits_per_entry - (bits_in_buffer - 64)); // reset it, keeping the remaining bits
            bits_in_buffer -= 64; // and reset the counter
        }

        buffer |= value << bits_in_buffer; // append current entry to the buffer
        bits_in_buffer += bits_per_entry; // add amount of direct bits per entry to the counter
    }

    if bits_in_buffer > 0 {
        output.push(buffer);
    }

    output
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkSection {
    pub block_states: [VarInt; 16 * 16 * 16],
    pub biomes: [VarInt; 64],
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self {
            block_states: [VarInt(0); 4096],
            biomes: [VarInt(1); 64],
        }
    }
}

impl ChunkSection {
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block_id: VarInt) {
        let block_index = (y << 8) | (z << 4) | x;
        self.block_states[block_index] = block_id;
    }

    pub fn pack_block_states(&self) -> Vec<u64> {
        pack_data_array(&self.block_states, 15)
    }

    pub fn pack_biomes(&self) -> Vec<u64> {
        pack_data_array(&self.biomes, 6)
    }

    pub fn non_air_block_count(&self) -> u16 {
        self.block_states
            .iter()
            .filter(|b| b.0 != 0)
            .count()
            .min(u16::MAX as usize) as u16
    }
}

impl Readable for ChunkSection {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        todo!()
    }
}

impl Writeable for ChunkSection {
    fn write(&self) -> Result<bytes::Bytes, crate::protocol::WriteError> {
        let mut buffer = bytes::BytesMut::new();
        buffer.put_u16(self.non_air_block_count()); // block count (non-air)

        // BLOCK STATES START
        buffer.put_u8(15); // bits per block

        // block states
        let block_states_data = self.pack_block_states();
        for block_state_word in block_states_data {
            buffer.extend_from_slice(&block_state_word.to_be_bytes());
        }

        // BIOMES START
        buffer.put_u8(6); // bits per block
        let biomes_data = self.pack_biomes();
        for biome_word in biomes_data {
            buffer.extend_from_slice(&biome_word.to_be_bytes());
        }

        Ok(buffer.freeze())
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub sections: Vec<Option<ChunkSection>>,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            sections: vec![Some(ChunkSection::default()); 24],
        }
    }

    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block_id: VarInt) {
        let section_index = y.div_euclid(16);

        let local_section_x = x.rem_euclid(16);
        let local_section_y = y.rem_euclid(16);
        let local_section_z = z.rem_euclid(16);

        if self.sections[section_index].is_none() {
            self.sections[section_index] = Some(ChunkSection::default());
        }

        let section = self.sections[section_index].as_mut().unwrap();
        section.set_block_at(local_section_x, local_section_y, local_section_z, block_id);
    }
}

impl Readable for Chunk {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        todo!()
    }
}

impl Writeable for Chunk {
    fn write(&self) -> Result<Bytes, crate::protocol::WriteError> {
        let mut buffer = BytesMut::new();

        let mut section_data = BytesMut::new();
        for section in self.sections.iter().flatten() {
            section_data.extend_from_slice(&section.write()?);
        }

        buffer.extend_from_slice(&VarInt(section_data.len() as i32).write()?); // section data length
        buffer.extend_from_slice(&section_data); // section data itself

        Ok(buffer.freeze())
    }
}

#[derive(Debug, Clone)]
pub struct Heightmap {
    pub kind: VarInt,
    pub heights: [u16; 256],
}

impl Heightmap {
    pub fn new(kind: VarInt) -> Self {
        Self {
            kind,
            heights: [0; 256],
        }
    }

    pub fn pack(&self) -> Vec<u64> {
        let mut longs = Vec::with_capacity(36);
        let mut current = 0u64;
        let mut bits = 0;
        let mut index = 0;

        for value in self.heights {
            let value = value as u64;

            if bits + 9 > 64 {
                longs.push(current);
                index += 1;
                current = 0;
                bits = 0;
            }

            current |= value << bits;
            bits += 9;
        }

        if index < 36 {
            longs.push(current);
        }

        longs
    }
}

impl Readable for Heightmap {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        todo!()
    }
}

impl Writeable for Heightmap {
    fn write(&self) -> Result<Bytes, crate::protocol::WriteError> {
        let mut buffer = BytesMut::new();

        let packed = self.pack();
        buffer.extend_from_slice(&self.kind.write()?);
        buffer.extend_from_slice(&VarInt(packed.len() as i32).write()?);

        for word in packed {
            buffer.extend_from_slice(&word.to_be_bytes());
        }

        Ok(buffer.freeze())
    }
}
