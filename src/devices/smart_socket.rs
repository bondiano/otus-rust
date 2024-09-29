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
