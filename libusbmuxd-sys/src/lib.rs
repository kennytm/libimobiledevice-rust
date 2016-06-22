//! Bindings to `libusbmuxd`.

#![allow(non_camel_case_types)]

pub mod proto;

use std::os::raw::{c_int, c_uint, c_char, c_void, c_ushort};

#[repr(C)]
pub struct usbmuxd_device_info_t {
    pub handle: u32,
    pub product_id: c_int,
    pub udid: [c_char; 41],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum usbmuxd_event_type {
    DeviceAdd = 1,
    DeviceRemove = 2,
}

pub const UE_DEVICE_ADD: usbmuxd_event_type = usbmuxd_event_type::DeviceAdd;
pub const UE_DEVICE_REMOVE: usbmuxd_event_type = usbmuxd_event_type::DeviceRemove;

#[repr(C)]
pub struct usbmuxd_event_t {
    pub event: c_int,
    pub device: usbmuxd_device_info_t,
}

pub type usbmuxd_event_cb_t = unsafe extern "C" fn(event: *const usbmuxd_event_t, user_data: *mut c_void);

extern "C" {
    pub fn usbmuxd_subscribe(callback: usbmuxd_event_cb_t, user_data: *mut c_void) -> c_int;
    pub fn usbmuxd_unsubscribe() -> c_int;

    pub fn usbmuxd_get_device_list(device_list: *mut *mut usbmuxd_device_info_t) -> c_int;
    pub fn usbmuxd_device_list_free(device_list: *mut *mut usbmuxd_device_info_t) -> c_int;

    pub fn usbmuxd_get_device_by_udid(udid: *const c_char, device: *mut usbmuxd_device_info_t) -> c_int;

    pub fn usbmuxd_connect(handle: c_int, tcp_port: c_ushort) -> c_int;
    pub fn usbmuxd_disconnect(sfd: c_int) -> c_int;

    pub fn usbmuxd_send(sfd: c_int, data: *const c_char, len: u32, sent_bytes: *mut u32) -> c_int;
    pub fn usbmuxd_recv_timeout(sfd: c_int, data: *mut c_char, len: u32, recv_bytes: *mut u32, timeout: c_uint) -> c_int;
    pub fn usbmuxd_recv(sfd: c_int, data: *mut c_char, len: u32, recv_bytes: *mut u32) -> c_int;

    pub fn usbmuxd_read_buid(buid: *mut *mut c_char) -> c_int;

    pub fn usbmuxd_read_pair_record(record_id: *const c_char, record_data: *mut *mut c_char, record_size: *mut u32) -> c_int;
    pub fn usbmuxd_save_pair_record(record_id: *const c_char, record_data: *const c_char, record_size: u32) -> c_int;
    pub fn usbmuxd_delete_pair_record(record_id: *const c_char) -> c_int;

    pub fn libusbmuxd_set_use_inotify(set: c_int);
    pub fn libusbmuxd_set_debug_level(level: c_int);
}

#[test]
fn test_validity() {
    unsafe {
        // Just to check if libusbmuxd is linked.
        libusbmuxd_set_debug_level(0);
    }
}

