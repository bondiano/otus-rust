use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum SmartHouseError {
    #[error("Room already exists: {0}")]
    #[diagnostic(code(smart_home::room_already_exists))]
    RoomAlreadyExistsError(String),

    #[error("Room not found: {0}")]
    #[diagnostic(code(smart_home::room_not_found))]
    RoomNotFoundError(String),

    #[error("Device already exists: {0}")]
    #[diagnostic(code(smart_home::device_already_exists))]
    DeviceAlreadyExistsError(String),

    #[error("Device not found: {0}")]
    #[diagnostic(code(smart_home::device_not_found))]
    DeviceNotFoundError(String),
}
