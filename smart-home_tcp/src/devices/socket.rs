use smart_home::devices::socket::SmartSocket;
use std::{
    io::Write,
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::protocol::{read_till_rn, ParseError, ProtocolCommand, ProtocolError, OK};

use super::tcp_device::TcpDevice;

#[derive(Clone, Debug)]
pub enum SocketCommand {
    Switch,
    Status,
}

impl ProtocolCommand for SocketCommand {
    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s.trim() {
            "switch" => Ok(SocketCommand::Switch),
            "status" => Ok(SocketCommand::Status),
            other => Err(ParseError::UnknownCommand(other.to_owned())),
        }
    }

    fn to_string(&self) -> String {
        match self {
            SocketCommand::Switch => "switch\r\n".to_owned(),
            SocketCommand::Status => "status\r\n".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct SocketServer {
    listener: TcpListener,
    device: SmartSocket,
}

impl TcpDevice<SmartSocket> for SocketServer {
    fn bind<Addrs: ToSocketAddrs>(
        device: SmartSocket,
        addr: Addrs,
    ) -> Result<SocketServer, std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        Ok(SocketServer { listener, device })
    }

    fn get_listener(&self) -> TcpListener {
        self.listener.try_clone().expect("Failed to clone listener")
    }

    fn handle(&mut self, stream: &mut TcpStream) -> Result<(), ProtocolError> {
        let command = read_till_rn(stream).map_err(|_| ProtocolError::InvalidResponse)?;
        println!("Received command: {}", command);

        let command =
            SocketCommand::from_str(&command).map_err(|_| ProtocolError::InvalidCommand)?;

        let result = match command {
            SocketCommand::Switch => {
                self.device.switch();
                OK.to_owned()
            }
            SocketCommand::Status => {
                let status = if self.device.is_on() { "on" } else { "off" };
                format!("{}\r\n", status)
            }
        };

        stream
            .write_all(result.as_bytes())
            .map_err(|_| ProtocolError::CouldNotSend)?;

        Ok(())
    }
}
