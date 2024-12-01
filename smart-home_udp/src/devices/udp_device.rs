use std::net::{ToSocketAddrs, UdpSocket};

use smart_home::devices::device::Device;

use crate::protocol::ProtocolError;

pub trait UdpDevice<D: Device>: Sized {
    fn bind<Addrs>(device: D, data_addr: Addrs) -> Result<Self, std::io::Error>
    where
        Addrs: ToSocketAddrs;
    fn handle(&mut self, stream: &mut UdpSocket) -> Result<(), ProtocolError>;
}
