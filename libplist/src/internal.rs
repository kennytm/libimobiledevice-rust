use libc::{c_void, c_char, free};

use std::ptr::null_mut;
use std::ffi::CStr;
use std::slice::from_raw_parts;

use error::PlistError;

/// Number of seconds between 1970 Jan 1st and 2001 Jan 1st. Note that it does not include the
/// missing 22 leap seconds.
pub const TIMESTAMP_OFFSET: i64 = 978307200;

//-------------------------------------------------------------------------------------------------

/// Execute `f()`, then free the pointer `p`, then return the result of `f()`.
pub fn free_finally<F: FnOnce() -> T, P: ?Sized, T>(p: *mut P, f: F) -> T {
    let result = f();
    unsafe { free(p as *mut c_void) };
    result
}

#[cfg(test)]
fn malloc_c_string(src: &[u8]) -> *mut c_char {
    use std::ptr::copy_nonoverlapping;
    use libc::malloc;
    unsafe {
        let content = malloc(src.len() + 1) as *mut c_char;
        copy_nonoverlapping(src.as_ptr() as *const c_char, content, src.len());
        *content.offset(src.len() as isize) = 0;
        content
    }
}

//-------------------------------------------------------------------------------------------------

/// Copies a C string into a Rust string. Frees the C string afterwards.
pub fn char_ptr_to_string(p: *mut c_char) -> Result<String, PlistError> {
    free_finally(p, || {
        let cstr = unsafe { CStr::from_ptr(p) };
        let converted = try!(cstr.to_str()).to_owned();
        Ok(converted)
    })
}

#[cfg(test)]
mod char_ptr_to_string_tests {
    use super::{char_ptr_to_string, malloc_c_string};

    #[test]
    fn standard() {
        let m = malloc_c_string(b"abc\0def");
        let r = char_ptr_to_string(m).unwrap();
        assert_eq!(r, "abc".to_owned());
    }

    #[test]
    fn null() {
        let m = malloc_c_string(b"\0");
        let r = char_ptr_to_string(m).unwrap();
        assert_eq!(r, "".to_owned());
    }

    #[test]
    fn non_utf8() {
        let m = malloc_c_string(b"ab\x88\x99ce");
        let r = char_ptr_to_string(m);
        assert!(r.is_err());
    }
}

//-------------------------------------------------------------------------------------------------

/// Copies a C string into a Rust string, with non-UTF-8 characters replaced by U+FFFD.  Frees the
/// C string afterwards.
pub fn char_ptr_to_lossy_string(p: *mut c_char) -> String {
    free_finally(p, || {
        let cstr = unsafe { CStr::from_ptr(p) };
        cstr.to_string_lossy().into_owned()
    })
}

#[cfg(test)]
mod char_ptr_to_lossy_string_tests {
    use super::{char_ptr_to_lossy_string, malloc_c_string};

    #[test]
    fn standard() {
        let m = malloc_c_string(b"abc\0def");
        let r = char_ptr_to_lossy_string(m);
        assert_eq!(r, "abc".to_owned());
    }

    #[test]
    fn null() {
        let m = malloc_c_string(b"\0");
        let r = char_ptr_to_lossy_string(m);
        assert_eq!(r, "".to_owned());
    }

    #[test]
    fn non_utf8() {
        let m = malloc_c_string(b"ab\x88\x99ce");
        let r = char_ptr_to_lossy_string(m);
        assert_eq!(r, "ab\u{fffd}\u{fffd}ce".to_owned());
    }
}

//-------------------------------------------------------------------------------------------------

/// Receives data provided by a libplist.
pub fn recv_data<F: FnOnce(*mut *mut c_char, *mut u32)>(f: F) -> Vec<u8> {
    let mut data = null_mut();
    let mut length = 0;
    f(&mut data, &mut length);
    free_finally(data, || unsafe { from_raw_parts(data as *const u8, length as usize).to_vec() })
}

pub fn recv_data_64<F: FnOnce(*mut *mut c_char, *mut u64)>(f: F) -> Vec<u8> {
    let mut data = null_mut();
    let mut length = 0;
    f(&mut data, &mut length);
    free_finally(data, || unsafe { from_raw_parts(data as *const u8, length as usize).to_vec() })
}

#[cfg(test)]
mod recv_data_tests {
    use super::{recv_data, malloc_c_string};

    #[test]
    fn standard() {
        let bytes = recv_data(|ptr, len| unsafe {
            *ptr = malloc_c_string(b"123456789abcdef");
            *len = 15u32;
        });
        assert_eq!(&bytes, b"123456789abcdef");
    }
}

//-------------------------------------------------------------------------------------------------

macro_rules! generate_roundtrip_test {
    ($name:ident, $src:expr, $ty:ty) => {
        #[test]
        fn $name() {
            let original = $src;
            let n = original.to_plist_node();
            let b: $ty = FromPlistNode::from_plist_node(&n).unwrap();
            let m = b.to_plist_node();
            assert_eq!(b, original);
            assert_eq!(n, m);
        }
    }
}

