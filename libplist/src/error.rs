//! Error types.

use std::fmt;
use std::error::Error;
use std::str::Utf8Error;
use std::convert::From;

use libplist_sys::plist_type;

/// Error while converting a libplist value to a Rust value.
#[derive(Debug)]
pub enum PlistError {
    /// The plist type is not supported.
    UnsupportedType(plist_type),

    /// The plist contains non-UTF-8 strings.
    Utf8(Utf8Error),
}

impl Error for PlistError {
    fn description(&self) -> &str {
        match *self {
            PlistError::UnsupportedType(_) => "unsupported plist type",
            PlistError::Utf8(_) => "string is not properly UTF-8-encoded",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PlistError::Utf8(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for PlistError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlistError::UnsupportedType(t) => {
                writeln!(formatter, "unsupported plist type {:?}", t)
            }
            PlistError::Utf8(ref e) => e.fmt(formatter),
        }
    }
}

impl From<Utf8Error> for PlistError {
    fn from(e: Utf8Error) -> Self {
        PlistError::Utf8(e)
    }
}

