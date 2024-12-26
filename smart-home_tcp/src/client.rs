use std::io::{Read, Write};

use crate::protocol::{
    read_till_rn, ProtocolCommand, ProtocolError, CLIENT_HANDSHAKE, SERVER_HANDSHAKE,
};

fn send_handshake<Stream: Write + Read>(stream: &mut Stream) -> Result<(), ProtocolError> {
    stream
        .write_all(CLIENT_HANDSHAKE)
        .map_err(|_| ProtocolError::BadHandshake)?;
    stream.flush().map_err(|_| ProtocolError::BadHandshake)?;

    let mut buf = vec![0; SERVER_HANDSHAKE.len()];
    stream
        .read_exact(&mut buf)
        .map_err(|_| ProtocolError::BadHandshake)?;

    if buf != *SERVER_HANDSHAKE {
        return Err(ProtocolError::BadHandshake);
    }
    Ok(())
}

pub fn send_command<Stream: Write + Read>(
    command: impl ProtocolCommand,
    stream: &mut Stream,
) -> Result<(), ProtocolError> {
    send_handshake(stream)?;
    let binding = command.to_string();
    let bytes = binding.as_bytes();
    stream
        .write_all(bytes)
        .map_err(|_| ProtocolError::CouldNotSend)?;

    Ok(())
}

pub fn receive_response<Reader: Read>(reader: &mut Reader) -> Result<String, ProtocolError> {
    read_till_rn(reader).map_err(|_| ProtocolError::InvalidResponse)
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    use crate::{
        client::{receive_response, send_command},
        protocol::{ParseError, ProtocolCommand, CLIENT_HANDSHAKE, SERVER_HANDSHAKE},
    };

    #[derive(Debug)]
    enum TestCommand {
        Switch,
    }

    impl ProtocolCommand for TestCommand {
        fn from_str(_s: &str) -> Result<Self, ParseError> {
            Ok(TestCommand::Switch)
        }

        fn to_string(&self) -> String {
            "switch\r\n".to_owned()
        }
    }

    #[test]
    fn test_send() {
        use std::net::{TcpListener, TcpStream};
        use std::thread;

        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener");
        let addr = listener.local_addr().expect("Failed to get local address");

        let server_thread = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("Failed to accept connection");

            let mut buf = [0u8; 4];
            stream
                .read_exact(&mut buf)
                .expect("Failed to read handshake");
            assert_eq!(&buf, CLIENT_HANDSHAKE);

            stream
                .write_all(SERVER_HANDSHAKE)
                .expect("Failed to write handshake");

            let mut buf = [0u8; 8]; // "switch\r\n" is 8 bytes
            stream.read_exact(&mut buf).expect("Failed to read command");
            assert_eq!(&buf, b"switch\r\n");
        });

        // Client side
        let mut stream = TcpStream::connect(addr).expect("Failed to connect to server");
        let command = TestCommand::Switch;
        send_command(command, &mut stream).expect("Failed to send command");

        server_thread.join().expect("Failed to join server thread");
    }

    #[test]
    fn test_send_receive() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener");
        let addr = listener.local_addr().expect("Failed to get local address");

        // Spawn server thread
        let server_thread = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("Failed to accept connection");

            // Read client handshake
            let mut buf = [0u8; 4];
            stream
                .read_exact(&mut buf)
                .expect("Failed to read handshake");
            assert_eq!(&buf, CLIENT_HANDSHAKE);

            // Send server handshake
            stream
                .write_all(SERVER_HANDSHAKE)
                .expect("Failed to write handshake");

            // Read command
            let mut buf = [0u8; 8]; // "switch\r\n" is 8 bytes
            stream.read_exact(&mut buf).expect("Failed to read command");
            assert_eq!(&buf, b"switch\r\n");

            // Echo the command back as response
            stream
                .write_all(b"switch\r\n")
                .expect("Failed to write response");
        });

        // Client side
        let mut stream = TcpStream::connect(addr).expect("Failed to connect to server");
        let command = TestCommand::Switch;

        send_command(command, &mut stream).expect("Failed to send command");
        let response = receive_response(&mut stream).expect("Failed to receive response");
        assert_eq!(response, "switch\r\n");

        server_thread.join().expect("Failed to join server thread");
    }
}
