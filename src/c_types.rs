extern crate libc;

use ::core::mem::MaybeUninit as MU;
use anyhow::Result;
use libc::{c_char, c_void};
use libipld::Cid;
use log::trace;
use std::boxed::Box;
use std::ffi::{CStr, CString};
use wnfs::common::Metadata;
use wnfsutils::private_forest::PrivateDirectoryHelper;

#[repr(C)]
pub struct RustResult<T> {
    pub ok: bool,
    pub err: *const c_char,
    pub result: *const T,
}

impl<T> RustResult<T> {
    fn from(err: Option<String>, result: *mut T) -> *mut Self {
        let rust_result = Box::new(match err {
            Some(err) => Self {
                ok: false,
                err: serialize_string(err),
                result,
            },
            None => Self {
                ok: true,
                err: ::std::ptr::null_mut(),
                result,
            },
        });
        let out = Box::into_raw(rust_result);
        // ::std::mem::forget(rust_result);
        out
    }

    pub unsafe fn drop(ptr: *const Self) {
        if ptr.is_null() {
            return;
        }
        let out = Box::from_raw(ptr as *mut T);
        std::mem::drop(out)
    }
}

pub unsafe fn serialize_result(err: Option<String>) -> *mut RustResult<c_void> {
    trace!("**********************serialize_result started**************");
    RustResult::from(err, std::ptr::null_mut())
}

pub unsafe fn serialize_bytes_result(err: Option<String>, result: *mut u8) -> *mut RustResult<u8> {
    trace!("**********************serialize_bytes_result started**************");
    RustResult::from(err, result)
}

pub unsafe fn serialize_string_result(
    err: Option<String>,
    result: *const c_char,
) -> *mut RustResult<c_char> {
    trace!("**********************serialize_string_result started**************");
    RustResult::from(err, result as *mut _)
}

pub unsafe fn deserialize_cid(cid: *const c_char) -> Cid {
    let cid_str: String = CStr::from_ptr(cid)
        .to_str()
        .expect("Failed to parse cid")
        .into();
    let cid = Cid::try_from(cid_str).unwrap();
    trace!("**********************deserialize_cid started**************");
    trace!(
        "**********************deserialize_cid cid={}",
        cid.to_string()
    );
    cid
}

pub fn serialize_cid(cid: Cid) -> *mut c_char {
    trace!("**********************serialize_cid started**************");
    trace!(
        "**********************serialize_cid cid={:?}",
        cid.to_string()
    );
    CString::new(cid.to_string())
        .expect("Failed to serialize result")
        .into_raw()
}

pub unsafe fn deserialize_string(text: *const c_char) -> String {
    trace!("**********************deserialize_cid started**************");
    let _str: String = CStr::from_ptr(text)
        .to_str()
        .expect("Failed to parse cid")
        .into();

    trace!("**********************deserialize_text text={}", _str);
    _str
}

pub fn serialize_string(text: String) -> *mut c_char {
    trace!("**********************serialize_string started**************");
    trace!("**********************serialize_string text={:?}", text);
    CString::new(text)
        .expect("Failed to serialize result")
        .into_raw()
}

pub unsafe fn prepare_path_segments(path_segments: *const c_char) -> Vec<String> {
    let path: String = CStr::from_ptr(path_segments)
        .to_str()
        .expect("Failed to parse input path segments")
        .into();

    PrivateDirectoryHelper::parse_path(path)
        .iter()
        .map(|s| s.to_string())
        .collect()
}

pub fn prepare_ls_output(ls_result: Vec<(String, Metadata)>) -> Result<Vec<u8>, String> {
    let mut result: Vec<u8> = Vec::new();

    let item_separator = "???".to_owned();
    let line_separator = "!!!".to_owned();
    for item in ls_result.iter() {
        let created = item.1.clone().get_created();
        let modification = item.1.clone().get_modified();
        if created.is_some() && modification.is_some() {
            let filename: String = item.0.clone().to_string().to_owned();
            let creation_time: String = created.unwrap().to_string().to_owned();
            let modification_time: String = modification.unwrap().to_string().to_owned();

            let row_string: String = format!(
                "{}{}{}{}{}{}",
                filename,
                item_separator,
                creation_time,
                item_separator,
                modification_time,
                line_separator
            );
            let row_byte = row_string.as_bytes().to_vec();
            result.append(&mut row_byte.to_owned());
        }
    }
    Ok(result)
}

pub unsafe fn ffi_input_array_to_vec(size: libc::size_t, array_pointer: *const u8) -> Vec<u8> {
    let result = std::slice::from_raw_parts(array_pointer as *const u8, size as usize).to_vec();
    std::mem::forget(&result);
    result
}

pub unsafe fn vec_to_c_array(buf: &mut Vec<u8>, len: *mut usize, capacity: *mut usize) -> *mut u8 {
    *len = buf.len();
    *capacity = buf.capacity();
    let ptr = unsafe { ::libc::malloc(buf.len()) };
    if ptr.is_null() {
        return ptr as *mut _;
    }
    let dst = ::core::slice::from_raw_parts_mut(ptr.cast::<MU<u8>>(), buf.len());
    let src = ::core::slice::from_raw_parts(buf.as_ptr().cast::<MU<u8>>(), buf.len());
    dst.copy_from_slice(src);
    ptr as *mut _
}

#[no_mangle]
pub extern "C" fn rust_result_string_free(ptr: *mut RustResult<c_char>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        RustResult::drop(ptr as *const _);
    }
}

#[no_mangle]
pub extern "C" fn rust_result_void_free(ptr: *mut RustResult<c_void>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        RustResult::drop(ptr as *const _);
    }
}

#[no_mangle]
pub extern "C" fn rust_result_bytes_free(ptr: *mut RustResult<u8>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        RustResult::drop(ptr as *const _);
    }
}

#[no_mangle]
pub extern "C" fn cstring_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
    }
}

#[no_mangle]
pub unsafe extern "C" fn cbytes_free(data: *mut u8, len: i32, capacity: i32) {
    let v = Vec::from_raw_parts(data, len as usize, capacity as usize);
    drop(v); // or it could be implicitly dropped
}
