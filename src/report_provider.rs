pub trait DeviceInfoProvider {
    fn get_devices(&self) -> Vec<&str>;
}
