//! Bindings to `afc.h`.

use idevice::idevice_t;
use lockdown::lockdownd_service_descriptor_t;

use std::os::raw::{c_char, c_void, c_int};

pub const AFC_SERVICE_NAME: &'static [u8] = b"com.apple.afc\0";
pub const AFC2_SERVICE_NAME: &'static [u8] = b"com.apple.afc2\0";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum afc_error_t {
    Success = 0,
    UnknownError = 1,
    OpHeaderInvalid = 2,
    NoResources = 3,
    ReadError = 4,
    WriteError = 5,
    UnknownPacketType = 6,
    InvalidArg = 7,
    ObjectNotFound = 8,
    ObjectIsDir = 9,
    PermDenied = 10,
    ServiceNotConnected = 11,
    OpTimeout = 12,
    TooMuchData = 13,
    EndOfData = 14,
    OpNotSupported = 15,
    ObjectExists = 16,
    ObjectBusy = 17,
    NoSpaceLeft = 18,
    OpWouldBlock = 19,
    IoError = 20,
    OpInterrupted = 21,
    OpInProgress = 22,
    InternalError = 23,
    MuxError = 30,
    NoMem = 31,
    NotEnoughData = 32,
    DirNotEmpty = 33,
    ForceSignedType = -1,
}

pub const AFC_E_SUCCESS: afc_error_t = afc_error_t::Success;
pub const AFC_E_UNKNOWN_ERROR: afc_error_t = afc_error_t::UnknownError;
pub const AFC_E_OP_HEADER_INVALID: afc_error_t = afc_error_t::OpHeaderInvalid;
pub const AFC_E_NO_RESOURCES: afc_error_t = afc_error_t::NoResources;
pub const AFC_E_READ_ERROR: afc_error_t = afc_error_t::ReadError;
pub const AFC_E_WRITE_ERROR: afc_error_t = afc_error_t::WriteError;
pub const AFC_E_UNKNOWN_PACKET_TYPE: afc_error_t = afc_error_t::UnknownPacketType;
pub const AFC_E_INVALID_ARG: afc_error_t = afc_error_t::InvalidArg;
pub const AFC_E_OBJECT_NOT_FOUND: afc_error_t = afc_error_t::ObjectNotFound;
pub const AFC_E_OBJECT_IS_DIR: afc_error_t = afc_error_t::ObjectIsDir;
pub const AFC_E_PERM_DENIED: afc_error_t = afc_error_t::PermDenied;
pub const AFC_E_SERVICE_NOT_CONNECTED: afc_error_t = afc_error_t::ServiceNotConnected;
pub const AFC_E_OP_TIMEOUT: afc_error_t = afc_error_t::OpTimeout;
pub const AFC_E_TOO_MUCH_DATA: afc_error_t = afc_error_t::TooMuchData;
pub const AFC_E_END_OF_DATA: afc_error_t = afc_error_t::EndOfData;
pub const AFC_E_OP_NOT_SUPPORTED: afc_error_t = afc_error_t::OpNotSupported;
pub const AFC_E_OBJECT_EXISTS: afc_error_t = afc_error_t::ObjectExists;
pub const AFC_E_OBJECT_BUSY: afc_error_t = afc_error_t::ObjectBusy;
pub const AFC_E_NO_SPACE_LEFT: afc_error_t = afc_error_t::NoSpaceLeft;
pub const AFC_E_OP_WOULD_BLOCK: afc_error_t = afc_error_t::OpWouldBlock;
pub const AFC_E_IO_ERROR: afc_error_t = afc_error_t::IoError;
pub const AFC_E_OP_INTERRUPTED: afc_error_t = afc_error_t::OpInterrupted;
pub const AFC_E_OP_IN_PROGRESS: afc_error_t = afc_error_t::OpInProgress;
pub const AFC_E_INTERNAL_ERROR: afc_error_t = afc_error_t::InternalError;
pub const AFC_E_MUX_ERROR: afc_error_t = afc_error_t::MuxError;
pub const AFC_E_NO_MEM: afc_error_t = afc_error_t::NoMem;
pub const AFC_E_NOT_ENOUGH_DATA: afc_error_t = afc_error_t::NotEnoughData;
pub const AFC_E_DIR_NOT_EMPTY: afc_error_t = afc_error_t::DirNotEmpty;
pub const AFC_E_FORCE_SIGNED_TYPE: afc_error_t = afc_error_t::ForceSignedType;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum afc_file_mode_t {
    ReadOnly = 1,
    ReadWrite = 2,
    Truncate = 3,
    ReadTruncate = 4,
    Append = 5,
    ReadAppend = 6,
}

pub const AFC_FOPEN_RDONLY: afc_file_mode_t = afc_file_mode_t::ReadOnly;
pub const AFC_FOPEN_RW: afc_file_mode_t = afc_file_mode_t::ReadWrite;
pub const AFC_FOPEN_WRONLY: afc_file_mode_t = afc_file_mode_t::Truncate;
pub const AFC_FOPEN_WR: afc_file_mode_t = afc_file_mode_t::ReadTruncate;
pub const AFC_FOPEN_APPEND: afc_file_mode_t = afc_file_mode_t::Append;
pub const AFC_FOPEN_RDAPPEND: afc_file_mode_t = afc_file_mode_t::ReadAppend;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum afc_link_type_t {
    Hardlink = 1,
    Symlink = 2,
}

pub const AFC_HARDLINK: afc_link_type_t = afc_link_type_t::Hardlink;
pub const AFC_SYMLINK: afc_link_type_t = afc_link_type_t::Symlink;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum afc_lock_op_t {
    LockShared = 5,
    LockExclusive = 6,
    Unlock = 12,
}

pub const AFC_LOCK_SH: afc_lock_op_t = afc_lock_op_t::LockShared;
pub const AFC_LOCK_EX: afc_lock_op_t = afc_lock_op_t::LockExclusive;
pub const AFC_LOCK_UN: afc_lock_op_t = afc_lock_op_t::Unlock;

#[doc(hidden)]
#[repr(C)]
pub struct afc_client_private(c_void);
pub type afc_client_t = *mut afc_client_private;

extern "C" {
    pub fn afc_client_new(device: idevice_t, service: lockdownd_service_descriptor_t, client: *mut afc_client_t) -> afc_error_t;
    pub fn afc_client_start_service(device: idevice_t, client: *mut afc_client_t, label: *const c_char) -> afc_error_t;
    pub fn afc_client_free(client: afc_client_t) -> afc_error_t;

    pub fn afc_get_device_info(client: afc_client_t, device_information: *mut *mut *mut c_char) -> afc_error_t;
    pub fn afc_read_directory(client: afc_client_t, path: *const c_char, device_information: *mut *mut *mut c_char) -> afc_error_t;
    pub fn afc_get_file_info(client: afc_client_t, filename: *const c_char, file_information: *mut *mut *mut c_char) -> afc_error_t;
    pub fn afc_dictionary_free(dictionary: *mut *mut c_char) -> afc_error_t;

    pub fn afc_file_open(client: afc_client_t, filename: *const c_char, file_mode: afc_file_mode_t, handle: *mut u64) -> afc_error_t;
    pub fn afc_file_close(client: afc_client_t, handle: u64) -> afc_error_t;
    pub fn afc_file_lock(client: afc_client_t, handle: u64, operation: afc_lock_op_t) -> afc_error_t;
    pub fn afc_file_read(client: afc_client_t, handle: u64, data: *mut c_char, length: u32, bytes_read: *mut u32) -> afc_error_t;
    pub fn afc_file_write(client: afc_client_t, handle: u64, data: *const c_char, length: u32, bytes_written: *mut u32) -> afc_error_t;
    pub fn afc_file_seek(client: afc_client_t, handle: u64, offset: i64, whence: c_int) -> afc_error_t;
    pub fn afc_file_tell(client: afc_client_t, handle: u64, position: *mut u64) -> afc_error_t;
    pub fn afc_file_truncate(client: afc_client_t, handle: u64, newsize: u64) -> afc_error_t;

    pub fn afc_remove_path(client: afc_client_t, path: *const c_char) -> afc_error_t;
    pub fn afc_rename_path(client: afc_client_t, from: *const c_char, to: *const c_char) -> afc_error_t;
    pub fn afc_make_directory(client: afc_client_t, path: *const c_char) -> afc_error_t;
    pub fn afc_truncate(client: afc_client_t, path: *const c_char, newsize: u64) -> afc_error_t;
    pub fn afc_make_link(client: afc_client_t, linktype: afc_link_type_t, target: *const c_char, linkname: *const c_char) -> afc_error_t;
    pub fn afc_set_file_time(client: afc_client_t, path: *const c_char, mtime: u64) -> afc_error_t;
    pub fn afc_remove_path_and_contents(client: afc_client_t, path: *const c_char) -> afc_error_t;

    pub fn afc_get_device_info_key(client: afc_client_t, key: *const c_char, value: *mut *mut c_char) -> afc_error_t;
}























