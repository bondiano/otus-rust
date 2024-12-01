use std::{error::Error, net::UdpSocket};

use smart_home_udp_client::protocol::receive_message;

fn main() -> Result<(), Box<dyn Error>> {
    let receiver_address = "127.0.0.1:4321";
    let mut socket = UdpSocket::bind(receiver_address)?;

    loop {
        match receive_message(&mut socket) {
            Ok(message) => println!("Received message: {message}"),
            Err(error) => println!("Error receiving message: {:?}", error),
        }
    }
}
