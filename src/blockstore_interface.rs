use libc::c_void;

use crate::c_types::{RustBytes, RustResult, RustVoid};

#[repr(C)]
#[derive(Clone)]
pub struct BlockStoreInterface {
    pub userdata: *mut c_void,
    pub put_fn: extern "C" fn(
        userdata: *mut c_void,
        cid: RustBytes,
        bytes: RustBytes,
    ) -> RustResult<RustVoid>,
    pub get_fn: extern "C" fn(
        userdata: *mut c_void,
        cid: RustBytes,
    ) -> RustResult<RustBytes>,
    pub dealloc_after_get: extern "C" fn(data: RustResult<RustBytes>),
    pub dealloc_after_put: extern "C" fn(data: RustResult<RustVoid>),
}

unsafe impl Send for BlockStoreInterface {}

impl BlockStoreInterface {
    pub fn put(
        self,
        cid: RustBytes,
        bytes: RustBytes,
    ) -> RustResult<RustVoid> {
        let result = (self.put_fn)(self.userdata, cid, bytes);
        std::mem::forget(self);
        result
    }
    pub fn get(self, cid: RustBytes) -> RustResult<RustBytes> {
        let result = (self.get_fn)(self.userdata, cid);
        std::mem::forget(self);
        result
    }

    pub fn dealloc_after_get(self, data: RustResult<RustBytes>) {
        (self.dealloc_after_get)(data);
        std::mem::forget(self);
    }

    pub fn dealloc_after_put(self, data: RustResult<RustVoid>) {
        (self.dealloc_after_put)(data);
        std::mem::forget(self);
    }
}

// TODO: fixme
// impl Drop for BlockStoreInterface {
//     fn drop(&mut self) {
//         panic!("BlockStoreInterface must have explicit put or get call")
//     }
// }
