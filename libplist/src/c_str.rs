use std::ffi::{CStr, CString, NulError};
use std::borrow::Cow;
use std::ops::Deref;

/// A convenient trait to return a `Cow<CStr>` from a string-like object. This
/// method may cause allocation.
pub trait ToCStr {
    fn to_c_str(&self) -> Result<Cow<CStr>, NulError>;
}

impl ToCStr for CStr {
    fn to_c_str(&self) -> Result<Cow<CStr>, NulError> {
        Ok(Cow::Borrowed(self))
    }
}

impl ToCStr for str {
    fn to_c_str(&self) -> Result<Cow<CStr>, NulError> {
        Ok(Cow::Owned(try!(CString::new(self))))
    }
}

impl<R: Deref> ToCStr for R where R::Target: ToCStr {
    fn to_c_str(&self) -> Result<Cow<CStr>, NulError> {
        self.deref().to_c_str()
    }
}

