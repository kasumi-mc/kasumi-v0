use serde::{Deserialize, Serialize};

use crate::protocol::{Readable, Writeable, text::TextComponent};

/// Representation of a `version` field in the Server List Ping object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerListPingVersion {
    /// Name of the version, usually set to version (i.e. "1.21.5").
    pub name: String,
    /// Version's protocol that server supports.
    pub protocol: u32,
}

/// Represents a single player from a `sample` array.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerListPingPlayer {
    /// UUID of the player.
    pub id: String, // TODO: use UUID
    /// Nickname of the player.
    pub name: String,
}

/// Representation of a `players` field in the Server List Ping object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerListPingPlayers {
    /// Maximum amount of players that server accepts. Can be a negative number.
    pub max: i32,
    /// Amount of online players. Can be a negative number.
    pub online: i32,
    /// List of online players. See documentation of `ServerListPingPlayer`
    /// for more information about the contents of this field.
    pub sample: Vec<ServerListPingPlayer>,
}

/// Representation of a Server List Ping - the payload that is sent when the
/// client requests information about the server to be displayed in the server
/// list.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerListPing {
    /// Server's version details. See documentation of `ServerListPingVersion`
    /// for more information.
    pub version: ServerListPingVersion,
    /// Details about server players. See documentation of
    /// `ServerListPingPlayers` for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<ServerListPingPlayers>,
    /// The "Message Of The Day" (MOTD) of the server represented as text
    /// component. See `TextComponent` documentation for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextComponent>,
    /// Favicon icon of the server in base64 format. Must be 64x64. If omitted,
    /// the default icon from a texture pack is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    /// Whether the server enforces the secure chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforces_secure_chat: Option<bool>,
}

impl Readable for ServerListPing {
    fn read(buffer: &[u8]) -> Result<(Self, usize), super::ReadError> {
        let (payload, read_len) = String::read(buffer)?;
        Ok((serde_json::from_str(&payload)?, read_len))
    }
}

impl Writeable for ServerListPing {
    fn write(&self) -> Result<bytes::Bytes, super::WriteError> {
        serde_json::to_string(&self)?.write()
    }
}
