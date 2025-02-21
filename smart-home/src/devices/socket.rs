use crate::devices::device::Device;

#[derive(Debug, Clone)]
pub struct SmartSocket {
    power_consumption: u32,
    is_on: bool,
    description: String,
    name: String,
}

impl SmartSocket {
    pub fn new(name: &str, description: &str, power_consumption: u32) -> SmartSocket {
        SmartSocket {
            power_consumption,
            name: name.into(),
            description: description.into(),
            is_on: false,
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn switch(&mut self) {
        self.is_on = !self.is_on;
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn power_consumption(&self) -> u32 {
        self.power_consumption
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Device for SmartSocket {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_socket() {
        let mut smart_socket = SmartSocket::new("Socket", "A smart socket", 100);
        assert_eq!(smart_socket.get_name(), "Socket");
        assert_eq!(smart_socket.get_description(), "A smart socket");
        assert_eq!(smart_socket.power_consumption(), 100);
        assert!(!smart_socket.is_on());

        smart_socket.turn_on();
        assert!(smart_socket.is_on());

        smart_socket.turn_off();
        assert!(!smart_socket.is_on());
    }
}
