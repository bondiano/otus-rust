use std::net::{ToSocketAddrs, UdpSocket};

use thiserror::Error;

pub const OK: &str = "ok\r\n";
pub const LENGTH_PREFIX_SIZE: usize = 4;
pub const MAX_MESSAGE_SIZE: u32 = 65507 - LENGTH_PREFIX_SIZE as u32; // Max UDP payload size minus prefix

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
    let mut buffer = [0; LENGTH_PREFIX_SIZE];
    socket_reader
        .recv_from(&mut buffer)
        .map_err(|_| ProtocolError::InvalidMessage("Could not read message length".to_string()))?;
    let len = u32::from_le_bytes(buffer);

    if len > MAX_MESSAGE_SIZE {
        return Err(ProtocolError::InvalidMessage(
            "Message too large".to_string(),
        ));
    }

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

    if len > MAX_MESSAGE_SIZE {
        return Err(ProtocolError::InvalidMessage(
            "Message too large".to_string(),
        ));
    }

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
    use std::net::UdpSocket;

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

    #[test]
    fn send_message_too_large_test() {
        let mut sender = UdpSocket::bind("127.0.0.1:0").expect("Could not bind sender socket");
        let receiver_addr = sender.local_addr().expect("Could not get sender address");

        let large_message = "a".repeat((MAX_MESSAGE_SIZE + 1) as usize);
        let result = send_message(&mut sender, receiver_addr, &large_message);
        assert!(matches!(result, Err(ProtocolError::InvalidMessage(_))));
    }

    #[test]
    fn receive_message_invalid_length_test() {
        let sender = UdpSocket::bind("127.0.0.1:0").expect("Could not bind sender socket");
        let mut receiver = UdpSocket::bind("127.0.0.1:0").expect("Could not bind receiver socket");
        let receiver_addr = receiver
            .local_addr()
            .expect("Could not get receiver address");

        // Send an invalid length prefix
        let invalid_length = u32::to_le_bytes(MAX_MESSAGE_SIZE + 1);
        sender
            .send_to(&invalid_length, receiver_addr)
            .expect("Could not send invalid length");

        let result = receive_message(&mut receiver);
        assert!(matches!(result, Err(ProtocolError::InvalidMessage(_))));
    }
}
