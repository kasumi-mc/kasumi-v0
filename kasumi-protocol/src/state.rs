//! The definition for the connection's state. Should be used by a lower-level
//! stream wrappers to differentiate between packets and to correctly parse and
//! handle them.

/// Represents the current connection state to differentiate the packets sent
/// by the client. Should only be changed by the stream owner (or via its API)
/// after a specific set of instructions and conditions are met.
///
/// Each variant is placed in logical order, representing the flow of the
/// packets between the client and the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// The initial state before any packets have been received. Both client
    /// and server start in the this state.
    Handshake,
    /// Indicates that client has requested the current status of the server.
    Status,
    /// Indicates that the client is currently logging in to the server and
    /// expects to be able to play.
    Login,
    /// Indicates that the client has completed login and is now exchanging
    /// configuration information with the server.
    Configuration,
    /// Indicates that the client is ready to receive game packets and is
    /// properly identified and configured.
    Play,
}
