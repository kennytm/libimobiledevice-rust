//! Bindings to `diagnostics_relay.h`.

use idevice::idevice_t;
use lockdown::lockdownd_service_descriptor_t;
use libplist_sys::plist_t;

use std::os::raw::{c_char, c_void, c_int};

pub const DIAGNOSTICS_RELAY_SERVICE_NAME: &'static [u8] = b"com.apple.mobile.diagnostics_relay\0";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum diagnostics_relay_error_t {
    Success = 0,
    InvalidArg = -1,
    PlistError = -2,
    MuxError = -3,
    UnknownRequest = -4,
    UnknownError = -256,
}

pub const DIAGNOSTICS_RELAY_E_SUCCESS: diagnostics_relay_error_t = diagnostics_relay_error_t::Success;
pub const DIAGNOSTICS_RELAY_E_INVALID_ARG: diagnostics_relay_error_t = diagnostics_relay_error_t::InvalidArg;
pub const DIAGNOSTICS_RELAY_E_PLIST_ERROR: diagnostics_relay_error_t = diagnostics_relay_error_t::PlistError;
pub const DIAGNOSTICS_RELAY_E_MUX_ERROR: diagnostics_relay_error_t = diagnostics_relay_error_t::MuxError;
pub const DIAGNOSTICS_RELAY_E_UNKNOWN_REQUEST: diagnostics_relay_error_t = diagnostics_relay_error_t::UnknownRequest;
pub const DIAGNOSTICS_RELAY_E_UNKNOWN_ERROR: diagnostics_relay_error_t = diagnostics_relay_error_t::UnknownError;

pub const DIAGNOSTICS_RELAY_ACTION_FLAG_WAIT_FOR_DISCONNECT: c_int = 2;
pub const DIAGNOSTICS_RELAY_ACTION_FLAG_DISPLAY_PASS: c_int = 4;
pub const DIAGNOSTICS_RELAY_ACTION_FLAG_DISPLAY_FAIL: c_int = 8;

pub const DIAGNOSTICS_RELAY_REQUEST_TYPE_ALL: &'static [u8] = b"All\0";
pub const DIAGNOSTICS_RELAY_REQUEST_TYPE_WIFI: &'static [u8] = b"WiFi\0";
pub const DIAGNOSTICS_RELAY_REQUEST_TYPE_GAS_GAUGE: &'static [u8] = b"GasGauge\0";
pub const DIAGNOSTICS_RELAY_REQUEST_TYPE_NAND: &'static [u8] = b"NAND\0";

#[repr(C)]
#[doc(hidden)]
pub struct diagnostics_relay_client_private(c_void);
pub type diagnostics_relay_client_t = *mut diagnostics_relay_client_private;

extern "C" {
    pub fn diagnostics_relay_client_new(device: idevice_t, service: lockdownd_service_descriptor_t, client: *mut diagnostics_relay_client_t) -> diagnostics_relay_error_t;
    pub fn diagnostics_relay_client_start_service(device: idevice_t, client: *mut diagnostics_relay_client_t, label: *const c_char) -> diagnostics_relay_error_t;
    pub fn diagnostics_relay_client_free(client: diagnostics_relay_client_t) -> diagnostics_relay_error_t;

    pub fn diagnostics_relay_goodbye(client: diagnostics_relay_client_t) -> diagnostics_relay_error_t;
    pub fn diagnostics_relay_sleep(client: diagnostics_relay_client_t) -> diagnostics_relay_error_t;
    pub fn diagnostics_relay_restart(client: diagnostics_relay_client_t, flags: c_int) -> diagnostics_relay_error_t;
    pub fn diagnostics_relay_shutdown(client: diagnostics_relay_client_t, flags: c_int) -> diagnostics_relay_error_t;

    pub fn diagnostics_relay_request_diagnostics(client: diagnostics_relay_client_t, type_: *const c_char, diagnostics: *mut plist_t) -> diagnostics_relay_error_t;
    pub fn diagnostics_relay_query_mobilegestalt(client: diagnostics_relay_client_t, keys: plist_t, result: *mut plist_t) -> diagnostics_relay_error_t;
    pub fn diagnostics_relay_query_ioregistry_entry(client: diagnostics_relay_client_t, name: *const c_char, class: *const c_char, result: *mut plist_t) -> diagnostics_relay_error_t;
    pub fn diagnostics_relay_query_ioregistry_plane(client: diagnostics_relay_client_t, plane: *const c_char, result: *mut plist_t) -> diagnostics_relay_error_t;
}

