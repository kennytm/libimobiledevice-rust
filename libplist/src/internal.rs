use libc::c_char;

use asprim::AsPrim;
use mbox::MBox;

use std::ptr::null_mut;

/// Number of seconds between 1970 Jan 1st and 2001 Jan 1st. Note that it does not include the
/// missing 22 leap seconds.
pub const TIMESTAMP_OFFSET: i64 = 978307200;

//-------------------------------------------------------------------------------------------------

/// Receives data provided by a libplist.
pub fn recv_data<T: AsPrim, F: FnOnce(*mut *mut c_char, *mut T)>(f: F) -> MBox<[u8]> {
    let mut data = null_mut();
    let mut length = T::cast_from(0);
    f(&mut data, &mut length);
    unsafe { MBox::from_raw_parts(data as *mut u8, length.as_usize()) }
}

#[cfg(test)]
mod recv_data_tests {
    use super::recv_data;
    use libc::{malloc, c_char};
    use std::ptr::copy_nonoverlapping;

    #[test]
    fn standard() {
        let bytes = recv_data(|ptr, len| unsafe {
            *ptr = malloc(15) as *mut c_char;
            copy_nonoverlapping(b"123456789abcdef".as_ptr() as *const c_char, *ptr, 15);
            *len = 15u32;
        });
        assert_eq!(&*bytes, b"123456789abcdef");
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

