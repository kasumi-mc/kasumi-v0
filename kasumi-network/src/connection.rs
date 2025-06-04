use bytes::BytesMut;
use kasumi_protocol::state::ConnectionState;
use thiserror::Error;
use tokio::{io::AsyncReadExt, net::TcpStream};

/// The maximum number of bytes to be present at any given point in time. Since
/// we are using a "rolling" buffer (i.e., after the packet is read, its whole
/// body will be removed), this should be a plenty of space.
/// Protocol should accommodate for this size.
pub const BUFFER_CAPACITY: usize = 4096;

/// Errors that can occur during connection operations.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// Indicates that some type of I/O error has occurred.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// High-level wrapper around a low-level connection, providing convenient
/// read/write operations as well as tracking the current protocol and
/// connection state.
pub struct Connection {
    /// The underlying stream for the connection from the client to the server.
    stream: TcpStream,
    /// The current state of the connection to distinguish between packets.
    state: ConnectionState,
    /// The underlying buffer for reading operations.
    read_buffer: BytesMut,
}

impl Connection {
    /// Creates a new `Connection` instance from the provided stream and
    /// allocates the required memory for its buffer.
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            state: ConnectionState::Handshake,
            read_buffer: BytesMut::with_capacity(BUFFER_CAPACITY),
        }
    }

    /// Retrieves the current protocol and connection state.
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Sets the connection state to the provided value. The caller is
    /// responsible for validating that the new state is correct.
    pub fn set_state(&mut self, new_state: ConnectionState) {
        self.state = new_state
    }

    /// Serves the connection with the error stack propagation. This method
    /// performs two crucial actions:
    /// 1. Continuously reads from the buffer,
    /// 2. Tries to read and serialize the packet from the available data.
    pub async fn serve(&mut self) -> Result<(), ConnectionError> {
        loop {
            let read_size = self.stream.read_buf(&mut self.read_buffer).await?;
            // client has disconnected
            if read_size == 0 {
                break;
            }
        }

        Ok(())
    }
}
