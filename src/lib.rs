pub mod blockstore;
pub mod blockstore_interface;
pub mod c_types;
pub mod tests;
pub mod ios {
    extern crate libc;
    use crate::blockstore::BridgedStore;
    use crate::blockstore_interface::BlockStoreInterface;
    use crate::c_types::{
        deserialize_cid, deserialize_string, ffi_input_array_to_vec, prepare_ls_output,
        prepare_path_segments, serialize_bytes_result, serialize_cid, serialize_result,
        serialize_string, serialize_string_result, vec_to_c_array, RustResult,
    };
    use log::trace;
    use std::boxed::Box;
    use std::os::raw::c_char;
    use wnfsutils::blockstore::FFIFriendlyBlockStore;
    use wnfsutils::private_forest::PrivateDirectoryHelper;

    #[no_mangle]
    pub extern "C" fn load_with_wnfs_key_native(
        block_store_interface: BlockStoreInterface,
        wnfs_key_arr_len: libc::size_t,
        wnfs_key_arr_pointer: *const u8,
        cid: *const c_char,
    ) -> *mut RustResult<libc::c_void> {
        trace!("**********************load_with_wnfs_key_native started**************");
        unsafe {
            let store = BridgedStore::new(block_store_interface);
            let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
            let wnfs_key: Vec<u8> = ffi_input_array_to_vec(wnfs_key_arr_len, wnfs_key_arr_pointer);
            let forest_cid = deserialize_cid(cid);
            let helper_res = PrivateDirectoryHelper::synced_load_with_wnfs_key(
                block_store,
                forest_cid,
                wnfs_key,
            );
            trace!("**********************load_with_wnfs_key_native finished**************");
            if helper_res.is_ok() {
                serialize_result(None)
            } else {
                let msg = helper_res.err().unwrap();
                trace!("wnfsError in load_with_wnfs_key_native: {:?}", msg);
                serialize_result(Some(msg))
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn init_native(
        block_store_interface: BlockStoreInterface,
        wnfs_key_arr_len: libc::size_t,
        wnfs_key_arr_pointer: *const u8,
    ) -> *mut RustResult<c_char> {
        trace!("**********************init_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let wnfs_key: Vec<u8> =
            unsafe { ffi_input_array_to_vec(wnfs_key_arr_len, wnfs_key_arr_pointer) };
        let helper_res = PrivateDirectoryHelper::synced_init(block_store, wnfs_key);

        if helper_res.is_ok() {
            let (_, _, cid) = helper_res.unwrap();
            unsafe { serialize_string_result(None, serialize_cid(cid)) }
        } else {
            let msg = helper_res.err().unwrap();
            trace!("wnfsError in init_native: {:?}", msg.to_owned());
            unsafe {
                serialize_string_result(Some(msg.to_owned()), serialize_string(String::new()))
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn write_file_from_path_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,

        path_segments: *const c_char,
        _filename: *const c_char,
    ) -> *mut RustResult<c_char> {
        trace!("**********************write_file_from_path_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let path_segments = unsafe { prepare_path_segments(path_segments) };

            let filename = unsafe { deserialize_string(_filename) };
            trace!("filename, path: {:?} -- {:?}", filename, path_segments);
            let write_file_result = helper.synced_write_file_from_path(&path_segments, &filename);
            trace!("**********************write_file_from_path_native finished**************");
            if write_file_result.is_ok() {
                let cid = write_file_result.ok().unwrap();
                unsafe {
                    return serialize_string_result(None, serialize_cid(cid));
                }
            } else {
                let msg = write_file_result.err().unwrap();
                trace!("wnfsError in write_file_from_path_native: {:?}", msg);
                unsafe {
                    return serialize_string_result(Some(msg), serialize_string(String::new()));
                }
            }
        } else {
            let msg = &mut helper_res.err().unwrap();
            trace!(
                "wnfsError in write_file_from_path_native: {:?}",
                msg.to_owned()
            );
            unsafe {
                return serialize_string_result(
                    Some(msg.to_owned()),
                    serialize_string(String::new()),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn read_filestream_to_path_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,

        path_segments: *const c_char,
        _filename: *const c_char,
    ) -> *mut RustResult<c_char> {
        trace!("wnfs11 **********************read_filestream_to_path_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);
        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let path_segments = unsafe { prepare_path_segments(path_segments) };
            let filename = unsafe { deserialize_string(_filename) };

            trace!("wnfs11 **********************read_filestream_to_path_native filename created**************");
            let result = helper.synced_read_filestream_to_path(&filename, &path_segments, 0);
            trace!("wnfs11 **********************read_filestream_to_path_native finished**************");
            if result.is_ok() {
                unsafe {
                    return serialize_string_result(None, serialize_string(filename));
                }
            } else {
                let err = result.err().unwrap();
                trace!(
                    "wnfsError occured in read_filestream_to_path_native on result: {:?}",
                    err.to_owned()
                );
                unsafe {
                    return serialize_string_result(
                        Some(err.to_owned()),
                        serialize_string(String::new()),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!(
                "wnfsError in read_filestream_to_path_native: {:?}",
                msg.to_owned()
            );
            unsafe {
                return serialize_string_result(
                    Some(msg.to_owned()),
                    serialize_string(String::new()),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn read_file_to_path_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        path_segments: *const c_char,
        _filename: *const c_char,
    ) -> *mut RustResult<c_char> {
        trace!("wnfs11 **********************read_file_to_path_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let path_segments = unsafe { prepare_path_segments(path_segments) };
            let filename = unsafe { deserialize_string(_filename) };

            trace!("wnfs11 **********************read_file_to_path_native filename created**************");
            let result = helper.synced_read_file_to_path(&path_segments, &filename);
            trace!("wnfs11 **********************read_file_to_path_native finished**************");
            if result.is_ok() {
                unsafe {
                    return serialize_string_result(None, serialize_string(filename));
                }
            } else {
                let err = result.err().unwrap();
                trace!(
                    "wnfsError occured in read_file_to_path_native on result: {:?}",
                    err.to_owned()
                );
                unsafe {
                    return serialize_string_result(
                        Some(err.to_owned()),
                        serialize_string(String::new()),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!(
                "wnfsError in read_file_to_path_native: {:?}",
                msg.to_owned()
            );
            unsafe {
                return serialize_string_result(
                    Some(msg.to_owned()),
                    serialize_string(String::new()),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn write_file_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,

        path_segments: *const c_char,
        content_arr_len: libc::size_t,
        content_arr_pointer: *const u8,
    ) -> *mut RustResult<c_char> {
        trace!("**********************write_file_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let path_segments = unsafe { prepare_path_segments(path_segments) };
            let content = unsafe { ffi_input_array_to_vec(content_arr_len, content_arr_pointer) };

            let write_file_res = helper.synced_write_file(&path_segments, content, 0);
            trace!("**********************write_file_native finished**************");
            if write_file_res.is_ok() {
                let cid = write_file_res.ok().unwrap();
                unsafe { serialize_string_result(None, serialize_cid(cid)) }
            } else {
                let msg = write_file_res.err().unwrap();
                trace!("wnfsError in write_file_native: {:?}", msg);
                unsafe {
                    return serialize_string_result(
                        Some(msg.to_owned()),
                        serialize_string(String::new()),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!("wnfsError in write_file_native: {:?}", msg.to_owned());
            unsafe {
                return serialize_string_result(
                    Some(msg.to_owned()),
                    serialize_string(String::new()),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn read_file_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,

        path_segments: *const c_char,
        len: *mut libc::size_t,
        capacity: *mut libc::size_t,
    ) -> *mut RustResult<u8> {
        trace!("**********************read_file_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);
        let empty_vec = &mut Vec::new();
        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let path_segments = unsafe { prepare_path_segments(path_segments) };
            trace!("**********************read_file_native finished**************");
            let result = helper.synced_read_file(&path_segments);
            if result.is_ok() {
                unsafe {
                    return serialize_bytes_result(
                        None,
                        vec_to_c_array(&mut result.unwrap(), len, capacity),
                    );
                }
            } else {
                let msg = result.err().unwrap();
                trace!("wnfsError in read_file_native: {:?}", msg);
                unsafe {
                    return serialize_bytes_result(
                        Some(msg.to_owned()),
                        vec_to_c_array(empty_vec, len, capacity),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!("wnfsError in read_file_native: {:?}", msg.to_owned());
            unsafe {
                return serialize_bytes_result(
                    Some(msg.to_owned()),
                    vec_to_c_array(empty_vec, len, capacity),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn mkdir_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,

        path_segments: *const c_char,
    ) -> *mut RustResult<c_char> {
        trace!("**********************mkdir_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let path_segments = unsafe { prepare_path_segments(path_segments) };
            let mkdir_res = helper.synced_mkdir(&path_segments);
            if mkdir_res.is_ok() {
                let cid = mkdir_res.ok().unwrap();
                trace!("**********************mkdir_native finished**************");
                unsafe { return serialize_string_result(None, serialize_cid(cid)) }
            } else {
                let msg = mkdir_res.err().unwrap();
                trace!("wnfsError in mkdir_native: {:?}", msg.to_owned());
                unsafe {
                    return serialize_string_result(
                        Some(msg.to_owned()),
                        serialize_string(String::new()),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!("wnfsError in mkdir_native: {:?}", msg.to_owned());
            unsafe {
                return serialize_string_result(
                    Some(msg.to_owned()),
                    serialize_string(String::new()),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn mv_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,

        source_path_segments: *const c_char,
        target_path_segments: *const c_char,
    ) -> *mut RustResult<c_char> {
        trace!("**********************mv_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let source_path_segments = unsafe { prepare_path_segments(source_path_segments) };
            let target_path_segments = unsafe { prepare_path_segments(target_path_segments) };
            let result = helper.synced_mv(&source_path_segments, &target_path_segments);
            trace!("**********************mv_native finished**************");
            if result.is_ok() {
                let cid = result.ok().unwrap();
                unsafe {
                    return serialize_string_result(None, serialize_cid(cid));
                }
            } else {
                let msg = result.err().unwrap();
                trace!("wnfsError occured in mv_native: {:?}", msg.to_owned());
                unsafe {
                    return serialize_string_result(
                        Some(msg.to_owned()),
                        serialize_string(String::new()),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!("wnfsError in mv_native: {:?}", msg.to_owned());
            unsafe {
                return serialize_string_result(
                    Some(msg.to_owned()),
                    serialize_string(String::new()),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn cp_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,

        source_path_segments: *const c_char,
        target_path_segments: *const c_char,
    ) -> *mut RustResult<c_char> {
        trace!("**********************cp_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let source_path_segments = unsafe { prepare_path_segments(source_path_segments) };
            let target_path_segments = unsafe { prepare_path_segments(target_path_segments) };
            let result = helper.synced_cp(&source_path_segments, &target_path_segments);
            trace!("**********************cp_native finished**************");
            if result.is_ok() {
                let cid = result.ok().unwrap();
                unsafe {
                    return serialize_string_result(None, serialize_cid(cid));
                }
            } else {
                let msg = result.err().unwrap();
                trace!("wnfsError occured in cp_native: {:?}", msg.to_owned());
                unsafe {
                    return serialize_string_result(
                        Some(msg.to_owned()),
                        serialize_string(String::new()),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!("wnfsError in cp_native: {:?}", msg.to_owned());
            unsafe {
                return serialize_string_result(
                    Some(msg.to_owned()),
                    serialize_string(String::new()),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn rm_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,

        path_segments: *const c_char,
    ) -> *mut RustResult<c_char> {
        trace!("**********************rm_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let path_segments = unsafe { prepare_path_segments(path_segments) };
            let rm_res = helper.synced_rm(&path_segments);
            if rm_res.is_ok() {
                let cid = rm_res.ok().unwrap();
                trace!("**********************rm_native finished**************");
                unsafe { return serialize_string_result(None, serialize_cid(cid)) }
            } else {
                let msg = rm_res.err().unwrap();
                trace!("wnfsError in rm_native: {:?}", msg.to_owned());
                unsafe {
                    return serialize_string_result(
                        Some(msg.to_owned()),
                        serialize_string(String::new()),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!("wnfsError in rm_native: {:?}", msg.to_owned());
            unsafe {
                return serialize_string_result(
                    Some(msg.to_owned()),
                    serialize_string(String::new()),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn ls_native(
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        path_segments: *const c_char,
        len: *mut libc::size_t,
        capacity: *mut libc::size_t,
    ) -> *mut RustResult<u8> {
        trace!("**********************ls_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid = unsafe { deserialize_cid(cid) };
        let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);
        let empty_vec = &mut Vec::new();
        if helper_res.is_ok() {
            let helper = &mut helper_res.ok().unwrap();
            let path_segments = unsafe { prepare_path_segments(path_segments) };
            let ls_res = helper.synced_ls_files(&path_segments);
            if ls_res.is_ok() {
                let output = prepare_ls_output(ls_res.ok().unwrap());
                trace!("**********************ls_native finished**************");
                if output.is_ok() {
                    let res = output.ok().unwrap();
                    unsafe {
                        return serialize_bytes_result(
                            None,
                            vec_to_c_array(&mut res.to_owned(), len, capacity),
                        );
                    }
                } else {
                    let msg = output.err().unwrap().to_string();
                    trace!(
                        "wnfsError occured in ls_native output: {:?}",
                        msg.to_owned()
                    );
                    unsafe {
                        return serialize_bytes_result(
                            Some(msg),
                            vec_to_c_array(empty_vec, len, capacity),
                        );
                    }
                }
            } else {
                let msg = ls_res.err().unwrap();
                trace!(
                    "wnfsError occured in ls_native ls_res: {:?}",
                    msg.to_owned()
                );
                unsafe {
                    return serialize_bytes_result(
                        Some(msg.to_owned()),
                        vec_to_c_array(empty_vec, len, capacity),
                    );
                }
            }
        } else {
            let msg = helper_res.err().unwrap();
            trace!(
                "wnfsError occured in ls_native forest_res: {:?}",
                msg.to_owned()
            );
            unsafe {
                return serialize_bytes_result(
                    Some(msg.to_owned()),
                    vec_to_c_array(empty_vec, len, capacity),
                );
            }
        }
    }
}
