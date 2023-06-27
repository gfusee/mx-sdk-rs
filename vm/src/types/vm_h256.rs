use core::fmt::Debug;

// const ERR_BAD_H256_LENGTH: &str = "bad H256 length";
const ZERO_32: &[u8] = &[0u8; 32];

/// Type that holds 32 bytes of data.
/// Data is kept on the heap to keep wasm size low and avoid copies.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct H256(Box<[u8; 32]>);

impl From<[u8; 32]> for H256 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(arr: [u8; 32]) -> Self {
        H256(Box::new(arr))
    }
}

impl<'a> From<&'a [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 32]) -> Self {
        H256(Box::new(*bytes))
    }
}

impl<'a> From<&'a mut [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 32]) -> Self {
        H256(Box::new(*bytes))
    }
}

impl From<Box<[u8; 32]>> for H256 {
    #[inline]
    fn from(bytes: Box<[u8; 32]>) -> Self {
        H256(bytes)
    }
}

impl H256 {
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut arr = [0u8; 32];
        let len = core::cmp::min(slice.len(), 32);
        arr[..len].copy_from_slice(&slice[..len]);
        H256(Box::new(arr))
    }
}

impl From<H256> for [u8; 32] {
    #[inline]
    fn from(s: H256) -> Self {
        *(s.0)
    }
}

impl AsRef<[u8]> for H256 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for H256 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl H256 {
    /// Returns a new zero-initialized fixed hash.
    /// Allocates directly in heap.
    /// Minimal resulting wasm code (14 bytes if not inlined).
    pub fn zero() -> Self {
        use alloc::alloc::{alloc_zeroed, Layout};
        unsafe {
            let ptr = alloc_zeroed(Layout::new::<[u8; 32]>()) as *mut [u8; 32];
            H256(Box::from_raw(ptr))
        }
    }

    /// Returns the size of this hash in bytes.
    #[inline]
    pub fn len_bytes() -> usize {
        32
    }

    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    #[inline]
    pub fn as_array(&self) -> &[u8; 32] {
        self.0.as_ref()
    }

    #[inline]
    pub fn copy_to_array(&self, target: &mut [u8; 32]) {
        target.copy_from_slice(&self.0[..]);
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        self.0[..].to_vec()
    }

    /// Pointer to the data on the heap.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the data on the heap.
    /// Used by the API to populate data.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    /// True if all 32 bytes of the hash are zero.
    pub fn is_zero(&self) -> bool {
        self.as_bytes() == ZERO_32
    }

    // /// Transmutes self to an (in principle) variable length boxed bytes object.
    // /// Both BoxedBytes and H256 keep the data on the heap, so only the pointer to that data needs to be transmuted.
    // /// Does not reallocate or copy data, the data on the heap remains untouched.
    // pub fn into_boxed_bytes(self) -> BoxedBytes {
    //     let raw = Box::into_raw(self.0) as *mut u8;
    //     unsafe {
    //         let bytes_box = Box::<[u8]>::from_raw(core::slice::from_raw_parts_mut(raw, 32));
    //         bytes_box.into()
    //     }
    // }
}
