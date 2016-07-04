//! Bindings for libplist.
//!
//! Please refer to the [C header](https://cgit.libimobiledevice.org/libplist.git/tree/include/plist/plist.h)
//! for usage.
//!
//! Note: This module exists only to support `libimobiledevice`, and not for general consumption.
//! If you would like to read or write a property list in Rust, please check the
//! [`plist`](https://crates.io/crates/plist), [`plist-rs`](https://crates.io/crates/plist-rs) or
//! other dedicated crates.

#![allow(non_camel_case_types)]

use std::os::raw::{c_void, c_char, c_double};

#[repr(C)]
#[doc(hidden)]
pub struct plist_private(c_void);
pub type plist_t = *mut plist_private;

#[repr(C)]
#[doc(hidden)]
pub struct plist_dict_iter_private(c_void);
pub type plist_dict_iter = *mut plist_dict_iter_private;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum plist_type {
    /// The node is a boolean.
    Boolean,

    /// The node is an integer (u64).
    UInt,

    /// The node is a real number (f64).
    Real,

    /// The node is a UTF-8 string.
    String,

    /// The node is an array.
    Array,

    /// The node is a dictionary (map).
    Dict,

    /// The node is a date/timestamp.
    Date,

    /// The node is raw binary data.
    Data,

    /// The node is a dictionary key.
    Key,

    /// The node is a unique ID used in archived object graph.
    Uid,

    /// No type.
    None,
}

pub const PLIST_BOOLEAN: plist_type = plist_type::Boolean;
pub const PLIST_UINT: plist_type = plist_type::UInt;
pub const PLIST_REAL: plist_type = plist_type::Real;
pub const PLIST_STRING: plist_type = plist_type::String;
pub const PLIST_ARRAY: plist_type = plist_type::Array;
pub const PLIST_DICT: plist_type = plist_type::Dict;
pub const PLIST_DATE: plist_type = plist_type::Date;
pub const PLIST_DATA: plist_type = plist_type::Data;
pub const PLIST_KEY: plist_type = plist_type::Key;
pub const PLIST_UID: plist_type = plist_type::Uid;
pub const PLIST_NONE: plist_type = plist_type::None;

extern "C" {

//{{{ Creation & Destruction ----------------------------------------------------------------------

    pub fn plist_new_dict() -> plist_t;
    pub fn plist_new_array() -> plist_t;
    pub fn plist_new_string(val: *const c_char) -> plist_t;
    pub fn plist_new_bool(val: u8) -> plist_t;
    pub fn plist_new_uint(val: u64) -> plist_t;
    pub fn plist_new_real(val: c_double) -> plist_t;
    pub fn plist_new_data(val: *const c_char, length: u64) -> plist_t;
    pub fn plist_new_date(sec: i32, usec: i32) -> plist_t;
    pub fn plist_new_uid(val: u64) -> plist_t;
    pub fn plist_free(plist: plist_t);
    pub fn plist_copy(node: plist_t) -> plist_t;

//}}}

//{{{ Array functions -----------------------------------------------------------------------------

    pub fn plist_array_get_size(node: plist_t) -> u32;
    pub fn plist_array_get_item(node: plist_t, n: u32) -> plist_t;
    pub fn plist_array_get_item_index(node: plist_t) -> u32;
    pub fn plist_array_set_item(node: plist_t, item: plist_t, n: u32);
    pub fn plist_array_append_item(node: plist_t, item: plist_t);
    pub fn plist_array_insert_item(node: plist_t, item: plist_t, n: u32);
    pub fn plist_array_remove_item(node: plist_t, n: u32);

//}}}

//{{{ Dictionary functions ------------------------------------------------------------------------

    pub fn plist_dict_get_size(node: plist_t) -> u32;
    pub fn plist_dict_new_iter(node: plist_t, iter: *mut plist_dict_iter);
    pub fn plist_dict_next_item(node: plist_t, iter: plist_dict_iter, key: *mut *mut c_char, val: *mut plist_t);
    pub fn plist_dict_get_item_key(node: plist_t, key: *mut *mut c_char);
    pub fn plist_dict_get_item(node: plist_t, key: *const c_char) -> plist_t;
    pub fn plist_dict_set_item(node: plist_t, key: *const c_char, item: plist_t);
    #[deprecated(since="1.11", note="use plist_dict_set_item instead")]
    pub fn plist_dict_insert_item(node: plist_t, key: *const c_char, item: plist_t);
    pub fn plist_dict_remove_item(node: plist_t, key: *const c_char);
    pub fn plist_dict_merge(target: *mut plist_t, source: plist_t);

//}}}

//{{{ Getters -------------------------------------------------------------------------------------

    pub fn plist_get_parent(node: plist_t) -> plist_t;
    pub fn plist_get_node_type(node: plist_t) -> plist_type;
    pub fn plist_get_key_val(node: plist_t, val: *mut *mut c_char);
    pub fn plist_get_string_val(node: plist_t, val: *mut *mut c_char);
    pub fn plist_get_bool_val(node: plist_t, val: *mut u8);
    pub fn plist_get_uint_val(node: plist_t, val: *mut u64);
    pub fn plist_get_real_val(node: plist_t, val: *mut c_double);
    pub fn plist_get_data_val(node: plist_t, val: *mut *mut c_char, length: *mut u64);
    pub fn plist_get_date_val(node: plist_t, sec: *mut i32, usec: *mut i32);
    pub fn plist_get_uid_val(node: plist_t, val: *mut u64);

//}}}

//{{{ Setters -------------------------------------------------------------------------------------

    pub fn plist_set_type(node: plist_t, type_: plist_type);
    pub fn plist_set_key_val(node: plist_t, val: *const c_char);
    pub fn plist_set_string_val(node: plist_t, val: *const c_char);
    pub fn plist_set_bool_val(node: plist_t, val: u8);
    pub fn plist_set_uint_val(node: plist_t, val: u64);
    pub fn plist_set_real_val(node: plist_t, val: c_double);
    pub fn plist_set_data_val(node: plist_t, val: *const c_char, length: u64);
    pub fn plist_set_date_val(node: plist_t, sec: i32, usec: i32);
    pub fn plist_set_uid_val(node: plist_t, val: u64);

//}}}

//{{{ Import and Export ---------------------------------------------------------------------------

    pub fn plist_to_xml(plist: plist_t, plist_xml: *mut *mut c_char, length: *mut u32);
    pub fn plist_to_bin(plist: plist_t, plist_bin: *mut *mut c_char, length: *mut u32);
    pub fn plist_from_xml(plist_xml: *const c_char, length: u32, plist: *mut plist_t);
    pub fn plist_from_bin(plist_bin: *const c_char, length: u32, plist: *mut plist_t);

//}}}

//{{{ Utils ---------------------------------------------------------------------------------------

    pub fn plist_access_path(plist: plist_t, length: u32, ...) -> plist_t;
    //pub fn plist_access_pathv(plist: plist_t, length: u32, v: VaList) -> plist_t;
    pub fn plist_compare_node_value(node_l: plist_t, node_r: plist_t) -> c_char;

//}}}

}

#[test]
fn test_validity() {
    unsafe {
        // Just to check if libplist is linked.
        let plist = plist_new_array();
        plist_array_append_item(plist, plist_new_bool(1));
        plist_array_append_item(plist, plist_new_bool(0));
        assert_eq!(plist_get_node_type(plist), PLIST_ARRAY);
        assert_eq!(plist_array_get_size(plist), 2);
        plist_free(plist);
    }
}


