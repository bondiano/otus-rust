use std::{
    error::Error,
    net::UdpSocket,
    thread,
    time::{Duration, Instant},
};

use smart_home_udp_client::protocol::send_message;

fn main() -> Result<(), Box<dyn Error>> {
    let generator = TemperatureGenerator::default();
    let generator_address = "127.0.0.1:4323";
    let mut socket = UdpSocket::bind(generator_address)?;
    let receiver_address = "127.0.0.1:4322";

    loop {
        let temperature = generator.generate();
        println!("Sending temperature: {temperature}");
        if let Err(error) = send_message(&mut socket, receiver_address, &temperature.to_string()) {
            println!("Error sending message: {:?}", error);
        }
        thread::sleep(Duration::from_secs(1));
    }
}

struct TemperatureGenerator {
    started: Instant,
}

impl Default for TemperatureGenerator {
    fn default() -> Self {
        Self {
            started: Instant::now(),
        }
    }
}

impl TemperatureGenerator {
    pub fn generate(&self) -> f32 {
        let delay = Instant::now() - self.started;
        20.0 + (delay.as_secs_f32() / 2.0).sin()
    }
}
