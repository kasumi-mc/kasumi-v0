use kasumi_network::connection::Connection;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:25565").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        Connection::new(stream).serve().await.unwrap();
    }
}
