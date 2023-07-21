extern crate libc;

use anyhow::Result;
use libc::{c_char, size_t};
use libipld::Cid;
use log::trace;
use std::boxed::Box;
use std::ffi::{CStr, CString};
use wnfs::common::Metadata;
use wnfsutils::private_forest::PrivateDirectoryHelper;

pub trait Empty{
 fn empty() ->  Self;
}

#[derive(Clone)]
#[repr(C)]
pub struct RustBytes {
    pub data: *const u8,
    pub len: size_t,
}

impl From<Vec<u8>> for RustBytes {
    fn from(value: Vec<u8>) -> Self {
        let mut buf = value.into_boxed_slice();
        let data = buf.as_mut_ptr();
        let len = buf.len();
        std::mem::forget(buf);
        Self { data, len }
    }
}

impl Into<Vec<u8>> for RustBytes{
    fn into(self) -> Vec<u8> {
        if self.data.is_null(){
            Vec::new()
        }else {
        let result = unsafe {std::slice::from_raw_parts(self.data as *const u8, self.len as usize).to_vec()};
        std::mem::forget(&result);
        result
        }
    }
}

impl Empty for RustBytes {
    fn empty() ->  Self {
        Self { data: ::std::ptr::null_mut(), len: 0 }
    }
}

impl Drop for RustBytes{
    fn drop(&mut self) {
        if !self.data.is_null(){
            let s = unsafe { std::slice::from_raw_parts_mut(self.data as *mut u8, self.len) };
            let s = s.as_mut_ptr();
            unsafe {
                Box::from_raw(s);
            }
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
        trace!("**********************From<String> for RustString  text={:?}", value);
        Self {   str:  CString::new(value)
            .expect("Failed to serialize string")
            .into_raw() }
    }
}

impl Into<String> for RustString{
    fn into(self) -> String {
        trace!("**********************Into<String> for RustString started**************");
        if self.str.is_null(){
            "".into()
        }else{
            let _str: String = unsafe { CStr::from_ptr(self.str)
                .to_str()
                .expect("Failed to parse cid")
                .into() };
        
            trace!("**********************Into<String> for RustString text={}", _str);
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
        }else{
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
    fn empty() ->  Self {
        Self { str: ::std::ptr::null_mut() }
    }
}

impl Drop for RustString{
    fn drop(&mut self) {
        if !self.str.is_null(){
            unsafe {
                drop(CString::from_raw(self.str as *mut c_char))
            }
        }
    }
}


#[repr(C)]
pub struct RustVoid {
}
impl Empty for RustVoid {
    fn empty() ->  Self {
        Self {  }
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
    fn from(err: Option<RustString>, result: T) -> Self {
        match err {
            Some(err) => Self {
                ok: false,
                err: err,
                result,
            },
            None => Self {
                ok: true,
                err: RustString { str: ::std::ptr::null_mut() },
                result,
            },
        }
        
    }

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
    drop(arg)
}

#[no_mangle]
pub extern "C" fn rust_result_void_free(arg: *mut RustResult<RustVoid>) {
    drop(arg)
}

#[no_mangle]
pub extern "C" fn rust_result_bytes_free(arg: RustResult<RustBytes>) {
    drop(arg)
}