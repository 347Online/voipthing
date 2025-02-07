use common::DEFAULT_PORT;
use silence::{
    packet::{VoipHeader, VoipMessageType, VoipPacket},
    udp::server::Server,
};

#[tokio::main]
async fn main() {
    let port = DEFAULT_PORT;

    let mut server = Server::new(port).await.unwrap();

    println!("Listening on port {}", port);

    loop {
        if let Some((header, bytes, sock)) = server.message_receiver().recv().await {
            dbg!(header, bytes, sock);
        }

        server
            .reply_to_clients(VoipPacket(Vec::from(b"Hello world")))
            .await
            .unwrap();
    }
}
