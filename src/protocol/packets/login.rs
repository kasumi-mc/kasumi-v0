use bytes::{Bytes, BytesMut};
use uuid::Uuid;

use crate::{
    define_packet,
    network::BufferReader,
    protocol::{PrefixedArray, Readable, Writeable, registry::PacketsRegistry},
    register_packet,
};

/// Setups the registry for this packets set and protocol state. Only
/// serverbound packets are registered, through.
pub fn setup_registry(registry: &mut PacketsRegistry) {
    register_packet!(registry, ServerboundLoginStartPacket);
    register_packet!(registry, ServerboundLoginAcknowledgedPacket);
}

/// Represents a single player game property from a game profile sent in
/// `LoginSuccess` packet.
#[derive(Debug, Clone)]
pub struct Property {
    /// Name of this property. Must be unique.
    pub name: String,
    /// The value of this property.
    pub value: String,
    // TODO: signatures
}

impl Readable for Property {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        let mut reader = BufferReader::new(buffer);
        let name = reader.read(String::read)?;
        let value = reader.read(String::read)?;
        Ok((Self { name, value }, reader.consumed()))
    }
}

impl Writeable for Property {
    fn write(&self) -> Result<Bytes, crate::protocol::WriteError> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.name.write()?);
        buffer.extend_from_slice(&self.value.write()?);
        Ok(buffer.freeze())
    }
}

define_packet!(ServerboundLoginStartPacket, 0x00, Login, {
    name: String,
    id: Uuid,
});
define_packet!(ServerboundLoginAcknowledgedPacket, 0x03, Login, {});

define_packet!(ClientboundLoginSuccessPacket, 0x02, Login, {
    id: Uuid,
    name: String,
    properties: PrefixedArray<Property>,
});
