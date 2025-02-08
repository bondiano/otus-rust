use crate::devices::device::Device;

#[derive(Debug, Clone)]
pub struct SmartThermometer {
    name: String,
    description: String,
}

impl SmartThermometer {
    pub fn new(name: &str, description: &str) -> SmartThermometer {
        SmartThermometer {
            name: name.into(),
            description: description.into(),
        }
    }

    pub fn get_temperature(&self) -> f32 {
        todo!("Implement get_temperature() for SmartThermometer")
    }
}

impl Device for SmartThermometer {
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
    fn test_smart_thermometer() {
        let smart_thermometer = SmartThermometer::new("Thermometer", "A smart thermometer");
        assert_eq!(smart_thermometer.get_name(), "Thermometer");
        assert_eq!(smart_thermometer.get_description(), "A smart thermometer");
    }
}
