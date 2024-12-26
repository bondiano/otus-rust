use smart_home::devices::thermometer::SmartThermometer;
use std::{
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crate::protocol::{receive_message, send_message, ProtocolError};

use super::udp_device::UdpDevice;

#[derive(Debug)]
pub struct ThermometerSocketServer {
    receiver_adr: SocketAddr,
    device: SmartThermometer,
    temperature: Arc<Temperature>,
    finished: Arc<AtomicBool>,
}

impl UdpDevice<SmartThermometer> for ThermometerSocketServer {
    fn bind<Addrs: ToSocketAddrs>(
        device: SmartThermometer,
        receiver_addr: Addrs,
    ) -> Result<Self, std::io::Error> {
        let receiver_adr = receiver_addr
            .to_socket_addrs()?
            .next()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid receiver address",
            ))?;

        Ok(Self {
            receiver_adr,
            device,
            temperature: Arc::new(Temperature::default()),
            finished: Arc::new(AtomicBool::new(false)),
        })
    }

    fn handle(&mut self, data_stream: &mut UdpSocket) -> Result<(), ProtocolError> {
        let mut data_stream_clone = data_stream
            .try_clone()
            .map_err(|_| ProtocolError::CouldNotSend)?;
        let finished_clone = self.finished.clone();
        let temperature_clone = self.temperature.clone();
        let receiver_adr = self.receiver_adr;

        thread::spawn(move || loop {
            if finished_clone.load(Ordering::SeqCst) {
                return;
            }

            match receive_message(&mut data_stream_clone) {
                Ok(temperature) => {
                    if let Ok(temperature) = temperature.parse::<f32>() {
                        temperature_clone.set(temperature);
                    }
                }
                Err(error) => {
                    println!("Error receiving message: {:?}", error);
                }
            }

            if let Err(error) = send_message(
                &mut data_stream_clone,
                receiver_adr,
                &temperature_clone.get().to_string(),
            ) {
                println!("Error sending message: {:?}", error);
            }

            thread::sleep(Duration::from_secs(1));
        });

        Ok(())
    }
}

impl ThermometerSocketServer {
    pub fn get_temperature(&self) -> f32 {
        self.temperature.get()
    }

    pub fn get_device(&self) -> SmartThermometer {
        self.device.clone()
    }
}

impl Drop for ThermometerSocketServer {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst)
    }
}

#[derive(Default, Debug)]
struct Temperature(Mutex<f32>);

impl Temperature {
    pub fn get(&self) -> f32 {
        *self.0.lock().expect("Failed to lock mutex")
    }

    pub fn set(&self, val: f32) {
        *self.0.lock().expect("Failed to lock mutex") = val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature() {
        let temperature = Temperature::default();
        temperature.set(20.0);
        assert_eq!(temperature.get(), 20.0);
    }
}
