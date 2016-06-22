//! Bindings to `libimobiledevice.h`.

use std::os::raw::{c_void, c_char, c_int, c_uint};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum idevice_error_t {
    Success = 0,
    InvalidArg = -1,
    UnknownError = -2,
    NoDevice = -3,
    NotEnoughData = -4,
    BadHeader = -5,
    SslError = -6,
}

pub const IDEVICE_E_SUCCESS: idevice_error_t = idevice_error_t::Success;
pub const IDEVICE_E_INVALID_ARG: idevice_error_t = idevice_error_t::InvalidArg;
pub const IDEVICE_E_UNKNOWN_ERROR: idevice_error_t = idevice_error_t::UnknownError;
pub const IDEVICE_E_NO_DEVICE: idevice_error_t = idevice_error_t::NoDevice;
pub const IDEVICE_E_NOT_ENOUGH_DATA: idevice_error_t = idevice_error_t::NotEnoughData;
pub const IDEVICE_E_BAD_HEADER: idevice_error_t = idevice_error_t::BadHeader;
pub const IDEVICE_E_SSL_ERROR: idevice_error_t = idevice_error_t::SslError;

#[doc(hidden)]
pub struct idevice_private(c_void);
pub type idevice_t = *mut idevice_private;

#[doc(hidden)]
pub struct idevice_connection_private(c_void);
pub type idevice_connection_t = *mut idevice_connection_private;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum idevice_event_type {
    DeviceAdd = 1,
    DeviceRemove = 2,
}

pub const IDEVICE_DEVICE_ADD: idevice_event_type = idevice_event_type::DeviceAdd;
pub const IDEVICE_DEVICE_REMOVE: idevice_event_type = idevice_event_type::DeviceRemove;

#[repr(C)]
pub struct idevice_event_t {
    pub event: idevice_event_type,
    pub udid: *const c_char,
    pub conn_type: c_int,
}

pub type idevice_event_cb_t = unsafe extern "C" fn(event: *const idevice_event_t, user_data: *mut c_void);

extern "C" {
    pub fn idevice_set_debug_level(level: c_int);

    pub fn idevice_event_subscribe(callback: idevice_event_cb_t, user_data: *mut c_void) -> idevice_error_t;
    pub fn idevice_event_unsubscribe() -> idevice_error_t;

    pub fn idevice_get_device_list(devices: *mut *mut *mut c_char, count: *mut c_int) -> idevice_error_t;
    pub fn idevice_device_list_free(devices: *mut *mut c_char) -> idevice_error_t;

    pub fn idevice_new(device: *mut idevice_t, udid: *const c_char) -> idevice_error_t;
    pub fn idevice_free(device: idevice_t) -> idevice_error_t;

    pub fn idevice_connect(device: idevice_t, port: u16, connection: *mut idevice_connection_t) -> idevice_error_t;
    pub fn idevice_disconnect(connection: idevice_connection_t) -> idevice_error_t;

    pub fn idevice_connection_send(connection: idevice_connection_t, data: *const c_char, len: u32, sent_bytes: *mut u32) -> idevice_error_t;
    pub fn idevice_connection_receive_timeout(connection: idevice_connection_t, data: *mut c_char, len: u32, recv_bytes: *mut u32, timeout: c_uint) -> idevice_error_t;
    pub fn idevice_connection_receive(connection: idevice_connection_t, data: *mut c_char, len: u32, recv_bytes: *mut u32) -> idevice_error_t;

    pub fn idevice_connection_enable_ssl(connection: idevice_connection_t) -> idevice_error_t;
    pub fn idevice_connection_disable_ssl(connection: idevice_connection_t) -> idevice_error_t;

    pub fn idevice_get_handle(device: idevice_t, handle: *mut u32) -> idevice_error_t;
    pub fn idevice_get_udid(device: idevice_t, udid: *mut *mut c_char) -> idevice_error_t;
}


