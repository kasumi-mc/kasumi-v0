use crate::{define_packet, define_varint_enum, varint::VarInt};

define_varint_enum!(
    /// Describes client's intentions after the handshake.
    HandshakeIntent, {
        Status = 0x01,
        Login = 0x02,
        Transfer = 0x03,
    }
);

define_packet!(
    /// Packet that is sent by the client right after opening connection to the
    /// server, containing data required for later communication.
    ServerboundHandshakePacket, 0x00, Handshake, {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        intent: HandshakeIntent,
    }
);
