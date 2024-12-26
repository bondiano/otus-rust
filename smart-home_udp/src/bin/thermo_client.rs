use std::{net::UdpSocket, thread, time::Duration};

use smart_home::devices::thermometer::SmartThermometer;
use smart_home_udp_client::devices::{thermometer::ThermometerSocketServer, udp_device::UdpDevice};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let receiver_address = "127.0.0.1:4321";
    let mut thermo = ThermometerSocketServer::bind(
        SmartThermometer::new("Thermometer", "A smart thermometer"),
        receiver_address,
    )?;

    let generator_address = "127.0.0.1:4322";
    let mut data_stream = UdpSocket::bind(generator_address)?;
    thermo.handle(&mut data_stream)?;

    for _ in 0..120 {
        thread::sleep(Duration::from_secs(1));
        let temperature = thermo.get_temperature();
        println!("The temperature is {temperature}");
    }

    Ok(())
}
