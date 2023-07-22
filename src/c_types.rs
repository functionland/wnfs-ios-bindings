extern crate libc;

use ::core::mem::MaybeUninit as MU;
use anyhow::Result;
use libc::{c_char, size_t};
use libipld::Cid;
use log::trace;

use std::ffi::{CStr, CString};
use wnfs::common::Metadata;
use wnfsutils::private_forest::PrivateDirectoryHelper;

pub trait Empty {
    fn empty() -> Self;
}

#[derive(Clone)]
#[repr(C)]
pub struct RustBytes {
    pub data: *const u8,
    len: size_t,
    cap: size_t,
}

impl From<Vec<u8>> for RustBytes {
    fn from(value: Vec<u8>) -> Self {
        // let mut buf = std::mem::ManuallyDrop::new(value);
        // let data = buf.as_mut_ptr();
        // let len = buf.len();
        // Self { data, len }

        let buf = value;
        let len = buf.len();
        let cap = buf.capacity();
        let ptr = unsafe { ::libc::malloc(len) };
        if ptr.is_null() {
            return Self::empty();
        }
        let dst = unsafe { ::core::slice::from_raw_parts_mut(ptr.cast::<MU<u8>>(), len) };
        let src = unsafe { ::core::slice::from_raw_parts(buf.as_ptr().cast::<MU<u8>>(), len) };
        dst.copy_from_slice(src);
        Self {
            data: ptr as *mut u8,
            len,
            cap,
        }
    }
}

impl Into<Vec<u8>> for RustBytes {
    fn into(self) -> Vec<u8> {
        let result: Vec<u8>;
        if self.data.is_null() {
            result = Vec::new();
        } else {
            result = unsafe {
                std::slice::from_raw_parts(self.data as *const u8, self.len as usize).to_vec()
            };
        }
        result
    }
}

impl Empty for RustBytes {
    fn empty() -> Self {
        Self {
            data: ::std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
}

impl RustBytes {
    fn free(self) {
        if !self.data.is_null() {
            let _s = unsafe { Vec::from_raw_parts(self.data as *mut u8, self.len, self.cap) };
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct RustString {
    pub str: *const c_char,
}

impl From<String> for RustString {
    fn from(value: String) -> Self {
        trace!("**********************From<String> for RustString  started**************");
        trace!(
            "**********************From<String> for RustString  text={:?}",
            value
        );
        Self {
            str: CString::new(value)
                .expect("Failed to serialize string")
                .into_raw(),
        }
    }
}

impl Into<String> for RustString {
    fn into(self) -> String {
        trace!("**********************Into<String> for RustString started**************");
        if self.str.is_null() {
            "".into()
        } else {
            let _str: String = unsafe {
                CStr::from_ptr(self.str)
                    .to_str()
                    .expect("Failed to parse cid")
                    .into()
            };

            trace!(
                "**********************Into<String> for RustString text={}",
                _str
            );
            _str
        }
    }
}

impl TryInto<Cid> for RustString {
    type Error = String;

    fn try_into(self) -> std::result::Result<Cid, Self::Error> {
        let cid_str: String = self.into();
        trace!("**********************TryInto<Cid> for RustString started**************");
        let cid_res = Cid::try_from(cid_str);
        if cid_res.is_ok() {
            let cid = cid_res.unwrap();
            trace!(
                "**********************TryInto<Cid> for RustString cid={}",
                cid.to_owned().to_string()
            );
            Ok(cid.to_owned())
        } else {
            Err(cid_res.err().unwrap().to_string())
        }
    }
}

impl From<Cid> for RustString {
    fn from(value: Cid) -> Self {
        RustString::from(value.to_string())
    }
}

impl Empty for RustString {
    fn empty() -> Self {
        Self {
            str: ::std::ptr::null_mut(),
        }
    }
}

impl RustString {
    fn free(self) {
        if !self.str.is_null() {
            unsafe { CString::from_raw(self.str as *mut c_char) };
        }
    }
}

#[repr(C)]
pub struct RustVoid {}
impl Empty for RustVoid {
    fn empty() -> Self {
        Self {}
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct RustResult<T> {
    pub ok: bool,
    pub err: RustString,
    pub result: T,
}

impl<T: Empty> RustResult<T> {
    pub fn error(err: RustString) -> Self {
        Self {
            ok: false,
            err: err,
            result: T::empty(),
        }
    }

    pub fn ok(result: T) -> Self {
        Self {
            ok: true,
            err: RustString::empty(),
            result,
        }
    }
}

pub unsafe fn prepare_path_segments(path_segments: RustString) -> Vec<String> {
    PrivateDirectoryHelper::parse_path(path_segments.into())
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

#[no_mangle]
pub extern "C" fn rust_result_string_free(arg: RustResult<RustString>) {
    arg.result.free();
}

#[no_mangle]
pub extern "C" fn rust_result_bytes_free(arg: RustResult<RustBytes>) {
    arg.result.free();
}
