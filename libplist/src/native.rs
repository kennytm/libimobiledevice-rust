#![doc(hidden)]

// Converting between native types and libplist.

use libplist_sys::*;
use mbox::MString;

use std::default::Default;
use std::ptr::null_mut;
use std::collections::{HashMap, BTreeMap};
use std::hash::{Hash, BuildHasher};
use std::time::{UNIX_EPOCH, SystemTime, Duration};
use std::ffi::{CStr, CString};

use libc::{c_double, c_char};

use node::{Node, OwnedNode, BorrowedNode, FromPlistNode, ToPlistNode};
use error::PlistError;
use internal::{recv_data, TIMESTAMP_OFFSET};
use c_str::ToCStr;

//{{{ bool ----------------------------------------------------------------------------------------

impl FromPlistNode for bool {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        try!(node.expect_type(PLIST_BOOLEAN));
        let mut result = 0;
        unsafe { plist_get_bool_val(node.as_ptr(), &mut result) };
        Ok(result != 0)
    }
}

impl ToPlistNode for bool {
    fn to_plist_node(&self) -> OwnedNode {
        OwnedNode::new_bool(*self)
    }
}

generate_roundtrip_test!(test_bool_roundtrip, true, bool);

//}}}

//{{{ uint and real -------------------------------------------------------------------------------

macro_rules! impl_plist_type_for_number {
    (@$ty:ty, $superty:ty, $node_type:expr, $get:expr, $new:ident) => {
        impl FromPlistNode for $ty {
            fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
                try!(node.expect_type($node_type));
                let mut result = Default::default();
                unsafe { $get(node.as_ptr(), &mut result) };
                Ok(result as $ty)
            }
        }

        impl ToPlistNode for $ty {
            fn to_plist_node(&self) -> OwnedNode {
                OwnedNode::$new(*self as $superty)
            }
        }
    };

    ('uint: $($ty:ty),*) => {
        $(impl_plist_type_for_number!(@$ty, u64, PLIST_UINT, plist_get_uint_val, new_uint);)*
    };

    ('real: $($ty:ty),*) => {
        $(impl_plist_type_for_number!(@$ty, c_double, PLIST_REAL, plist_get_real_val, new_real);)*
    };
}

impl_plist_type_for_number! {
    'uint: u16, i16, u32, i32, u64, i64, usize, isize
}

impl_plist_type_for_number! {
    'real: f32, f64
}

generate_roundtrip_test!(test_u16_roundtrip, 31u16, u16);
generate_roundtrip_test!(test_i16_roundtrip, -22i16, i16);
generate_roundtrip_test!(test_u32_roundtrip, 1238031u32, u32);
generate_roundtrip_test!(test_i64_roundtrip, -0x1234_5678_abcd_efaa_i64, i64);
generate_roundtrip_test!(test_f32_roundtrip, 8.625f32, f32);

//}}}

//{{{ String --------------------------------------------------------------------------------------

impl FromPlistNode for MString {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        try!(node.expect_type(PLIST_STRING));
        let mut result = null_mut();
        unsafe {
            plist_get_string_val(node.as_ptr(), &mut result);
            Ok(MString::from_raw_unchecked(result))
        }
    }
}

impl FromPlistNode for String {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        let mstring = try!(MString::from_plist_node(node));
        let sr: &str = &mstring;
        Ok(sr.to_owned())
    }
}

impl ToPlistNode for CStr {
    fn to_plist_node(&self) -> OwnedNode {
        OwnedNode::new_str(self)
    }
}

impl ToPlistNode for str {
    fn to_plist_node(&self) -> OwnedNode {
        let cstring = CString::new(self).expect("The string must not contain any interior null");
        cstring.to_plist_node()
    }
}

impl ToPlistNode for CString {
    fn to_plist_node(&self) -> OwnedNode {
        (**self).to_plist_node()
    }
}

impl ToPlistNode for String {
    fn to_plist_node(&self) -> OwnedNode {
        (**self).to_plist_node()
    }
}

generate_roundtrip_test!(test_str_roundtrip, "helloworld", String);

//}}}

//{{{ Array ---------------------------------------------------------------------------------------

impl<T: FromPlistNode> FromPlistNode for Vec<T> {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        let array = try!(node.array());
        let mut result = Vec::with_capacity(array.len());
        for child in array {
            result.push(try!(T::from_plist_node(child)));
        }
        Ok(result)
    }
}

impl<T: ToPlistNode> ToPlistNode for [T] {
    fn to_plist_node(&self) -> OwnedNode {
        self.iter().map(|x| x.to_plist_node()).collect()
    }
}

impl<T: ToPlistNode> ToPlistNode for Vec<T> {
    fn to_plist_node(&self) -> OwnedNode {
        (**self).to_plist_node()
    }
}

generate_roundtrip_test!(test_array_of_bool_roundtrip, &[true, false] as &[bool], Vec<bool>);
generate_roundtrip_test!(test_array_of_string_roundtrip, &["a", "b", "c"] as &[&'static str], Vec<String>);
generate_roundtrip_test!(test_empty_array_roundtrip, Vec::<u64>::new(), Vec<u64>);

//}}}

//{{{ Dictionary ----------------------------------------------------------------------------------

macro_rules! impl_from_plist_node_for_map {
    (|$d:ident| $n:expr, $key_transform:expr) => {
        fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
            let $d = try!(node.dict());
            let mut result = $n;
            for (key, val) in $d {
                result.insert($key_transform(key), try!(T::from_plist_node(val)));
            }
            Ok(result)
        }
    }
}

impl<T: FromPlistNode> FromPlistNode for BTreeMap<MString, T> {
    impl_from_plist_node_for_map!(|d| BTreeMap::new(), |k| k);
}

impl<T: FromPlistNode, S: BuildHasher + Default> FromPlistNode for HashMap<MString, T, S> {
    impl_from_plist_node_for_map!(|d| HashMap::with_capacity_and_hasher(d.len(), S::default()), |k| k);
}

impl<T: FromPlistNode> FromPlistNode for BTreeMap<String, T> {
    impl_from_plist_node_for_map!(|d| BTreeMap::new(), |k| (&k as &str).to_owned());
}

impl<T: FromPlistNode, S: BuildHasher + Default> FromPlistNode for HashMap<String, T, S> {
    impl_from_plist_node_for_map!(|d| HashMap::with_capacity_and_hasher(d.len(), S::default()), |k| (&k as &str).to_owned());
}

impl<K: ToCStr, V: ToPlistNode> ToPlistNode for BTreeMap<K, V> {
    fn to_plist_node(&self) -> OwnedNode {
        self.iter().map(|(k, v)| (k, v.to_plist_node())).collect()
    }
}

impl<K: ToCStr + Hash + Eq, V: ToPlistNode, S: BuildHasher> ToPlistNode for HashMap<K, V, S> {
    fn to_plist_node(&self) -> OwnedNode {
        self.iter().map(|(k, v)| (k, v.to_plist_node())).collect()
    }
}

generate_roundtrip_test!(test_hash_map_roundtrip, (|| {
    let mut hm = HashMap::with_capacity(5);
    hm.insert("one".to_owned(), 1.0);
    hm.insert("minus one".to_owned(), -1.0);
    hm.insert("zero".to_owned(), 0.0);
    hm.insert("half".to_owned(), 0.5);
    hm.insert("one hundred".to_owned(), 100.0);
    hm
})(), HashMap<String, f64>);

generate_roundtrip_test!(test_btree_map_roundtrip, (|| {
    let mut btm = BTreeMap::new();
    btm.insert("a".to_owned(), vec![123, 456]);
    btm.insert("abc".to_owned(), vec![2235, 19, 20]);
    btm.insert("b".to_owned(), vec![]);
    btm
})(), BTreeMap<String, Vec<u16>>);

generate_roundtrip_test!(test_empty_map_roundtrip, HashMap::new(), HashMap<String, u64>);

//}}}

//{{{ Data ----------------------------------------------------------------------------------------

impl FromPlistNode for Vec<u8> {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        try!(node.expect_type(PLIST_DATA));
        let data = recv_data(|ptr, len| unsafe { plist_get_data_val(node.as_ptr(), ptr, len) });
        Ok(data.to_vec())
    }
}

impl ToPlistNode for [u8] {
    fn to_plist_node(&self) -> OwnedNode {
        unsafe {
            OwnedNode::from_ptr(plist_new_data(self.as_ptr() as *const c_char, self.len() as u64))
        }
    }
}

impl ToPlistNode for Vec<u8> {
    fn to_plist_node(&self) -> OwnedNode {
        (**self).to_plist_node()
    }
}

generate_roundtrip_test!(test_data_roundtrip, b"\x01\x02\x03\x04", Vec<u8>);

//}}}

//{{{ Date ----------------------------------------------------------------------------------------

impl FromPlistNode for SystemTime {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        try!(node.expect_type(PLIST_DATE));
        let mut sec = 0;
        let mut usec = 0;
        unsafe { plist_get_date_val(node.as_ptr(), &mut sec, &mut usec) };
        let sec = sec as i64 + TIMESTAMP_OFFSET;
        if sec >= 0 {
            let dur = Duration::new(sec as u64, usec as u32 * 1000);
            Ok(UNIX_EPOCH + dur)
        } else {
            let dur = Duration::new((-sec-1) as u64, (1_000_000 - usec) as u32 * 1000);
            Ok(UNIX_EPOCH - dur)
        }
    }
}

impl ToPlistNode for SystemTime {
    fn to_plist_node(&self) -> OwnedNode {
        let (sec, nsec) = match self.duration_since(UNIX_EPOCH) {
            Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
            Err(e) => {
                let neg_dur = e.duration();
                match (neg_dur.as_secs() as i64, neg_dur.subsec_nanos()) {
                    (s, 0) => (-s, 0),
                    (s, n) => (-s-1, 1_000_000_000 - n),
                }
            }
        };
        unsafe {
            let raw = plist_new_date((sec - TIMESTAMP_OFFSET) as i32, (nsec / 1000) as i32);
            OwnedNode::from_ptr(raw)
        }
    }
}

// ~2009 Feb 18th 23:33:20
generate_roundtrip_test!(test_date_after_2001_roundtrip, UNIX_EPOCH + Duration::from_millis(1234567890123), SystemTime);

// ~1973 Nov 30th 09:33:20
generate_roundtrip_test!(test_date_after_1970_roundtrip, UNIX_EPOCH + Duration::from_millis(123456789012), SystemTime);

// ~1966 Feb 1st 14:26:40
generate_roundtrip_test!(test_date_before_1970_roundtrip, UNIX_EPOCH - Duration::from_millis(123456789012), SystemTime);
generate_roundtrip_test!(test_date_before_1970_round_secs_roundtrip, UNIX_EPOCH - Duration::from_secs(123456789), SystemTime);


//}}}

//{{{ References ----------------------------------------------------------------------------------

// we should be able to say
//
//  impl<T: Deref> ToPlistNode for T where T::Target: ToPlistNode
//
// but coherence blocked us.

impl<'a, T: ToPlistNode + ?Sized> ToPlistNode for &'a T {
    fn to_plist_node(&self) -> OwnedNode {
        (*self).to_plist_node()
    }
}

impl<T: ToPlistNode + ?Sized> ToPlistNode for Box<T> {
    fn to_plist_node(&self) -> OwnedNode {
        (**self).to_plist_node()
    }
}

