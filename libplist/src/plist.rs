#![doc(hidden)]

// Interop with the `plist` crate.
//
// This module enables conversion between libplist node and `plist::Plist` type.

#![cfg(feature="plist-interop")]

use libplist_sys::*;

use std::collections::BTreeMap;

use plist_crate::Plist;
use chrono::{DateTime, UTC, TimeZone, Timelike};

use internal::TIMESTAMP_OFFSET;
use error::PlistError;
use node::{Node, OwnedNode, BorrowedNode, FromPlistNode, ToPlistNode};

impl FromPlistNode for DateTime<UTC> {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        try!(node.expect_type(PLIST_DATE));
        let mut sec = 0;
        let mut usec = 0;
        unsafe { plist_get_date_val(node.as_ptr(), &mut sec, &mut usec) };
        Ok(UTC.timestamp(sec as i64 + TIMESTAMP_OFFSET, usec as u32 * 1000))
    }
}

impl<T: TimeZone> ToPlistNode for DateTime<T> {
    fn to_plist_node(&self) -> OwnedNode {
        let sec = self.timestamp() - TIMESTAMP_OFFSET;
        let usec = (self.nanosecond() / 1000) % 1_000_000; // ignore leap seconds here.
        unsafe { OwnedNode::from_ptr(plist_new_date(sec as i32, usec as i32)) }
    }
}

generate_roundtrip_test!(test_date_roundtrip_after_2001, "2005-06-30T13:44:07.123456Z".parse::<DateTime<UTC>>().unwrap(), DateTime<UTC>);
generate_roundtrip_test!(test_date_roundtrip_before_2001, "1952-12-04T23:59:59.999999Z".parse::<DateTime<UTC>>().unwrap(), DateTime<UTC>);

impl FromPlistNode for Plist {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        match node.node_type() {
            PLIST_BOOLEAN => bool::from_plist_node(node).map(Plist::Boolean),
            PLIST_UINT => i64::from_plist_node(node).map(Plist::Integer),
            PLIST_REAL => f64::from_plist_node(node).map(Plist::Real),
            PLIST_STRING => String::from_plist_node(node).map(Plist::String),
            PLIST_ARRAY => Vec::from_plist_node(node).map(Plist::Array),
            PLIST_DICT => BTreeMap::from_plist_node(node).map(Plist::Dictionary),
            PLIST_DATA => Vec::from_plist_node(node).map(Plist::Data),
            PLIST_DATE => DateTime::from_plist_node(node).map(Plist::Date),
            t => Err(PlistError::UnsupportedType(t)),
        }
    }
}

impl ToPlistNode for Plist {
    fn to_plist_node(&self) -> OwnedNode {
        match *self {
            Plist::Array(ref a) => a.to_plist_node(),
            Plist::Dictionary(ref d) => d.to_plist_node(),
            Plist::Boolean(b) => b.to_plist_node(),
            Plist::Data(ref d) => d.to_plist_node(),
            Plist::Date(ref d) => d.to_plist_node(),
            Plist::Real(r) => r.to_plist_node(),
            Plist::Integer(i) => i.to_plist_node(),
            Plist::String(ref s) => s.to_plist_node(),
        }
    }
}

generate_roundtrip_test!(test_composite_plist_node, Plist::Array(vec![
    Plist::Boolean(true),
    Plist::Date("2016-03-07T03:44:19Z".parse::<DateTime<UTC>>().unwrap()),
    Plist::Dictionary(BTreeMap::new()),
    Plist::Real(0.0),
    Plist::Integer(-145),
    Plist::Data(b"\x01\x23\0\0\xff".to_vec()),
    Plist::String("abc\u{dddef}gh".to_owned()),
]), Plist);

