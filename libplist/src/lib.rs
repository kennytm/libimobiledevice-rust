//! High-level bindings for libplist.
//!
//! Note: This crate exists only to support `libimobiledevice`, and not for general consumption.
//! If you would like to read or write a property list in Rust, please check the
//! [`plist`](https://crates.io/crates/plist), [`plist-rs`](https://crates.io/crates/plist-rs) or
//! other dedicated crates.
//!
//! # Examples
//!
//! Creating an `OwnedNode`, and also shows how to serialize it into XML.
//!
//! ```rust
//! use libplist::ToPlistNode;
//! use std::collections::HashMap;
//!
//! // Prepare the Rust value.
//! let mut options = HashMap::new();
//! options.insert("ApplicationType", "Any");
//!
//! // Convert the value to libplist node.
//! let node = options.to_plist_node();
//!
//! // Serialize node into XML.
//! assert_eq!(&*node.to_xml(), "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
//! <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
//! <plist version=\"1.0\">
//! <dict>
//! \t<key>ApplicationType</key>
//! \t<string>Any</string>
//! </dict>
//! </plist>
//! ");
//! ```
//!
//! Creating a node from XML, and convert into Rust type.
//!
//! ```rust
//! use libplist::{OwnedNode, FromPlistNode};
//!
//! // Deserialize plist node from XML.
//! let node = OwnedNode::from_xml("<plist>
//!     <array>
//!         <integer>1</integer>
//!         <integer>2</integer>
//!         <integer>3</integer>
//!     </array>
//! </plist>").unwrap();
//!
//! // Convert libplist node into Rust value.
//! let decoded = Vec::<u16>::from_plist_node(&node).unwrap();
//! assert_eq!(decoded, vec![1u16, 2u16, 3u16]);
//! ```
//!
//! # Integrating with `plist` or `plist-rs` crates
//!
//! The `libplist` crate and convert a Plist structure from `plist` or `plist-rs` crate into nodes.
//! To enable this, provide the features like:
//!
//! ```toml
//! [depedencies]
//! # interop with `plist` crate:
//! libplist = { version = "0.1.0", features = ["plist-interop"] }
//!
//! # interop with `plist-rs` crate:
//! libplist = { version = "0.1.0", features = ["plist-rs-interop"] }
//! ```
//!
//! (Note that `plist` and `plist-rs` are mutually-exclusive since they have the same crate name.)

extern crate libplist_sys;
extern crate libc;
extern crate mbox;
extern crate asprim;

#[cfg(test)] #[macro_use] extern crate const_cstr;

#[cfg(feature="plist-interop")] extern crate plist as plist_crate;
#[cfg(feature="plist-interop")] extern crate chrono;
#[cfg(feature="plist-rs-interop")] extern crate plist as plist_rs_crate; // why are you called `plist` as well???

#[macro_use] mod internal;
pub mod c_str;
pub mod node;
pub mod error;
pub mod native;
pub mod plist;
pub mod plist_rs;

pub use error::PlistError;
pub use node::{Node, ArrayNode, DictNode, OwnedNode, FromPlistNode, ToPlistNode};

