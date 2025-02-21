use std::{ffi::CStr, net::TcpStream};

use smart_home_tcp_client::{
    client::{receive_response, send_command},
    devices::socket::SocketCommand,
};

#[repr(u8)]
pub enum ReturnCode {
    Ok = 0,
    ConnectionError = 1,
    ParseError = 2,
    ProtocolError = 3,
}

#[no_mangle]
pub extern "C" fn toggle_power(addr: *const i8) -> ReturnCode {
    let addr = unsafe { CStr::from_ptr(addr).to_string_lossy() };
    let addr = addr.to_string();
    let mut stream = match TcpStream::connect(addr) {
        Ok(stream) => stream,
        Err(_) => return ReturnCode::ConnectionError,
    };
    if let Err(_) = send_command(SocketCommand::Switch, &mut stream) {
        return ReturnCode::ProtocolError;
    }

    let response = receive_response(&mut stream);
    if let Err(_) = response {
        return ReturnCode::ParseError;
    }

    ReturnCode::Ok
}

#[no_mangle]
pub extern "C" fn get_status(addr: *const i8) -> ReturnCode {
    let addr = unsafe { CStr::from_ptr(addr).to_string_lossy() };
    let addr = addr.to_string();
    let mut stream = match TcpStream::connect(addr) {
        Ok(stream) => stream,
        Err(_) => return ReturnCode::ConnectionError,
    };
    if let Err(_) = send_command(SocketCommand::Status, &mut stream) {
        return ReturnCode::ProtocolError;
    }

    let response = receive_response(&mut stream);
    if let Err(_) = response {
        return ReturnCode::ParseError;
    }

    ReturnCode::Ok
}
