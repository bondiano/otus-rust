use std::{
    error::Error,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    sync::Mutex,
    time::{self, Instant},
};

#[derive(Debug, Clone)]
pub struct SmartThermoClient {
    temperature: Arc<Temperature>,
    finished: Arc<AtomicBool>,
    socket: Arc<UdpSocket>,
}

impl SmartThermoClient {
    pub async fn init(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = Arc::new(UdpSocket::bind(address).await?);
        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Temperature::default());

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();
        let socket_clone = socket.clone();

        let timeout = Duration::from_secs(1);
        tokio::spawn(async move {
            loop {
                if finished_clone.load(Ordering::SeqCst) {
                    return;
                }

                let mut buf = [0; 4];
                if let Err(e) = time::timeout(timeout, socket_clone.recv_from(&mut buf)).await {
                    println!("Can't receive datagram: {e}");
                    continue;
                }

                let val = f32::from_be_bytes(buf);
                temperature_clone.set(val).await;
            }
        });

        Ok(Self {
            temperature,
            finished,
            socket,
        })
    }

    pub async fn get_temperature(&self) -> f32 {
        self.temperature.get().await
    }

    pub fn get_socket_address(&self) -> Result<SocketAddr, std::io::Error> {
        self.socket.local_addr()
    }
}

impl Drop for SmartThermoClient {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst);
    }
}

#[derive(Debug, Default)]
struct Temperature(Mutex<f32>);

impl Temperature {
    pub async fn get(&self) -> f32 {
        *self.0.lock().await
    }
    pub async fn set(&self, value: f32) {
        *self.0.lock().await = value;
    }
}

#[derive(Debug)]
pub struct TemperatureGeneratorServer {
    listener: UdpSocket,
    started: Instant,
}

impl TemperatureGeneratorServer {
    pub async fn init(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let listener = UdpSocket::bind(address).await?;

        Ok(Self {
            listener,
            started: Instant::now(),
        })
    }

    pub async fn listen(&self, receiver: &str) {
        let receiver = receiver
            .parse::<SocketAddr>()
            .expect("valid socket address expected");

        loop {
            let temperature = self.generate();
            let bytes = temperature.to_be_bytes();
            let send_result = self.listener.send_to(&bytes, receiver).await;

            if let Err(e) = send_result {
                println!("Can't send temperature: {e}");
            }

            let duration = Duration::from_secs_f32(0.5);
            time::sleep(duration).await;
        }
    }

    fn generate(&self) -> f32 {
        let delay = Instant::now() - self.started;
        20.0 + (delay.as_secs_f32() / 2.0).sin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_temperature_struct() {
        let temp = Temperature::default();
        assert_eq!(temp.get().await, 0.0);

        temp.set(25.5).await;
        assert_eq!(temp.get().await, 25.5);
    }

    #[tokio::test]
    async fn test_temperature_generator() {
        let server = TemperatureGeneratorServer::init("127.0.0.1:0")
            .await
            .expect("Failed to initialize server");

        let temp = server.generate();
        assert!((19.5..21.5).contains(&temp));
    }

    #[tokio::test]
    async fn test_client_server_communication() {
        let server = TemperatureGeneratorServer::init("127.0.0.1:0")
            .await
            .expect("Failed to initialize server");

        // Start client with system-assigned port
        let client = SmartThermoClient::init("127.0.0.1:0")
            .await
            .expect("Failed to initialize client");
        let client_addr = client.get_socket_address().expect("invalid address");

        // Spawn server task using the client's assigned address
        tokio::spawn(async move {
            server.listen(&client_addr.to_string()).await;
        });

        sleep(Duration::from_secs(2)).await;
        let temp = client.get_temperature().await;
        assert!((19.5..=21.5).contains(&temp));
    }
}
