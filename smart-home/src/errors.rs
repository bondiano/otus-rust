#[derive(Debug)]
pub enum SmartHouseError {
    RoomAlreadyExistsError(String),
    RoomNotFoundError(String),
    DeviceAlreadyExistsError(String),
    DeviceNotFoundError(String),
}
