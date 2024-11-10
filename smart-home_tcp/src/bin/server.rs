use std::error::Error;
use std::fs;

use smart_home::devices::socket::SmartSocket;
use smart_home_tcp_client::devices::socket::SocketServer;
use smart_home_tcp_client::devices::tcp_device::TcpDevice;

fn main() -> Result<(), Box<dyn Error>> {
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let device = SmartSocket::new("socket", "socket", 1000);
    let mut smart_socket_server = SocketServer::bind(device, addr.clone())?;

    println!("Server started on {}", addr);

    loop {
        let Ok(mut connection) = smart_socket_server.accept() else {
            println!("Connection failed");
            continue;
        };

        smart_socket_server.handle(&mut connection)?;
    }
}
