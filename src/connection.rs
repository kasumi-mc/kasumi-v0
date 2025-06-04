use std::{
    io::{Error, Read, Write},
    net::TcpStream,
    sync::Arc,
};

use thiserror::Error;

use crate::{
    network::{BUFFER_CAPACITY, PacketReader, PacketReaderError},
    protocol::{
        ProtocolState, ReadError, Writeable,
        packets::Packet,
        registry::{HandlersRegistry, PacketsRegistry},
    },
    varint::{VarInt, VarIntError},
};

/// Errors that can occur with the client-server connection.
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// Indicates that something went wrong while reading the packets.
    #[error(transparent)]
    PacketReaderError(#[from] PacketReaderError),
    /// Indicates that some type of I/O error has occurred.
    #[error("I/O error has occurred: {0}")]
    IoError(Error),
    /// Indicates that something went wrong while reading the packet.
    #[error(transparent)]
    ReadError(#[from] ReadError),
}

/// Representation of the connection itself - the wrapper of the raw
/// `TcpStream` with packet parsing.
pub struct Connection {
    stream: TcpStream,
    reader: PacketReader,
    pub state: ProtocolState,

    // registries
    registry: Arc<PacketsRegistry>,
    handler_registry: Arc<HandlersRegistry>,
}

impl Connection {
    /// Creates a new instance of the `Connection` with the provided underlying
    /// stream, packet and handler registries.
    pub fn new(
        stream: TcpStream,
        registry: Arc<PacketsRegistry>,
        handler_registry: Arc<HandlersRegistry>,
    ) -> Self {
        Self {
            stream,
            reader: PacketReader::default(),
            state: ProtocolState::Handshake,
            registry,
            handler_registry,
            // client_information: None,
        }
    }

    /// Updates the protocol state of the client to the provided.
    pub fn set_state(&mut self, state: ProtocolState) {
        self.state = state;
    }

    pub fn write_packet(&mut self, packet: Box<dyn Packet>) {
        let mut body_buffer = vec![];
        body_buffer.extend_from_slice(&packet.id().write().unwrap()); // TODO: no unwrap
        body_buffer.extend_from_slice(&packet.write().unwrap());

        let mut buffer = vec![];
        buffer.extend_from_slice(&VarInt(body_buffer.len() as i32).write().unwrap());
        buffer.extend(body_buffer);

        self.stream.write_all(&buffer).unwrap();
    }

    /// Performs the handling of the connection in a loop with stack error
    /// propagation.
    pub fn serve(mut self) -> Result<(), ConnectionError> {
        let mut buffer = [0u8; BUFFER_CAPACITY];
        let mut observed_unknown_packets = vec![];

        loop {
            let size = match self.stream.read(&mut buffer) {
                Ok(0) => break, // client has disconnected
                Ok(size) => size,
                Err(e) => return Err(ConnectionError::IoError(e)),
            };

            self.reader.extend_from_slice(&buffer[..size]);
            while let Some((id, body)) = self.reader.try_next_packet()? {
                let packet_decode_fn = match self.registry.get(self.state, id) {
                    Some(data) => data,
                    None => {
                        if !observed_unknown_packets.contains(&id) {
                            eprintln!(
                                "Received an unknown packet ID={id} in state {:?}",
                                self.state
                            ); // TODO: use proper logging
                            observed_unknown_packets.push(id);
                        }

                        continue;
                    }
                };

                let (packet, _) = match packet_decode_fn(&body) {
                    Ok(value) => value,
                    Err(ReadError::VarIntError(VarIntError::Incomplete)) => continue, // we'll wait a little bit more...
                    Err(e) => return Err(ConnectionError::ReadError(e)),
                };

                if let Some(handler) = self.handler_registry.get(self.state, id) {
                    handler(&mut self, &packet);
                }
            }
        }

        Ok(())
    }
}
