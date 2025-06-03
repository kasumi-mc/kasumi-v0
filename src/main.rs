use std::{net::TcpListener, sync::Arc};

use crate::protocol::{
    handlers::{self},
    packets::{self, Packet},
    registry::{HandlersRegistry, PacketsRegistry},
};

use crate::connection::Connection;

pub mod connection;
pub mod network;
pub mod protocol;
pub mod registry;
pub mod varint;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:25565").unwrap();

    let mut registry = PacketsRegistry::default();
    packets::handshake::setup_registry(&mut registry);
    packets::status::setup_registry(&mut registry);
    packets::login::setup_registry(&mut registry);
    packets::configuration::setup_registry(&mut registry);

    let mut handler_registry = HandlersRegistry::default();
    handlers::handshake::setup_registry(&mut handler_registry);
    handlers::status::setup_registry(&mut handler_registry);
    handlers::login::setup_registry(&mut handler_registry);
    // handlers::configuration::setup_registry(&mut handler_registry);

    let packet_registry = Arc::new(registry);
    let handler_registry = Arc::new(handler_registry);

    loop {
        let (stream, addr) = match listener.accept() {
            Ok(values) => values,
            Err(e) => {
                eprintln!("Failed to accept the stream: {e}");
                continue;
            }
        };

        println!("New client from {addr}");
        Connection::new(stream, packet_registry.clone(), handler_registry.clone())
            .serve()
            .unwrap();
    }
}
