use std::{error::Error, fmt::Display, sync::Arc};

use smart_home::devices::socket::SmartSocket;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::Mutex,
};

#[derive(Clone, Debug)]
pub enum Command {
    Switch,
    Status,
    Unknown,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Switch,
            1 => Self::Status,
            _ => Self::Unknown,
        }
    }
}

impl From<Command> for u8 {
    fn from(command: Command) -> u8 {
        match command {
            Command::Switch => 0,
            Command::Status => 1,
            Command::Unknown => 255,
        }
    }
}

#[derive(Debug)]
pub enum Response {
    Ok,
    Enabled,
    Disabled,
    Error,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Ok => write!(f, "Ok"),
            Response::Enabled => write!(f, "Enabled"),
            Response::Disabled => write!(f, "Disabled"),
            Response::Error => write!(f, "Error"),
        }
    }
}

impl From<[u8; 1]> for Response {
    fn from(buffer: [u8; 1]) -> Self {
        match &buffer {
            [0, ..] => Self::Ok,
            [1, ..] => Self::Enabled,
            [2, ..] => Self::Disabled,
            _ => Self::Error,
        }
    }
}

impl From<Response> for [u8; 1] {
    fn from(response: Response) -> [u8; 1] {
        let mut buffer = [0u8; 1];
        match response {
            Response::Ok => {}
            Response::Enabled => buffer[0] = 1,
            Response::Disabled => buffer[0] = 2,
            Response::Error => buffer[0] = 255,
        };

        buffer
    }
}

pub struct SmartSocketClient {
    stream: TcpStream,
}

impl SmartSocketClient {
    pub async fn init(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(address).await?;

        Ok(Self { stream })
    }

    pub async fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()]).await?;
        let mut buffer = [0u8; 1];
        self.stream.read_exact(&mut buffer).await?;
        Ok(buffer.into())
    }
}

pub trait ExecCommand {
    fn exec_command(&mut self, command: Command) -> Response;
}

impl ExecCommand for SmartSocket {
    fn exec_command(&mut self, command: Command) -> Response {
        match command {
            Command::Switch => {
                self.switch();
                Response::Ok
            }
            Command::Status => {
                if self.is_on() {
                    Response::Enabled
                } else {
                    Response::Disabled
                }
            }
            Command::Unknown => Response::Error,
        }
    }
}

pub struct SmartSocketServer {
    listener: TcpListener,
    device: Arc<Mutex<SmartSocket>>,
}

impl SmartSocketServer {
    pub async fn init(
        address: impl ToSocketAddrs,
        name: &str,
        description: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(address).await?;
        let device = Arc::new(Mutex::new(SmartSocket::new(name, description, 0)));
        Ok(Self { listener, device })
    }

    pub async fn listen(&self) {
        while let Ok((mut stream, addr)) = self.listener.accept().await {
            let peer = addr.to_string();
            println!("Accepted connection from: {peer}");

            let smart_socket = self.device.clone();
            tokio::spawn(async move {
                let mut buffer = [0u8; 1];
                while stream.read_exact(&mut buffer).await.is_ok() {
                    let response = smart_socket.lock().await.exec_command(buffer[0].into());
                    let response_buffer: [u8; 1] = response.into();
                    if let Err(e) = stream.write_all(&response_buffer).await {
                        eprintln!("Failed to send response: {e}");
                        break;
                    }
                }

                println!("Connection with {peer} lost. Waiting for new connections...");
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;

    use super::*;

    #[test]
    fn test_exec_command_for_smart_socket() {
        let mut socket = SmartSocket::new("test", "test description", 0);

        // Test initial state (should be off)
        assert!(matches!(
            socket.exec_command(Command::Status),
            Response::Disabled
        ));

        // Test switch command
        assert!(matches!(socket.exec_command(Command::Switch), Response::Ok));
        assert!(matches!(
            socket.exec_command(Command::Status),
            Response::Enabled
        ));

        // Test switch again
        assert!(matches!(socket.exec_command(Command::Switch), Response::Ok));
        assert!(matches!(
            socket.exec_command(Command::Status),
            Response::Disabled
        ));

        // Test unknown command
        assert!(matches!(
            socket.exec_command(Command::Unknown),
            Response::Error
        ));
    }

    #[tokio::test]
    async fn test_client_server_interaction() {
        const SERVER_ADDR: &str = "127.0.0.1:8080";

        let server = SmartSocketServer::init(SERVER_ADDR, "test_socket", "test description")
            .await
            .expect("Failed to initialize server");

        tokio::spawn(async move {
            server.listen().await;
        });

        // Give server time to start
        sleep(Duration::from_millis(100)).await;

        let mut client = SmartSocketClient::init(SERVER_ADDR)
            .await
            .expect("Failed to initialize client");

        let response = client
            .run_command(Command::Status)
            .await
            .expect("Failed to run command");
        assert!(matches!(response, Response::Disabled));

        let response = client
            .run_command(Command::Switch)
            .await
            .expect("Failed to run command");
        assert!(matches!(response, Response::Ok));

        let response = client
            .run_command(Command::Status)
            .await
            .expect("Failed to run command");
        assert!(matches!(response, Response::Enabled));

        let response = client
            .run_command(Command::Switch)
            .await
            .expect("Failed to run command");
        assert!(matches!(response, Response::Ok));

        let response = client
            .run_command(Command::Status)
            .await
            .expect("Failed to run command");
        assert!(matches!(response, Response::Disabled));

        let response = client
            .run_command(Command::Unknown)
            .await
            .expect("Failed to run command");
        assert!(matches!(response, Response::Error));
    }
}
