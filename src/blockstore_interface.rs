use std::os::raw::c_char;

use libc::c_void;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct SwiftData {
    pub err: *const c_char,
    pub result_ptr: *const u8,
    pub result_count: libc::size_t,
}

#[repr(C)]
#[derive(Clone)]
pub struct BlockStoreInterface {
    pub userdata: *mut c_void,
    pub put_fn: extern "C" fn(
        userdata: *mut c_void,
        cid: *const u8,
        cid_len: *const libc::size_t,
        bytes: *const u8,
        bytes_len: *const libc::size_t,
    ) -> *const SwiftData,
    pub get_fn: extern "C" fn(
        userdata: *mut c_void,
        cid: *const u8,
        cid_len: *const libc::size_t,
    ) -> *const SwiftData,
    pub dealloc: extern "C" fn(swiftdata: *const SwiftData),
}

unsafe impl Send for BlockStoreInterface {}

impl BlockStoreInterface {
    pub fn put(
        self,
        cid: *const u8,
        cid_len: *const libc::size_t,
        bytes: *const u8,
        bytes_len: *const libc::size_t,
    ) -> *const SwiftData {
        let result = (self.put_fn)(self.userdata, cid, cid_len, bytes, bytes_len);
        std::mem::forget(self);
        result
    }
    pub fn get(self, cid: *const u8, cid_len: *const libc::size_t) -> *const SwiftData {
        let result = (self.get_fn)(self.userdata, cid, cid_len);
        std::mem::forget(self);
        result
    }

    pub fn dealloc(self, data: *const SwiftData) {
        (self.dealloc)(data);
        std::mem::forget(self);
    }
}

// TODO: fixme
// impl Drop for BlockStoreInterface {
//     fn drop(&mut self) {
//         panic!("BlockStoreInterface must have explicit put or get call")
//     }
// }
