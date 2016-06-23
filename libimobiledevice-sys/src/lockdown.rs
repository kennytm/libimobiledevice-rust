//! Bindings to `lockdown.h`.

use std::os::raw::{c_void, c_char, c_int};
use idevice::{idevice_t};
use libplist_sys::plist_t;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum lockdownd_error_t {
    // custom
    Success = 0,
    InvalidArg = -1,
    InvalidConf = -2,
    PlistError = -3,
    PairingFailed = -4,
    SslError = -5,
    DictError = -6,
    NotEnoughData = -7,
    MuxError = -8,
    NoRunningSession = -9,

    // native
    InvalidResponse = -10,
    MissingKey = -11,
    MissingValue = -12,
    GetProhibited = -13,
    SetProhibited = -14,
    RemoveProhibited = -15,
    ImmutableValue = -16,
    PasswordProtected = -17,
    UserDeniedPairing = -18,
    PairingDialogResponsePending = -19,
    MissingHostId = -20,
    InvalidHostId = -21,
    SessionActive = -22,
    SessionInactive = -23,
    MissingSessionId = -24,
    InvalidSessionId = -25,
    MissingService = -26,
    InvalidService = -27,
    ServiceLimit = -28,
    MissingPairRecord = -29,
    SavePairRecordFailed = -30,
    InvalidPairRecord = -31,
    InvalidActivationRecord = -32,
    MissingActivationRecord = -33,
    ServiceProhibited = -34,
    EscrowLocked = -35,
    UnknownError = -256,
}

pub const LOCKDOWN_E_SUCCESS: lockdownd_error_t = lockdownd_error_t::Success;
pub const LOCKDOWN_E_INVALID_ARG: lockdownd_error_t = lockdownd_error_t::InvalidArg;
pub const LOCKDOWN_E_INVALID_CONF: lockdownd_error_t = lockdownd_error_t::InvalidConf;
pub const LOCKDOWN_E_PLIST_ERROR: lockdownd_error_t = lockdownd_error_t::PlistError;
pub const LOCKDOWN_E_PAIRING_FAILED: lockdownd_error_t = lockdownd_error_t::PairingFailed;
pub const LOCKDOWN_E_SSL_ERROR: lockdownd_error_t = lockdownd_error_t::SslError;
pub const LOCKDOWN_E_DICT_ERROR: lockdownd_error_t = lockdownd_error_t::DictError;
pub const LOCKDOWN_E_NOT_ENOUGH_DATA: lockdownd_error_t = lockdownd_error_t::NotEnoughData;
pub const LOCKDOWN_E_MUX_ERROR: lockdownd_error_t = lockdownd_error_t::MuxError;
pub const LOCKDOWN_E_NO_RUNNING_SESSION: lockdownd_error_t = lockdownd_error_t::NoRunningSession;
pub const LOCKDOWN_E_INVALID_RESPONSE: lockdownd_error_t = lockdownd_error_t::InvalidResponse;
pub const LOCKDOWN_E_MISSING_KEY: lockdownd_error_t = lockdownd_error_t::MissingKey;
pub const LOCKDOWN_E_MISSING_VALUE: lockdownd_error_t = lockdownd_error_t::MissingValue;
pub const LOCKDOWN_E_GET_PROHIBITED: lockdownd_error_t = lockdownd_error_t::GetProhibited;
pub const LOCKDOWN_E_SET_PROHIBITED: lockdownd_error_t = lockdownd_error_t::SetProhibited;
pub const LOCKDOWN_E_REMOVE_PROHIBITED: lockdownd_error_t = lockdownd_error_t::RemoveProhibited;
pub const LOCKDOWN_E_IMMUTABLE_VALUE: lockdownd_error_t = lockdownd_error_t::ImmutableValue;
pub const LOCKDOWN_E_PASSWORD_PROTECTED: lockdownd_error_t = lockdownd_error_t::PasswordProtected;
pub const LOCKDOWN_E_USER_DENIED_PAIRING: lockdownd_error_t = lockdownd_error_t::UserDeniedPairing;
pub const LOCKDOWN_E_PAIRING_DIALOG_RESPONSE_PENDING: lockdownd_error_t = lockdownd_error_t::PairingDialogResponsePending;
pub const LOCKDOWN_E_MISSING_HOST_ID: lockdownd_error_t = lockdownd_error_t::MissingHostId;
pub const LOCKDOWN_E_INVALID_HOST_ID: lockdownd_error_t = lockdownd_error_t::InvalidHostId;
pub const LOCKDOWN_E_SESSION_ACTIVE: lockdownd_error_t = lockdownd_error_t::SessionActive;
pub const LOCKDOWN_E_SESSION_INACTIVE: lockdownd_error_t = lockdownd_error_t::SessionInactive;
pub const LOCKDOWN_E_MISSING_SESSION_ID: lockdownd_error_t = lockdownd_error_t::MissingSessionId;
pub const LOCKDOWN_E_INVALID_SESSION_ID: lockdownd_error_t = lockdownd_error_t::InvalidSessionId;
pub const LOCKDOWN_E_MISSING_SERVICE: lockdownd_error_t = lockdownd_error_t::MissingService;
pub const LOCKDOWN_E_INVALID_SERVICE: lockdownd_error_t = lockdownd_error_t::InvalidService;
pub const LOCKDOWN_E_SERVICE_LIMIT: lockdownd_error_t = lockdownd_error_t::ServiceLimit;
pub const LOCKDOWN_E_MISSING_PAIR_RECORD: lockdownd_error_t = lockdownd_error_t::MissingPairRecord;
pub const LOCKDOWN_E_SAVE_PAIR_RECORD_FAILED: lockdownd_error_t = lockdownd_error_t::SavePairRecordFailed;
pub const LOCKDOWN_E_INVALID_PAIR_RECORD: lockdownd_error_t = lockdownd_error_t::InvalidPairRecord;
pub const LOCKDOWN_E_INVALID_ACTIVATION_RECORD: lockdownd_error_t = lockdownd_error_t::InvalidActivationRecord;
pub const LOCKDOWN_E_MISSING_ACTIVATION_RECORD: lockdownd_error_t = lockdownd_error_t::MissingActivationRecord;
pub const LOCKDOWN_E_SERVICE_PROHIBITED: lockdownd_error_t = lockdownd_error_t::ServiceProhibited;
pub const LOCKDOWN_E_ESCROW_LOCKED: lockdownd_error_t = lockdownd_error_t::EscrowLocked;
pub const LOCKDOWN_E_UNKNOWN_ERROR: lockdownd_error_t = lockdownd_error_t::UnknownError;

#[doc(hidden)]
#[repr(C)]
pub struct lockdownd_client_private(c_void);
pub type lockdownd_client_t = *mut lockdownd_client_private;

#[repr(C)]
pub struct lockdownd_pair_record {
    pub device_certificate: *mut c_char,
    pub host_certificate: *mut c_char,
    pub root_certificate: *mut c_char,
    pub host_id: *mut c_char,
    pub system_buid: *mut c_char,
}
pub type lockdownd_pair_record_t = *mut lockdownd_pair_record;

#[repr(C)]
pub struct lockdownd_service_descriptor {
    pub port: u16,
    pub ssl_enabled: u8,
}
pub type lockdownd_service_descriptor_t = *mut lockdownd_service_descriptor;

extern "C" {
    pub fn lockdownd_client_new(device: idevice_t, client: *mut lockdownd_client_t, label: *const c_char) -> lockdownd_error_t;
    pub fn lockdownd_client_new_with_handshake(device: idevice_t, client: *mut lockdownd_client_t, label: *const c_char) -> lockdownd_error_t;
    pub fn lockdownd_client_free(client: lockdownd_client_t) -> lockdownd_error_t;

    pub fn lockdownd_query_type(client: lockdownd_client_t, type_: *mut *mut c_char) -> lockdownd_error_t;
    pub fn lockdownd_get_value(client: lockdownd_client_t, domain: *const c_char, key: *const c_char, value: *mut plist_t) -> lockdownd_error_t;
    pub fn lockdownd_set_value(client: lockdownd_client_t, domain: *const c_char, key: *const c_char, value: plist_t) -> lockdownd_error_t;
    pub fn lockdownd_remove_value(client: lockdownd_client_t, domain: *const c_char, key: *const c_char) -> lockdownd_error_t;

    pub fn lockdownd_start_service(client: lockdownd_client_t, identifier: *const c_char, service: *mut lockdownd_service_descriptor_t) -> lockdownd_error_t;
    pub fn lockdownd_start_service_with_escrow_bag(client: lockdownd_client_t, identifier: *const c_char, service: *mut lockdownd_service_descriptor_t) -> lockdownd_error_t;
    pub fn lockdownd_service_descriptor_free(service: lockdownd_service_descriptor_t) -> lockdownd_error_t;

    pub fn lockdownd_start_session(client: lockdownd_client_t, host_id: *const c_char, session_id: *mut *mut c_char, ssl_enabled: *mut c_int) -> lockdownd_error_t;
    pub fn lockdownd_stop_session(client: lockdownd_client_t, session_id: *const c_char) -> lockdownd_error_t;

    pub fn lockdownd_send(client: lockdownd_client_t, plist: plist_t) -> lockdownd_error_t;
    pub fn lockdownd_receive(client: lockdownd_client_t, plist: *mut plist_t) -> lockdownd_error_t;

    pub fn lockdownd_pair(client: lockdownd_client_t, pair_record: lockdownd_pair_record_t) -> lockdownd_error_t;
    pub fn lockdownd_validate_pair(client: lockdownd_client_t, pair_record: lockdownd_pair_record_t) -> lockdownd_error_t;
    pub fn lockdownd_unpair(client: lockdownd_client_t, pair_record: lockdownd_pair_record_t) -> lockdownd_error_t;

    pub fn lockdownd_activate(client: lockdownd_client_t, activation_record: plist_t) -> lockdownd_error_t;
    pub fn lockdownd_deactivate(client: lockdownd_client_t) -> lockdownd_error_t;

    pub fn lockdownd_enter_recovery(client: lockdownd_client_t) -> lockdownd_error_t;

    pub fn lockdownd_goodbye(client: lockdownd_client_t) -> lockdownd_error_t;

    pub fn lockdownd_client_set_label(client: lockdownd_client_t, label: *const c_char); // yes, returns void

    pub fn lockdownd_get_device_udid(control: lockdownd_client_t, udid: *mut *mut c_char) -> lockdownd_error_t;
    pub fn lockdownd_get_device_name(client: lockdownd_client_t, device_name: *mut *mut c_char) -> lockdownd_error_t;

    pub fn lockdownd_get_sync_data_classes(client: lockdownd_client_t, classes: *mut *mut *mut c_char, count: *mut c_int) -> lockdownd_error_t;
    pub fn lockdownd_data_classes_free(client: lockdownd_client_t, classes: *mut *mut c_char);
}


