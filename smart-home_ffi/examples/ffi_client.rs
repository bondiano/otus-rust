use libloading::{Library, Symbol};

#[repr(u8)]
#[derive(Debug)]
pub enum ReturnCode {
    Ok = 0,
    ConnectionError = 1,
    ParseError = 2,
    ProtocolError = 3,
}

type TogglePowerFn = unsafe extern "C" fn(addr: *const i8) -> ReturnCode;
type GetStatusFn = unsafe extern "C" fn(addr: *const i8) -> ReturnCode;

fn main() {
    let lib = unsafe { Library::new("../target/debug/libsmart_home_ffi.dylib") }
        .expect("Failed to load library");
    let toggle_power: Symbol<'_, TogglePowerFn> =
        unsafe { lib.get(b"toggle_power") }.expect("Failed to get toggle_power");
    let get_status: Symbol<'_, GetStatusFn> =
        unsafe { lib.get(b"get_status") }.expect("Failed to get get_status");

    let addr = "127.0.0.1:55331\0";
    let addr_ptr = addr.as_ptr() as *const i8;

    let result = unsafe { toggle_power(addr_ptr) };
    println!("Toggle power result: {:?}", result);

    let result = unsafe { get_status(addr_ptr) };
    println!("Get status result: {:?}", result);
}
