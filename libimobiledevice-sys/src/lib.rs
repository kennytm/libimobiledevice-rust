#![allow(non_camel_case_types)]

extern crate libplist_sys;

pub mod idevice;
pub mod lockdown;
pub mod afc;

pub use idevice::*;

#[test]
fn test_validity() {
    unsafe {
        // Just to check if libimobiledevice is linked.
        idevice_set_debug_level(0);
    }
}

