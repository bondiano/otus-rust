use std::io::Read;

use thiserror::Error;

pub const OK: &str = "ok\r\n";
pub const CLIENT_HANDSHAKE: &[u8] = b"clnt";
pub const SERVER_HANDSHAKE: &[u8] = b"serv";

pub trait ProtocolCommand {
    fn from_str(s: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn to_string(&self) -> String;
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unknown command: {0}\r\n")]
    UnknownCommand(String),
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Invalid command")]
    InvalidCommand,

    #[error("Could not send")]
    CouldNotSend,

    #[error("Invalid response")]
    InvalidResponse,

    #[error("Bad handshake")]
    BadHandshake,
}

pub fn read_till_rn<Reader: Read>(reader: &mut Reader) -> Result<String, ProtocolError> {
    let mut buffer = Vec::with_capacity(64);
    let mut last_byte = 0u8;

    loop {
        let mut byte = [0u8; 1];
        if reader.read_exact(&mut byte).is_err() {
            return Err(ProtocolError::InvalidResponse);
        }

        buffer.push(byte[0]);

        if last_byte == b'\r' && byte[0] == b'\n' {
            break;
        }
        last_byte = byte[0];
    }

    String::from_utf8(buffer).map_err(|_| ProtocolError::InvalidResponse)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn read_till_new_line_test() {
        let mut reader = Cursor::new(b"hello\r\nworld\r\n");
        let result = read_till_rn(&mut reader).expect("Failed to read");
        assert_eq!(result, "hello\r\n");
    }
}
