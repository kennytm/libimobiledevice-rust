//! Structures of the `usbmuxd` protocol.

use std::os::raw::c_char;

pub const USBMUXD_PROTOCOL_VERSION: i32 = 0;

pub const USBMUXD_SOCKET_PORT: u16 = 27015;
pub const USBMUXD_SOCKET_FILE: &'static str = "/var/run/usbmuxd";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum usbmuxd_result {
    Ok = 0,
    BadCommand = 1,
    BadDev = 2,
    ConnRefused = 3,
    BadVersion = 6,
}

pub const RESULT_OK: usbmuxd_result = usbmuxd_result::Ok;
pub const RESULT_BADCOMMAND: usbmuxd_result = usbmuxd_result::BadCommand;
pub const RESULT_BADDEV: usbmuxd_result = usbmuxd_result::BadDev;
pub const RESULT_CONNREFUSED: usbmuxd_result = usbmuxd_result::ConnRefused;
pub const RESULT_BADVERSION: usbmuxd_result = usbmuxd_result::BadVersion;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum usbmuxd_msgtype {
    Result = 1,
    Connect = 2,
    Listen = 3,
    DeviceAdd = 4,
    DeviceRemove = 5,
    Plist = 8,
}

pub const MESSAGE_RESULT: usbmuxd_msgtype = usbmuxd_msgtype::Result;
pub const MESSAGE_CONNECT: usbmuxd_msgtype = usbmuxd_msgtype::Connect;
pub const MESSAGE_LISTEN: usbmuxd_msgtype = usbmuxd_msgtype::Listen;
pub const MESSAGE_DEVICE_ADD: usbmuxd_msgtype = usbmuxd_msgtype::DeviceAdd;
pub const MESSAGE_DEVICE_REMOVE: usbmuxd_msgtype = usbmuxd_msgtype::DeviceRemove;
pub const MESSAGE_PLIST: usbmuxd_msgtype = usbmuxd_msgtype::Plist;

#[repr(packed)]
pub struct usbmuxd_header {
    pub length: u32,
    pub version: u32,
    pub message: u32,
    pub tag: u32,
}

#[repr(packed)]
pub struct usbmuxd_result_msg {
    pub header: usbmuxd_header,
    pub result: u32,
}

#[repr(packed)]
pub struct usbmuxd_connect_request {
    pub header: usbmuxd_header,
    pub device_id: u32,
    pub port: u16,
    pub _reserved: u16,
}

#[repr(packed)]
pub struct usbmuxd_listen_request {
    pub header: usbmuxd_header,
}

#[repr(packed)]
pub struct usbmuxd_device_record {
    pub device_id: u32,
    pub product_id: u16,
    pub serial_number: [c_char; 256],
    pub _padding: u16,
    pub location: u32,
}

