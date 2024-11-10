use std::fmt::Debug;
use std::{collections::HashMap, fmt};

use crate::{
    devices::{device::Device, socket::SmartSocket, thermometer::SmartThermometer},
    errors::SmartHouseError,
};

#[derive(Debug)]
pub enum RoomDevice {
    SmartSocket(SmartSocket),
    Thermometer(SmartThermometer),
}

impl Device for RoomDevice {
    fn get_name(&self) -> &str {
        match self {
            RoomDevice::Thermometer(t) => t.get_name(),
            RoomDevice::SmartSocket(s) => s.get_name(),
        }
    }

    fn get_description(&self) -> &str {
        match self {
            RoomDevice::Thermometer(t) => t.get_description(),
            RoomDevice::SmartSocket(s) => s.get_description(),
        }
    }
}

impl From<SmartSocket> for RoomDevice {
    fn from(socket: SmartSocket) -> Self {
        RoomDevice::SmartSocket(socket)
    }
}

impl From<SmartThermometer> for RoomDevice {
    fn from(thermometer: SmartThermometer) -> Self {
        RoomDevice::Thermometer(thermometer)
    }
}

pub struct Room {
    name: String,
    devices: HashMap<String, RoomDevice>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_device(&mut self, device: RoomDevice) -> Result<(), SmartHouseError> {
        if self.devices.contains_key(device.get_name()) {
            return Err(SmartHouseError::DeviceAlreadyExistsError(
                device.get_name().to_string(),
            ));
        }

        self.devices.insert(device.get_name().to_owned(), device);
        Ok(())
    }

    pub fn remove_device(&mut self, name: &str) -> Result<(), SmartHouseError> {
        if !self.devices.contains_key(name) {
            return Err(SmartHouseError::DeviceNotFoundError(name.to_string()));
        }

        self.devices.remove(name);
        Ok(())
    }

    pub fn get_devices(&self) -> impl Iterator<Item = &RoomDevice> {
        let mut devices = self.devices.values().collect::<Vec<&RoomDevice>>();
        devices.sort_by(|a, b| a.get_name().cmp(b.get_name()));
        devices.into_iter()
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let devices: Vec<String> = self
            .devices
            .values()
            .map(|device| device.get_name().to_string())
            .collect();
        f.debug_struct("Room")
            .field("name", &self.name)
            .field("devices", &devices)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_device() {
        let mut room = Room::new("bedroom");

        let name = "socket near the bed";
        let socket = SmartSocket::new(name, "Smart socket", 1);
        room.add_device(socket.into()).unwrap();
        assert!(room.get_devices().count() == 1);

        room.remove_device(name).unwrap();
        assert!(room.get_devices().count() == 0);
    }
}
