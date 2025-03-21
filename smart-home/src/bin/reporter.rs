use smart_home::{
    devices::{device::Device, socket::SmartSocket, thermometer::SmartThermometer},
    errors::SmartHouseError,
    house::House,
    report_provider::DeviceInfoProvider,
};

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_devices(&self) -> Vec<&str> {
        vec![self.socket.get_name()]
    }
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_devices(&self) -> Vec<&str> {
        vec![self.socket.get_name(), self.thermo.get_name()]
    }
}

fn main() -> miette::Result<()> {
    let mut house = House::new("sweet home");
    house.add_room("bedroom")?;
    house.add_room("kitchen")?;
    house.add_room("bathroom")?;

    let bedroom = house
        .get_room_mut("bedroom")
        .ok_or(SmartHouseError::RoomNotFoundError("bedroom".to_string()))?;

    let socket1 = SmartSocket::new("Socket 1", "Smart socket 1", 1);
    bedroom.add_device(socket1.clone().into())?;

    let kitchen = house
        .get_room_mut("kitchen")
        .ok_or(SmartHouseError::RoomNotFoundError("kitchen".to_string()))?;
    let socket2 = SmartSocket::new("Socket 2", "Smart socket 2", 2);
    kitchen.add_device(socket2.clone().into())?;
    let thermo = SmartThermometer::new("Thermo 1", "Smart thermometer 1");
    kitchen.add_device(thermo.clone().into())?;

    println!("House: {:?}", house);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");

    Ok(())
}
