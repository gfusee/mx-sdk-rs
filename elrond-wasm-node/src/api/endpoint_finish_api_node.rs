use super::VmApiImpl;
use elrond_wasm::api::{EndpointFinishApi, Handle};

extern "C" {
    fn finish(dataOffset: *const u8, length: i32);

    // big int API
    fn bigIntFinishUnsigned(bih: i32);
    fn bigIntFinishSigned(bih: i32);

    // small int API
    fn smallIntFinishUnsigned(value: i64);
    fn smallIntFinishSigned(value: i64);

    // managed buffer API
    fn mBufferNew() -> i32;
    fn mBufferFromBigFloat(mBufferHandle: i32, bigFloatHandle: i32) -> i32;
    fn mBufferFinish(mBufferHandle: i32) -> i32;
}

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl EndpointFinishApi for VmApiImpl {
    #[inline]
    fn finish_slice_u8(&self, slice: &[u8]) {
        unsafe {
            finish(slice.as_ptr(), slice.len() as i32);
        }
    }

    #[inline]
    fn finish_big_int_raw(&self, handle: i32) {
        unsafe {
            bigIntFinishSigned(handle);
        }
    }

    #[inline]
    fn finish_big_uint_raw(&self, handle: i32) {
        unsafe {
            bigIntFinishUnsigned(handle);
        }
    }

    #[inline]
    fn finish_managed_buffer_raw(&self, handle: Handle) {
        unsafe {
            mBufferFinish(handle);
        }
    }

    #[inline]
    fn finish_u64(&self, value: u64) {
        unsafe {
            smallIntFinishUnsigned(value as i64);
        }
    }

    #[inline]
    fn finish_i64(&self, value: i64) {
        unsafe {
            smallIntFinishSigned(value);
        }
    }

    #[inline]
    fn finish_big_float(&self, big_float_handle: Handle) {
        unsafe {
            let new_buffer_handle = mBufferNew();
            mBufferFromBigFloat(new_buffer_handle, big_float_handle);
            mBufferFinish(new_buffer_handle);
        }
    }
}
