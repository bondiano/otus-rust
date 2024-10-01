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
        if self.rooms.contains_key(name) {
            return Err(SmartHouseError::RoomAlreadyExistsError(name.to_string()));
        }

        self.rooms.insert(name.to_owned(), Room::new(name));
        Ok(())
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), SmartHouseError> {
        if !self.rooms.contains_key(name) {
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

        let mut rooms = self.rooms.values().collect::<Vec<&Room>>();
        rooms.sort_by(|a, b| a.get_name().cmp(b.get_name()));

        for room in rooms {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::smart_socket::SmartSocket;
    use crate::report_provider::DeviceInfoProvider;

    struct TestDeviceInfoProvider;

    impl DeviceInfoProvider for TestDeviceInfoProvider {
        fn get_devices(&self) -> Vec<&str> {
            vec!["Socket"]
        }
    }

    #[test]
    fn test_house() {
        let mut house = House::new("House");
        assert_eq!(house.get_name(), "House");

        house.add_room("Room1").unwrap();
        house.add_room("Room2").unwrap();

        assert_eq!(house.get_rooms().count(), 2);

        house.remove_room("Room1").unwrap();
        assert_eq!(house.get_rooms().count(), 1);

        house.add_room("Room1").unwrap();
        house.add_room("Room3").unwrap();

        let room = house.get_room_mut("Room1").unwrap();
        room.add_device(SmartSocket::new("Socket", "A smart socket", 100).into())
            .unwrap();

        let report = house.create_report(&TestDeviceInfoProvider);
        assert_eq!(
            report,
            "House: House\nRoom: Room1\n  Device: Socket\n    Description: A smart socket\nRoom: Room2\nRoom: Room3\n"
        );
    }
}
