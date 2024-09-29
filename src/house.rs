use std::collections::HashMap;

use crate::{
    devices::device::Device, errors::SmartHouseError, report_provider::DeviceInfoProvider,
    room::Room,
};

#[derive(Debug)]
pub struct House {
    name: String,
    rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_room(&mut self, name: &str) -> Result<(), SmartHouseError> {
        if self.rooms.get(name).is_some() {
            return Err(SmartHouseError::RoomAlreadyExistsError(name.to_string()));
        }

        self.rooms.insert(name.to_owned(), Room::new(name));
        Ok(())
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), SmartHouseError> {
        if self.rooms.get(name).is_none() {
            return Err(SmartHouseError::RoomNotFoundError(name.to_string()));
        }

        self.rooms.remove(name);
        Ok(())
    }

    pub fn get_rooms(&self) -> impl Iterator<Item = &Room> {
        self.rooms.iter().map(|kv| kv.1)
    }

    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub fn create_report(&self, device_info_provider: &impl DeviceInfoProvider) -> String {
        let mut report = format!("House: {}\n", self.name);
        let devices = device_info_provider.get_devices();

        for room in self.rooms.values() {
            report.push_str(&format!("Room: {}\n", room.get_name()));
            for device in room.get_devices() {
                if devices.contains(&device.get_name()) {
                    report.push_str(&format!("  Device: {}\n", device.get_name()));
                    report.push_str(&format!("    Description: {}\n", device.get_description()));
                }
            }
        }

        report
    }
}
