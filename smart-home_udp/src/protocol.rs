use std::net::{ToSocketAddrs, UdpSocket};

use thiserror::Error;

pub const OK: &str = "ok\r\n";

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Invalid command")]
    InvalidCommand,

    #[error("Could not send")]
    CouldNotSend,

    #[error("Invalid message: {0}")]
    InvalidMessage(String),
}

pub fn receive_message(socket_reader: &mut UdpSocket) -> Result<String, ProtocolError> {
    let mut buffer = [0; 4];
    socket_reader
        .recv_from(&mut buffer)
        .map_err(|_| ProtocolError::InvalidMessage("Could not read message length".to_string()))?;
    let len = u32::from_le_bytes(buffer);

    let mut buffer = vec![0; len as _];
    socket_reader
        .recv_from(&mut buffer)
        .map_err(|_| ProtocolError::InvalidMessage("Could not read message".to_string()))?;
    String::from_utf8(buffer)
        .map_err(|_| ProtocolError::InvalidMessage("Invalid message".to_string()))
}

pub fn send_message(
    socket: &mut UdpSocket,
    receiver: impl ToSocketAddrs,
    message: &str,
) -> Result<usize, ProtocolError> {
    let bytes = message.as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_le_bytes();

    socket
        .send_to(&len_bytes, &receiver)
        .map_err(|_| ProtocolError::CouldNotSend)?;
    socket
        .send_to(bytes, &receiver)
        .map_err(|_| ProtocolError::CouldNotSend)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_receive_message_test() {
        let mut sender = UdpSocket::bind("127.0.0.1:0").expect("Could not bind sender socket");
        let mut receiver = UdpSocket::bind("127.0.0.1:0").expect("Could not bind receiver socket");
        let receiver_addr = receiver
            .local_addr()
            .expect("Could not get receiver address");

        send_message(&mut sender, receiver_addr, "hello").expect("Could not send message");
        let message = receive_message(&mut receiver).expect("Invalid message");
        assert_eq!(message, "hello");
    }
}
