use smart_home_tcp_client::{
    client::{receive_response, send_command},
    devices::socket::SocketCommand,
    protocol::{ParseError, ProtocolError},
};
use std::net::TcpStream;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SocketError {
    #[error("Failed to connect to socket: {0}")]
    ConnectionError(#[from] std::io::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] ParseError),
    #[error("Protocol error: {0}")]
    ProtocolError(#[from] ProtocolError),
}

#[derive(Debug)]
pub struct SmartSocketClient {
    addr: String,
}

impl SmartSocketClient {
    pub fn new(addr: &str) -> Result<Self, SocketError> {
        Ok(Self {
            addr: addr.to_string(),
        })
    }

    pub fn toggle_power(&mut self) -> Result<String, SocketError> {
        let mut stream = TcpStream::connect(&self.addr)?;
        send_command(SocketCommand::Switch, &mut stream)?;

        let response = receive_response(&mut stream)?;
        Ok(response.trim().to_string())
    }

    pub fn get_status(&mut self) -> Result<String, SocketError> {
        let mut stream = TcpStream::connect(&self.addr)?;
        send_command(SocketCommand::Status, &mut stream)?;

        let response = receive_response(&mut stream)?;
        Ok(response.trim().to_string())
    }
}
