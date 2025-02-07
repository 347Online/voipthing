use std::net::{Ipv4Addr, Ipv6Addr};

use common::DEFAULT_PORT;
use silence::{
    packet::VoipPacket,
    udp::client::{Client, establish_connection},
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("Hello from the client!");

    let uuid = Uuid::now_v7();
    let port = DEFAULT_PORT;
    let addr = (Ipv6Addr::LOCALHOST, 12412);
    let client = Client::new(uuid, addr).await.unwrap();
    match establish_connection(addr).await {
        Ok(sock) => {
            sock.connect(addr).await.unwrap();
            dbg!(sock);
        }
        Err(err) => eprintln!("ERROR: {err}"),
    };
    match client
        .send_message(&VoipPacket(b"Hello world".into()))
        .await
    {
        Ok(x) => println!("Sent, number was {x}"),
        Err(err) => eprintln!("ERROR: {err}"),
    }
}
