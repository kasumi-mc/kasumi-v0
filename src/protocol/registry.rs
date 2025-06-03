use std::collections::HashMap;

use crate::{
    protocol::{ProtocolState, ReadError, handlers::PacketHandlerFn, packets::Packet},
    varint::VarInt,
};

/// Represents the packets' registry, where the value (see
/// `ProtocolRegistry` documentation) is the decoder function for each packet.
pub type PacketsRegistry =
    ProtocolRegistry<Box<dyn Fn(&[u8]) -> Result<(Box<dyn Packet>, usize), ReadError>>>;

/// Represents the registry for all packets handlers (see `ProtocolRegistry`)
/// documentation, where value is the handler function itself.
pub type HandlersRegistry = ProtocolRegistry<PacketHandlerFn>;

/// A generic registry for mapping Minecraft protocol states and packet IDs to
/// values (e.g. handlers, etc.).
#[derive(Debug, Clone)]
pub struct ProtocolRegistry<V: Sized> {
    inner: HashMap<(ProtocolState, VarInt), V>,
}

impl<V: Sized> Default for ProtocolRegistry<V> {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl<V: Sized> ProtocolRegistry<V> {
    /// Registers a new combination of protocol state, flow and packet ID to
    /// the provided value. For example, if building a packet registry, value
    /// would be a packet.
    pub fn register(&mut self, state: ProtocolState, id: VarInt, value: V) {
        // TODO: return an error, if there are already value with this combination present
        self.inner.insert((state, id), value);
    }

    /// Tries to get the value by the protocol state, flow and packet's ID. If
    /// there is some entry present, return it. For example, in a packet
    /// registry, it would be a packet itself.
    pub fn get(&self, state: ProtocolState, id: VarInt) -> Option<&V> {
        self.inner.get(&(state, id))
    }

    /// Removes, if present, the entry by its protocol state, flow and packet ID.
    pub fn remove(&mut self, state: ProtocolState, id: VarInt) {
        self.inner.remove(&(state, id));
    }
}
