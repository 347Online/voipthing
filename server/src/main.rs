use common::DEFAULT_PORT;
use silence::udp::server::Server;

#[tokio::main]
async fn main() {
    let port = DEFAULT_PORT;

    let server = Server::new(port);

    println!("Hello from the server!");
}
