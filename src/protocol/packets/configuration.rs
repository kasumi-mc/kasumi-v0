use bytes::BytesMut;

use crate::{
    define_packet, define_varint_enum,
    network::BufferReader,
    protocol::{
        PrefixedArray, Readable, Writeable, identifier::Identifier, registry::PacketsRegistry,
    },
    register_packet,
};

/// Setups the registry for this packets set and protocol state. Only
/// serverbound packets are registered, through.
pub fn setup_registry(registry: &mut PacketsRegistry) {
    register_packet!(registry, ServerboundClientInformationPacket);
    register_packet!(registry, ServerboundPluginMessagePacket);
    register_packet!(registry, ServerboundAcknowledgeFinishPacket);
    register_packet!(registry, ServerboundKnownPacksPacket);
}

#[derive(Debug, Clone)]
pub struct KnownPack {
    pub namespace: Identifier,
    pub id: String,
    pub version: String,
}

impl Readable for KnownPack {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        let mut reader = BufferReader::new(buffer);
        let namespace = reader.read(Identifier::read)?;
        let id = reader.read(String::read)?;
        let version = reader.read(String::read)?;
        Ok((
            Self {
                namespace,
                id,
                version,
            },
            reader.consumed(),
        ))
    }
}

impl Writeable for KnownPack {
    fn write(&self) -> Result<bytes::Bytes, crate::protocol::WriteError> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.namespace.write()?);
        buffer.extend_from_slice(&self.id.write()?);
        buffer.extend_from_slice(&self.version.write()?);
        Ok(buffer.freeze())
    }
}

define_varint_enum!(ClientInformationChatMode, {
    Enabled = 0x00,
    CommandsOnly = 0x01,
    Hidden = 0x02,
});

define_varint_enum!(ClientInformationMainHand, {
    Left = 0x00,
    Right = 0x01
});

define_varint_enum!(ClientInformationParticleStatus, {
    All = 0x00,
    Decreased = 0x01,
    Minimal = 0x02,
});

define_packet!(ServerboundClientInformationPacket, 0x00, Configuration, {
    locale: String,
    view_distance: u8,
    chat_mode: ClientInformationChatMode,
    is_chat_colors: bool,
    displayed_skin_parts: u8, // TODO: proper typing
    main_hand: ClientInformationMainHand,
    enable_text_filtering: bool,
    allow_server_listings: bool,
    particle_status: ClientInformationParticleStatus,
});
define_packet!(ServerboundPluginMessagePacket, 0x02, Configuration, {
    channel: Identifier,
    // TODO: data
});
define_packet!(ServerboundAcknowledgeFinishPacket, 0x03, Configuration, {});
define_packet!(ServerboundKnownPacksPacket, 0x07, Configuration, {
    packs: PrefixedArray<KnownPack>,
});

define_packet!(ClientboundKnownPacksPacket, 0x03, Configuration, {
    packs: PrefixedArray<KnownPack>,
});
define_packet!(ClientboundFinishConfigurationPacket, 0x03, Configuration, {
});
define_packet!(ClientboundRegistryDataPacket, 0x07, Configuration, {
    registry_data: Vec<u8>,
});
