//! Safe wrapper of libplist nodes.
//!
//! In libplist, every item is represented as a node. Simple types like strings and integers are
//! single nodes while composite structures like array and dictionary carry child nodes. The whole
//! property list is a tree of these nodes.
//!
//! This crate represents the root of each tree as an [`OwnedNode`](struct.OwnedNode.html), and we
//! can borrow access of the child as [`Node`](struct.Node.html) references. The relationship
//! between `OwnedNode` and `Node` is similar to `String` vs `str`.

use libplist_sys::*;

use libc::{c_void, c_double, c_char, free};
use mbox::{MBox, MString};

use std::convert::{AsRef, AsMut};
use std::mem::forget;
use std::ops::{Deref, DerefMut, Index};
use std::iter::{IntoIterator, ExactSizeIterator, FromIterator, Extend};
use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::ffi::CStr;
use std::ptr::null_mut;
use std::fmt;

use error::PlistError;
use internal::recv_data;
use c_str::ToCStr;

//{{{ Node ----------------------------------------------------------------------------------------

/// Safe wrapper around a borrowed libplist node.
pub struct Node(plist_private);

impl Node {
    /// Obtains the parent of the node, if any.
    pub fn parent(&self) -> Option<&Node> {
        unsafe {
            Node::try_from_ptr(plist_get_parent(self.as_ptr()))
        }
    }

    /// Obtains the type of the node.
    pub fn node_type(&self) -> plist_type {
        unsafe {
            plist_get_node_type(self.as_ptr())
        }
    }

    /// Verifies that the node has the given type. If not, returns `Err(UnsupportedType)`.
    pub fn expect_type(&self, ty: plist_type) -> Result<(), PlistError> {
        let real_ty = self.node_type();
        if real_ty == ty {
            Ok(())
        } else {
            Err(PlistError::UnsupportedType(real_ty))
        }
    }

    /// Serializes the output to XML property list.
    pub fn to_xml(&self) -> MBox<str> {
        unsafe {
            let data = recv_data(|ptr, len| plist_to_xml(self.as_ptr(), ptr, len));
            MBox::from_utf8_unchecked(data)
        }
    }

    /// Serializes the output to binary property list.
    pub fn to_binary(&self) -> MBox<[u8]> {
        recv_data(|ptr, len| unsafe { plist_to_bin(self.as_ptr(), ptr, len) })
    }
}

//}}}

//{{{ Owned node ----------------------------------------------------------------------------------

/// Safe wrapper around an owned libplist node. The associated resource will be freed when dropped.
pub struct OwnedNode(plist_t);

impl OwnedNode {
    pub unsafe fn from_ptr(node: plist_t) -> OwnedNode {
        OwnedNode(node)
    }

    pub unsafe fn try_from_ptr(node: plist_t) -> Option<OwnedNode> {
        if node.is_null() {
            None
        } else {
            Some(Self::from_ptr(node))
        }
    }

    pub fn as_ptr(&self) -> plist_t {
        self.0
    }

    /// Obtains the underlying raw pointer, and prevent dropping it from now on.
    pub fn take(self) -> plist_t {
        let result = self.0;
        forget(self);
        result
    }

    /// Creates an empty array.
    pub fn new_array() -> OwnedNode {
        unsafe {
            OwnedNode::from_ptr(plist_new_array())
        }
    }

    /// Creates an empty dictionary.
    pub fn new_dict() -> OwnedNode {
        unsafe {
            OwnedNode::from_ptr(plist_new_dict())
        }
    }

    /// Creates an unsigned integer node.
    pub fn new_uint(value: u64) -> OwnedNode {
        unsafe {
            OwnedNode::from_ptr(plist_new_uint(value))
        }
    }

    /// Creates a string node.
    pub fn new_str(value: &CStr) -> OwnedNode {
        unsafe {
            OwnedNode::from_ptr(plist_new_string(value.as_ptr()))
        }
    }

    /// Creates a new boolean node.
    pub fn new_bool(value: bool) -> OwnedNode {
        unsafe {
            OwnedNode::from_ptr(plist_new_bool(if value { 1 } else { 0 }))
        }
    }

    /// Creates a new floating-point value node.
    pub fn new_real(value: c_double) -> OwnedNode {
        unsafe {
            OwnedNode::from_ptr(plist_new_real(value))
        }
    }

    fn deserialize(data: &[u8], reader: unsafe extern fn(*const c_char, u32, *mut plist_t)) -> Option<OwnedNode> {
        let mut output = null_mut();
        unsafe {
            reader(data.as_ptr() as *const c_char, data.len() as u32, &mut output);
            OwnedNode::try_from_ptr(output)
        }
    }

    /// Deserializes an XML property list into a node.
    pub fn from_xml(data: &str) -> Option<OwnedNode> {
        OwnedNode::deserialize(data.as_bytes(), plist_from_xml)
    }

    /// Deserializes a binary property list into a node.
    pub fn from_binary(data: &[u8]) -> Option<OwnedNode> {
        OwnedNode::deserialize(data, plist_from_bin)
    }
}

impl Deref for OwnedNode {
    type Target = Node;
    fn deref(&self) -> &Node {
        unsafe {
            Node::from_ptr(self.as_ptr())
        }
    }
}

impl DerefMut for OwnedNode {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            Node::from_mut_ptr(self.as_ptr())
        }
    }
}

impl Drop for OwnedNode {
    fn drop(&mut self) {
        unsafe { plist_free(self.as_ptr()) };
    }
}

impl Borrow<Node> for OwnedNode {
    fn borrow(&self) -> &Node {
        self
    }
}

impl BorrowMut<Node> for OwnedNode {
    fn borrow_mut(&mut self) -> &mut Node {
        self
    }
}

impl AsRef<Node> for OwnedNode {
    fn as_ref(&self) -> &Node {
        self
    }
}

impl AsMut<Node> for OwnedNode {
    fn as_mut(&mut self) -> &mut Node {
        self
    }
}

impl ToOwned for Node {
    type Owned = OwnedNode;
    fn to_owned(&self) -> OwnedNode {
        unsafe {
            OwnedNode::from_ptr(plist_copy(self.as_ptr()))
        }
    }
}

impl Clone for OwnedNode {
    fn clone(&self) -> OwnedNode {
        self.deref().to_owned()
    }
}

impl PartialEq for OwnedNode {
    fn eq(&self, other: &OwnedNode) -> bool {
        self.deref() == other.deref()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_xml())
    }
}

impl fmt::Debug for OwnedNode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(formatter)
    }
}

//}}}

//{{{ Node tests ----------------------------------------------------------------------------------

#[cfg(test)]
mod node_tests {
    use super::{Node, OwnedNode};
    use libplist_sys::{PLIST_BOOLEAN, PLIST_UID};

    #[test]
    fn test_new_bool() {
        let n1 = OwnedNode::new_bool(true);
        let n2 = OwnedNode::new_bool(true);
        let n3 = OwnedNode::new_bool(false);
        assert_eq!(n1, n2);
        assert!(n1 != n3);
        assert!(n2 != n3);
        assert_eq!(&*n1.to_xml(), r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<true/>
</plist>
"#);
        assert_eq!(&*n3.to_xml(), r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<false/>
</plist>
"#);
    }

    #[test]
    fn test_to_binary() {
        let n1 = OwnedNode::new_bool(true);
        assert_eq!(&*n1.to_binary(), &b"bplist00\x09\x08\0\0\0\0\0\0\x01\x01\0\0\0\0\0\0\x00\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x00\x09"[..]);
    }

    #[test]
    fn test_expect_type() {
        let n = OwnedNode::new_bool(true);
        n.expect_type(PLIST_BOOLEAN).unwrap();
        assert!(n.expect_type(PLIST_UID).is_err());
    }

    #[test]
    fn test_from_xml() {
        let n = OwnedNode::from_xml("<plist version='1.0'><real>4.5</real></plist>").unwrap();
        assert_eq!(n, OwnedNode::new_real(4.5));
    }

    #[test]
    fn test_from_binary() {
        let n1 = OwnedNode::from_xml("<plist><string>a&#x12345;b</string></plist>").unwrap();
        let n2 = OwnedNode::from_binary(b"bplist00\x64\0a\xd8\x08\xdf\x45\0b\x08\0\0\0\0\0\0\x01\x01\0\0\0\0\0\0\x00\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x00\x11").unwrap();
        let n3 = OwnedNode::new_str(const_cstr!("a\u{12345}b").as_cstr());
        assert_eq!(n1, n2);
        assert_eq!(n1, n3);
        assert_eq!(n2, n3);
    }

    #[test]
    fn test_clone() {
        let n1 = OwnedNode::new_bool(false);
        let n2 = n1.clone();
        assert_eq!(n1, n2);
    }

    #[test]
    fn test_to_owned() {
        let n1 = OwnedNode::new_bool(false);
        let n2: &Node = &n1;
        let n3 = n2.to_owned();
        assert_eq!(n1, n3);
    }

    #[test]
    fn test_borrowing() {
        use std::borrow::{Borrow, BorrowMut};

        let mut n1 = OwnedNode::new_bool(false);
        let _: &Node = &n1;
        let _: &Node = n1.as_ref();
        let _: &Node = n1.borrow();
        let _: &mut Node = &mut n1;
        let _: &mut Node = n1.as_mut();
        let _: &mut Node = n1.borrow_mut();
    }


    #[test]
    fn test_decode_from_invalid() {
        assert!(OwnedNode::from_xml("??").is_none());
        assert!(OwnedNode::from_binary(b"??").is_none());
    }
}

//}}}

//{{{ Array node ----------------------------------------------------------------------------------

/// Safe wrapper around a borrowed libplist array node.
pub struct ArrayNode {
    _private: c_void,
}

impl Node {
    /// Obtains an immutable array view of this node.
    pub fn array(&self) -> Result<&ArrayNode, PlistError> {
        try!(self.expect_type(PLIST_ARRAY));
        unsafe { Ok(ArrayNode::from_ptr(self.as_ptr())) }
    }

    /// Obtains a mutable array view of this node.
    pub fn array_mut(&mut self) -> Result<&mut ArrayNode, PlistError> {
        try!(self.expect_type(PLIST_ARRAY));
        unsafe { Ok(ArrayNode::from_mut_ptr(self.as_ptr())) }
    }

    /// If this node is attached to an array, returns its corresponding index. Otherwise, returns
    /// None.
    pub fn get_index_in_array(&self) -> Option<usize> {
        if self.parent().map(|p| p.node_type() == PLIST_ARRAY).unwrap_or(false) {
            Some(unsafe { plist_array_get_item_index(self.as_ptr()) as usize })
        } else {
            None
        }
    }
}

impl ArrayNode {
    /// Length of the array.
    pub fn len(&self) -> usize {
        unsafe {
            plist_array_get_size(self.as_ptr()) as usize
        }
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Obtains a child of this array. Returns None if the index is out-of-bounds.
    pub fn get(&self, index: usize) -> Option<&Node> {
        unsafe {
            Node::try_from_ptr(plist_array_get_item(self.as_ptr(), index as u32))
        }
    }

    /// Obtains a mutable child of this array. Returns None if the index is out-of-bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node> {
        unsafe {
            Node::try_from_mut_ptr(plist_array_get_item(self.as_ptr(), index as u32))
        }
    }

    /// Sets a child at a specified index. Crashes if the index is out of bounds.
    pub fn set(&mut self, index: usize, item: OwnedNode) {
        unsafe {
            plist_array_set_item(self.as_ptr(), item.take(), index as u32);
        }
    }

    /// Pushes a child to the end of the array.
    pub fn push(&mut self, item: OwnedNode) {
        unsafe {
            plist_array_append_item(self.as_ptr(), item.take());
        }
    }

    /// Inserts a child before the specified index. Crashes if the index is out of bounds.
    pub fn insert(&mut self, index: usize, item: OwnedNode) {
        unsafe {
            plist_array_insert_item(self.as_ptr(), item.take(), index as u32);
        }
    }

    /// Removes a child at the specified index. Crashes if the index is out of bounds.
    pub fn remove(&mut self, index: usize) {
        unsafe {
            plist_array_remove_item(self.as_ptr(), index as u32);
        }
    }

    /// Iterates the child nodes of this array.
    pub fn iter(&self) -> ArrayIter {
        self.into_iter()
    }
}

impl Index<usize> for ArrayNode {
    type Output = Node;

    fn index(&self, index: usize) -> &Node {
        self.get(index).unwrap()
    }
}

//}}}

//{{{ Array iteration -----------------------------------------------------------------------------

/// An iterator of `ArrayNode`.
pub struct ArrayIter<'a> {
    node: &'a ArrayNode,
    index: usize,
}

impl<'a> Iterator for ArrayIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        let result = self.node.get(self.index);
        self.index += 1;
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.node.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for ArrayIter<'a> {}

impl<'a> IntoIterator for &'a ArrayNode {
    type Item = &'a Node;
    type IntoIter = ArrayIter<'a>;
    fn into_iter(self) -> ArrayIter<'a> {
        ArrayIter {
            node: self,
            index: 0,
        }
    }
}

impl Extend<OwnedNode> for ArrayNode {
    fn extend<T: IntoIterator<Item=OwnedNode>>(&mut self, iter: T) {
        for node in iter {
            self.push(node);
        }
    }
}

impl FromIterator<OwnedNode> for OwnedNode {
    fn from_iter<T: IntoIterator<Item=OwnedNode>>(iter: T) -> Self {
        let mut result = OwnedNode::new_array();
        (|r: &mut Node| {
            let array = r.array_mut().unwrap();
            array.extend(iter);
        })(&mut result);
        result
    }
}

//}}}

//{{{ Array tests ---------------------------------------------------------------------------------

#[cfg(test)]
mod array_tests {
    use super::OwnedNode;

    #[test]
    fn test_simple_array() {
        let mut node = OwnedNode::new_array();
        assert!(node.dict().is_err());
        assert!(node.dict_mut().is_err());

        let array = node.array_mut().unwrap();
        array.push(OwnedNode::new_bool(true));
        array.push(OwnedNode::new_bool(false));
        array.push(OwnedNode::new_uint(4));

        assert_eq!(array.len(), 3);
        assert!(!array.is_empty());

        assert_eq!(&array[0], &*OwnedNode::new_bool(true));
        assert_eq!(&array[1], &*OwnedNode::new_bool(false));
        assert_eq!(&array[2], &*OwnedNode::new_uint(4));

        assert_eq!(array.get(0), Some(&*OwnedNode::new_bool(true)));
        assert_eq!(array.get(3), None);
    }

    #[test]
    fn test_parent_and_index() {
        let mut node = OwnedNode::new_array();
        {
            let mut array = node.array_mut().unwrap();
            array.push(OwnedNode::new_bool(false));
            array.push(OwnedNode::new_bool(true));
        }

        let a = OwnedNode::new_bool(true);
        let b = &node.array().unwrap()[1];

        assert_eq!(&*a, b);
        assert_eq!(a.parent(), None);
        assert_eq!(b.parent(), Some(&*node));
        assert_eq!(a.get_index_in_array(), None);
        assert_eq!(b.get_index_in_array(), Some(1));
        assert_eq!(a.get_key_in_dict(), None);
        assert_eq!(b.get_key_in_dict(), None);
    }

    #[test]
    fn test_iter() {
        let mut node = OwnedNode::new_array();
        let array = node.array_mut().unwrap();
        array.push(OwnedNode::new_real(3.5));
        array.push(OwnedNode::new_real(-3.5));

        let a2 = array.iter().collect::<Vec<_>>();

        assert_eq!(a2, vec![
            &*OwnedNode::new_real(3.5),
            &*OwnedNode::new_real(-3.5),
        ]);
    }

    #[test]
    fn test_from_iter() {
        let node = vec![
            OwnedNode::new_uint(1),
            OwnedNode::new_uint(55),
            OwnedNode::new_uint(12),
        ].into_iter().collect::<OwnedNode>();
        let array = node.array().unwrap();

        assert_eq!(array.len(), 3);
        assert_eq!(array.iter().collect::<Vec<_>>(), vec![
            &*OwnedNode::new_uint(1),
            &*OwnedNode::new_uint(55),
            &*OwnedNode::new_uint(12),
        ]);
    }

    #[test]
    fn test_not_a_structure() {
        let mut n = OwnedNode::new_bool(true);
        assert!(n.array().is_err());
        assert!(n.array_mut().is_err());
        assert!(n.dict().is_err());
        assert!(n.dict_mut().is_err());
    }

    #[test]
    fn test_composite() {
        let mut outer = OwnedNode::new_array();
        {
            let mut array_outer = outer.array_mut().unwrap();
            assert_eq!(array_outer.get_mut(0), None);

            array_outer.push(OwnedNode::new_array());
            let inner = array_outer.get_mut(0).unwrap().array_mut().unwrap();
            inner.push(OwnedNode::new_uint(1));
        }

        assert_eq!(outer, OwnedNode::from_xml("<plist><array><array><integer>1</integer></array></array></plist>").unwrap());
    }

    #[test]
    fn test_mutation() {
        let mut node = OwnedNode::from_xml("<plist><array><true/><false/><true/><integer>49</integer></array></plist>").unwrap();
        {
            let mut array = node.array_mut().unwrap();
            array.set(1, OwnedNode::new_str(const_cstr!("foo").as_cstr()));
            array.insert(0, OwnedNode::new_real(-2.5));
            array.remove(3);
        }
        assert_eq!(node, OwnedNode::from_xml("<plist><array><real>-2.5</real><true/><string>foo</string><integer>49</integer></array></plist>").unwrap());
    }
}

//}}}

//{{{ Dictionary node -----------------------------------------------------------------------------

/// Safe wrapper around a borrowed libplist dictionary node.
pub struct DictNode {
    _private: c_void,
}

impl Node {
    /// Obtains an immutable dictionary view of this node.
    pub fn dict(&self) -> Result<&DictNode, PlistError> {
        try!(self.expect_type(PLIST_DICT));
        unsafe { Ok(DictNode::from_ptr(self.as_ptr())) }
    }

    /// Obtains a mutable dictionary view of this node.
    pub fn dict_mut(&mut self) -> Result<&mut DictNode, PlistError> {
        try!(self.expect_type(PLIST_DICT));
        unsafe { Ok(DictNode::from_mut_ptr(self.as_ptr())) }
    }

    /// If this node is attached to a dictionary, returns its corresponding key. Otherwise, returns
    /// None.
    pub fn get_key_in_dict(&self) -> Option<MString> {
        let mut key = null_mut();
        unsafe {
            plist_dict_get_item_key(self.as_ptr(), &mut key);
            if key.is_null() {
                None
            } else {
                Some(MString::from_raw_unchecked(key))
            }
        }
    }
}

impl DictNode {
    /// Length of the dictionary.
    pub fn len(&self) -> usize {
        unsafe {
            plist_dict_get_size(self.as_ptr()) as usize
        }
    }

    /// Checks if the dictionary is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Obtains the node associated with the specified key. Returns `None` if the entry does not
    /// exist.
    pub fn get(&self, key: &CStr) -> Option<&Node> {
        unsafe {
            Node::try_from_ptr(plist_dict_get_item(self.as_ptr(), key.as_ptr()))
        }
    }

    /// Obtains a mutable node associated with the specified key. Returns `None` if the entry does
    /// not exist.
    pub fn get_mut(&mut self, key: &CStr) -> Option<&mut Node> {
        unsafe {
            Node::try_from_mut_ptr(plist_dict_get_item(self.as_ptr(), key.as_ptr()))
        }
    }

    /// Associates a child with the specified key in this dictionary.
    pub fn insert(&mut self, key: &CStr, value: OwnedNode) {
        unsafe {
            plist_dict_set_item(self.as_ptr(), key.as_ptr(), value.take());
        }
    }

    /// Removes the child associated with the specified key. Crashes if the entry did not exist.
    pub fn remove(&mut self, key: &CStr) {
        unsafe {
            plist_dict_remove_item(self.as_ptr(), key.as_ptr());
        }
    }

    pub fn iter(&self) -> DictIter {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> DictMutIter {
        self.into_iter()
    }
}

impl<'a> Index<&'a CStr> for DictNode {
    type Output = Node;
    fn index(&self, index: &'a CStr) -> &Node {
        self.get(index).unwrap()
    }
}

//}}}

//{{{ Dictionary iteration ------------------------------------------------------------------------

struct OwnedDictIter {
    raw: plist_dict_iter,
}

impl OwnedDictIter {
    fn new(dict: &DictNode) -> Self {
        let mut iter = null_mut();
        unsafe { plist_dict_new_iter(dict.as_ptr(), &mut iter) };
        OwnedDictIter { raw: iter }
    }
}

impl Drop for OwnedDictIter {
    fn drop(&mut self) {
        unsafe {
            free(self.raw as *mut c_void);
        }
    }
}

/// An iterator of `DictNode`.
pub struct DictIter<'a> {
    node: &'a DictNode,
    iter: OwnedDictIter,
}

/// A mutable iterator of `DictNode`.
pub struct DictMutIter<'a> {
    node: &'a mut DictNode,
    iter: OwnedDictIter,
}

impl<'a> Iterator for DictIter<'a> {
    type Item = (MString, &'a Node);

    fn next(&mut self) -> Option<(MString, &'a Node)> {
        unsafe {
            let mut key = null_mut();
            let mut val = null_mut();
            plist_dict_next_item(self.node.as_ptr(), self.iter.raw, &mut key, &mut val);
            Node::try_from_ptr(val).map(|node| (MString::from_raw_unchecked(key), node))
        }
    }
}

impl<'a> Iterator for DictMutIter<'a> {
    type Item = (MString, &'a mut Node);

    fn next(&mut self) -> Option<(MString, &'a mut Node)> {
        unsafe {
            let mut key = null_mut();
            let mut val = null_mut();
            plist_dict_next_item(self.node.as_ptr(), self.iter.raw, &mut key, &mut val);
            Node::try_from_mut_ptr(val).map(|node| (MString::from_raw_unchecked(key), node))
        }
    }
}

impl<'a> IntoIterator for &'a DictNode {
    type Item = (MString, &'a Node);
    type IntoIter = DictIter<'a>;
    fn into_iter(self) -> DictIter<'a> {
        DictIter {
            node: self,
            iter: OwnedDictIter::new(self),
        }
    }
}

impl<'a> IntoIterator for &'a mut DictNode {
    type Item = (MString, &'a mut Node);
    type IntoIter = DictMutIter<'a>;
    fn into_iter(self) -> DictMutIter<'a> {
        let raw_iter = OwnedDictIter::new(self);
        DictMutIter {
            node: self,
            iter: raw_iter,
        }
    }
}

impl<K: ToCStr> Extend<(K, OwnedNode)> for DictNode {
    fn extend<T: IntoIterator<Item=(K, OwnedNode)>>(&mut self, iter: T) {
        for (key, val) in iter {
            let cs = key.to_c_str().expect("invalid key for libplist");
            self.insert(&cs, val);
        }
    }
}

impl<K: ToCStr> FromIterator<(K, OwnedNode)> for OwnedNode {
    fn from_iter<T: IntoIterator<Item=(K, OwnedNode)>>(iter: T) -> Self {
        let mut result = OwnedNode::new_dict();
        (|r: &mut Node| {
            let dict = r.dict_mut().unwrap();
            dict.extend(iter);
        })(&mut result);
        result
    }
}

//}}}

//{{{ Node reborrowing ----------------------------------------------------------------------------

/// Common trait for a borrowed libplist node.
pub unsafe trait BorrowedNode: Sized {
    /// Converts a borrowed immutable plist item into a node reference.
    unsafe fn from_ptr<'a>(node: plist_t) -> &'a Self {
        &*(node as *const Self)
    }

    /// Converts a borrowed immutable plist item into a node reference.
    unsafe fn try_from_ptr<'a>(node: plist_t) -> Option<&'a Self> {
        (node as *const Self).as_ref()
    }

    /// Converts a borrowed mutable plist item into a node reference.
    unsafe fn from_mut_ptr<'a>(node: plist_t) -> &'a mut Self {
        &mut *(node as *mut Self)
    }

    /// Converts a borrowed mutable plist item into a node reference.
    unsafe fn try_from_mut_ptr<'a>(node: plist_t) -> Option<&'a mut Self> {
        (node as *mut Self).as_mut()
    }

    /// Obtains the raw plist item for this node.
    fn as_ptr(&self) -> plist_t {
        self as *const Self as plist_t
    }
}

unsafe impl BorrowedNode for Node {}
unsafe impl BorrowedNode for ArrayNode {}
unsafe impl BorrowedNode for DictNode {}

impl Deref for ArrayNode {
    type Target = Node;
    fn deref(&self) -> &Node {
        unsafe {
            Node::from_ptr(self.as_ptr())
        }
    }
}

impl DerefMut for ArrayNode {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            Node::from_mut_ptr(self.as_ptr())
        }
    }
}

impl Deref for DictNode {
    type Target = Node;
    fn deref(&self) -> &Node {
        unsafe {
            Node::from_ptr(self.as_ptr())
        }
    }
}

impl DerefMut for DictNode {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            Node::from_mut_ptr(self.as_ptr())
        }
    }
}

//}}}

//{{{ Dictionary tests ----------------------------------------------------------------------------

#[cfg(test)]
mod dict_tests {
    use super::OwnedNode;
    use std::collections::HashMap;
    use mbox::MString;

    #[test]
    fn test_simple_dict() {
        let mut node = OwnedNode::new_dict();
        assert!(node.array().is_err());
        assert!(node.array_mut().is_err());

        let dict = node.dict_mut().unwrap();
        dict.insert(const_cstr!("one").as_cstr(), OwnedNode::new_uint(1));
        dict.insert(const_cstr!("yes").as_cstr(), OwnedNode::new_bool(false));
        dict.insert(const_cstr!("yes").as_cstr(), OwnedNode::new_bool(true));
        dict.insert(const_cstr!("no").as_cstr(), OwnedNode::new_bool(false));
        dict.remove(const_cstr!("no").as_cstr());

        assert_eq!(dict.len(), 2);
        assert!(!dict.is_empty());

        assert_eq!(&dict[const_cstr!("one").as_cstr()], &*OwnedNode::new_uint(1));
        assert_eq!(&dict[const_cstr!("yes").as_cstr()], &*OwnedNode::new_bool(true));

        assert_eq!(dict.get(const_cstr!("one").as_cstr()), Some(&*OwnedNode::new_uint(1)));
        assert_eq!(dict.get(const_cstr!("no").as_cstr()), None);
    }

    #[test]
    fn test_parent_and_index() {
        let mut node = OwnedNode::new_dict();
        {
            let mut dict = node.dict_mut().unwrap();
            dict.insert(const_cstr!("first").as_cstr(), OwnedNode::new_bool(false));
            dict.insert(const_cstr!("second").as_cstr(), OwnedNode::new_bool(true));
        }

        let a = OwnedNode::new_bool(true);
        let b = &node.dict().unwrap()[const_cstr!("second").as_cstr()];

        assert_eq!(&*a, b);
        assert_eq!(a.parent(), None);
        assert_eq!(b.parent(), Some(&*node));
        assert_eq!(a.get_index_in_array(), None);
        assert_eq!(b.get_index_in_array(), None);
        assert_eq!(a.get_key_in_dict(), None);
        assert_eq!(b.get_key_in_dict(), Some(MString::from_str("second")));
    }

    #[test]
    fn test_iter() {
        let mut node = OwnedNode::new_dict();
        let dict = node.dict_mut().unwrap();
        dict.insert(const_cstr!("a").as_cstr(), OwnedNode::new_real(7.0));
        dict.insert(const_cstr!("b").as_cstr(), OwnedNode::new_real(7.0));

        let actual = dict.iter().collect::<HashMap<_, _>>();

        assert_eq!(actual["a"], &*OwnedNode::new_real(7.0));
        assert_eq!(actual["b"], &*OwnedNode::new_real(7.0));
        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn test_from_iter() {
        let node = vec![
            ("foo", OwnedNode::new_bool(false)),
            ("bar", OwnedNode::new_bool(true)),
        ].into_iter().collect::<OwnedNode>();
        let dict = node.dict().unwrap();

        assert_eq!(dict.len(), 2);

        assert_eq!(&dict[const_cstr!("foo").as_cstr()], &*OwnedNode::new_bool(false));
        assert_eq!(&dict[const_cstr!("bar").as_cstr()], &*OwnedNode::new_bool(true));
    }
}

//}}}

//{{{ Equality ------------------------------------------------------------------------------------

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        match (self.node_type(), other.node_type()) {
            // Arrays and dictionaries are compared by pointer instead of content in libplist.
            // Therefore we reimplement them here.
            (PLIST_ARRAY, PLIST_ARRAY) => {
                let left_array = self.array().unwrap();
                let right_array = other.array().unwrap();
                left_array.iter().eq(right_array.iter())
            }
            (PLIST_DICT, PLIST_DICT) => {
                let left_dict = self.dict().unwrap();
                let right_dict = other.dict().unwrap();
                left_dict.len() == right_dict.len() &&
                        left_dict.iter().all(|(k, v)| right_dict.get(k.as_ref()) == Some(v))
            }
            _ => unsafe {
                plist_compare_node_value(self.as_ptr(), other.as_ptr()) != 0
            }
        }
    }
}

//}}}

//{{{ Traits --------------------------------------------------------------------------------------

/// Implemented for types which can be converted from a node.
pub trait FromPlistNode: Sized {
    /// Reads the content of a libplist node and tries to convert it to a Rust value.
    fn from_plist_node(node: &Node) -> Result<Self, PlistError>;
}

/// Implemented for types which can be converted to a node.
pub trait ToPlistNode {
    /// Converts this type into a libplist node.
    fn to_plist_node(&self) -> OwnedNode;
}

impl FromPlistNode for OwnedNode {
    fn from_plist_node(node: &Node) -> Result<OwnedNode, PlistError> {
        Ok(node.to_owned())
    }
}

impl ToPlistNode for Node {
    fn to_plist_node(&self) -> OwnedNode {
        self.to_owned()
    }
}

//}}}

