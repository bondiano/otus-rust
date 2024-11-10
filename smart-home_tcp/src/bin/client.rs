use std::env;
use std::error::Error;
use std::fs;
use std::net::TcpStream;

use smart_home_tcp_client::client::{receive_response, send_command};
use smart_home_tcp_client::devices::socket::SocketCommand;

fn main() -> Result<(), Box<dyn Error>> {
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <command>", args[0]);
        println!("Available commands: switch, status");
        return Ok(());
    }

    let command = match args[1].as_str() {
        "switch" => SocketCommand::Switch,
        "status" => SocketCommand::Status,
        _ => {
            println!("Unknown command. Available commands: switch, status");
            return Ok(());
        }
    };

    println!("Sending command: {:?}", command);
    let mut stream = TcpStream::connect(&addr)?;
    send_command(command.clone(), &mut stream)?;
    println!("Sent command: {:?} to: {}", command, addr);
    let response = receive_response(&mut stream)?;
    println!("{}", response);

    Ok(())
}
