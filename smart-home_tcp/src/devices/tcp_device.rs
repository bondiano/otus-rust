use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use smart_home::devices::device::Device;
use thiserror::Error;

use crate::protocol::{ProtocolError, CLIENT_HANDSHAKE, SERVER_HANDSHAKE};

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Bad handshake")]
    BadHandshake,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn try_handshake(mut stream: TcpStream) -> Result<TcpStream, ConnectError> {
    let mut buf = [0; CLIENT_HANDSHAKE.len()];
    stream.read_exact(&mut buf)?;
    if buf != *CLIENT_HANDSHAKE {
        return Err(ConnectError::BadHandshake);
    }
    stream.write_all(SERVER_HANDSHAKE)?;
    Ok(stream)
}

pub trait TcpDevice<D: Device>: Sized {
    fn bind<Addrs>(device: D, addr: Addrs) -> Result<Self, std::io::Error>
    where
        Addrs: ToSocketAddrs;
    fn handle(&mut self, stream: &mut TcpStream) -> Result<(), ProtocolError>;
    fn get_listener(&self) -> TcpListener;

    fn accept(&self) -> Result<TcpStream, ConnectError> {
        let (stream, _) = self.get_listener().accept()?;
        println!("Accepted connection from: {}", stream.peer_addr()?);
        try_handshake(stream)
    }
}
