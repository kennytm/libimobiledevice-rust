#![doc(hidden)]

// Interop with the `plist-rs` crate.
//
// This module enables conversion between libplist node and `plist_rs::Plist` type.

#![cfg(feature="plist-rs-interop")]

use libplist_sys::*;

use std::collections::HashMap;
use std::time::SystemTime;
#[cfg(test)] use std::time::UNIX_EPOCH;
#[cfg(test)] use std::default::Default;

use plist_rs_crate::Plist;

use node::{FromPlistNode, ToPlistNode, Node, OwnedNode};
use error::PlistError;

impl FromPlistNode for Plist {
    fn from_plist_node(node: &Node) -> Result<Self, PlistError> {
        match node.node_type() {
            PLIST_BOOLEAN => bool::from_plist_node(node).map(Plist::Boolean),
            PLIST_UINT => i64::from_plist_node(node).map(Plist::Integer),
            PLIST_REAL => f64::from_plist_node(node).map(Plist::Real),
            PLIST_STRING => String::from_plist_node(node).map(Plist::String),
            PLIST_ARRAY => Vec::from_plist_node(node).map(Plist::Array),
            PLIST_DICT => HashMap::from_plist_node(node).map(Plist::Dict),
            PLIST_DATA => Vec::from_plist_node(node).map(Plist::Data),
            PLIST_DATE => SystemTime::from_plist_node(node).map(Plist::DateTime),
            t => Err(PlistError::UnsupportedType(t)),
        }
    }
}

impl ToPlistNode for Plist {
    fn to_plist_node(&self) -> OwnedNode {
        match *self {
            Plist::Array(ref a) => a.to_plist_node(),
            Plist::Dict(ref d) => d.to_plist_node(),
            Plist::Boolean(b) => b.to_plist_node(),
            Plist::Data(ref d) => d.to_plist_node(),
            Plist::DateTime(ref d) => d.to_plist_node(),
            Plist::Real(r) => r.to_plist_node(),
            Plist::Integer(i) => i.to_plist_node(),
            Plist::String(ref s) => s.to_plist_node(),
        }
    }
}

generate_roundtrip_test!(test_composite_plist_node, Plist::Array(vec![
    Plist::Boolean(true),
    Plist::DateTime(UNIX_EPOCH),
    Plist::Dict(HashMap::with_hasher(Default::default())),
    Plist::Real(0.0),
    Plist::Integer(-145),
    Plist::Data(b"\x01\x23\0\0\xff".to_vec()),
    Plist::String("abc\u{dddef}gh".to_owned()),
]), Plist);

