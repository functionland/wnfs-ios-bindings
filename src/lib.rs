pub mod store;
pub mod ios {
    extern crate libc;
    use crate::store::BridgedStore;
    use anyhow::Result;

    use libipld::Cid;
    use log::trace;

    use std::boxed::Box;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    use std::ptr::null_mut;
    use wnfs::private::PrivateRef;
    use wnfs::Metadata;
    use wnfsutils::blockstore::FFIFriendlyBlockStore;
    use wnfsutils::kvstore::KVBlockStore;

    use wnfsutils::private_forest::PrivateDirectoryHelper;

    #[repr(C)]
    pub struct Config {
        cid: *const c_char,
        private_ref: *const c_char,
    }

    #[no_mangle]
    pub unsafe extern "C" fn create_private_forest_native(db_path: *const c_char) -> *mut c_char {
        trace!("**********************createPrivateForest started**************");
        let store = KVBlockStore::new(
            CStr::from_ptr(db_path).to_str().unwrap().into(),
            libipld::IpldCodec::DagCbor,
        );
        let block_store = FFIFriendlyBlockStore::new(Box::new(store));
        let helper = &mut PrivateDirectoryHelper::new(block_store);
        trace!("**********************createPrivateForest finished**************");
        let private_forest = helper.synced_create_private_forest();
        if private_forest.is_ok() {
            serialize_cid(private_forest.ok().unwrap())
        } else {
            CString::new("")
                .expect("Failed to serialize result")
                .into_raw()
        }
    }

    #[no_mangle]
    pub extern "C" fn get_private_ref_native(
        db_path: *const c_char,
        wnfs_key_arr_size: libc::size_t,
        wnfs_key_arr_pointer: *const libc::uint8_t,
        cid: *const c_char,
    ) -> *mut c_char {
        trace!("**********************getPrivateRefNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let wnfs_key: Vec<u8> = c_array_to_vec(wnfs_key_arr_size, wnfs_key_arr_pointer);
            let forest_cid = deserialize_cid(cid);
            let private_ref = helper.synced_get_private_ref(wnfs_key, forest_cid);
            trace!("**********************getPrivateRefNative finished**************");
            if private_ref.is_ok() {
                return serialize_private_ref(private_ref.ok().unwrap());
            } else {
                CString::new("")
                    .expect("Failed to serialize result")
                    .into_raw()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn create_root_dir_native(
        db_path: *const c_char,
        wnfs_key_arr_size: libc::size_t,
        wnfs_key_arr_pointer: *const libc::uint8_t,
        cid: *const c_char,
    ) -> *mut Config {
        trace!("**********************createRootDirNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let forest_cid = deserialize_cid(cid);
            trace!("cid: {}", forest_cid);
            let forest_res = helper.synced_load_forest(forest_cid);
            if forest_res.is_ok() {
                let forest = forest_res.ok().unwrap();
                let wnfs_key: Vec<u8> = c_array_to_vec(wnfs_key_arr_size, wnfs_key_arr_pointer);
                let init_res = helper.synced_init(forest, wnfs_key);
                if init_res.is_ok() {
                    let (cid, private_ref) = init_res.ok().unwrap();
                    trace!("pref: {:?}", private_ref);
                    trace!("**********************createRootDirNative finished**************");
                    serialize_config(cid, private_ref)
                } else {
                    let msg = init_res.err().unwrap();
                    trace!("wnfsError in createRootDirNative: {:?}", msg);
                    return null_mut();
                }
            } else {
                let msg = forest_res.err().unwrap();
                trace!("wnfsError in createRootDirNative: {:?}", msg);
                return null_mut();
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn write_file_from_path_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        filename: *const c_char,
    ) -> *mut Config {
        trace!("**********************writeFileFromPathNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);
            let _old_private_ref = private_ref.to_owned();
            let _old_cid = cid.to_owned();

            let forest_res = helper.synced_load_forest(cid);
            if forest_res.is_ok() {
                let forest = forest_res.ok().unwrap();
                let root_dir_res = helper.synced_get_root_dir(forest.to_owned(), private_ref);
                if root_dir_res.is_ok() {
                    let root_dir = root_dir_res.ok().unwrap();
                    let path_segments = prepare_path_segments(path_segments);
                    let filename: String = CStr::from_ptr(filename)
                        .to_str()
                        .expect("Failed to parse input path segments")
                        .into();
                    let write_file_result = helper.synced_write_file_from_path(
                        forest.to_owned(),
                        root_dir,
                        &path_segments,
                        &filename,
                    );
                    trace!("**********************writeFileFromPathNative finished**************");
                    if write_file_result.is_ok() {
                        let (cid, private_ref) = write_file_result.ok().unwrap();
                        return serialize_config(cid, private_ref);
                    } else {
                        let msg = write_file_result.err().unwrap();
                        trace!("wnfsError in writeFileFromPathNative: {:?}", msg);
                        return null_mut();
                    }
                } else {
                    let msg = root_dir_res.err().unwrap();
                    trace!("wnfsError in writeFileFromPathNative: {:?}", msg);
                    return null_mut();
                }
            } else {
                let msg = forest_res.err().unwrap();
                trace!("wnfsError in writeFileFromPathNative: {:?}", msg);
                return null_mut();
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn read_filestream_to_path_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        filename: *const c_char,
    ) -> *mut c_char {
        trace!("wnfs11 **********************readFilestreamToPathNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest = helper.synced_load_forest(cid).unwrap();
            let root_dir = helper
                .synced_get_root_dir(forest.to_owned(), private_ref)
                .unwrap();
            let path_segments = prepare_path_segments(path_segments);
            let filename: String = CStr::from_ptr(filename)
                .to_str()
                .expect("Failed to parse input path segments")
                .into();
            trace!("wnfs11 **********************readFilestreamToPathNative filename created**************");
            let result = helper.synced_read_filestream_to_path(
                &filename,
                forest.to_owned(),
                root_dir,
                &path_segments,
                0,
            );
            trace!(
                "wnfs11 **********************readFilestreamToPathNative finished**************"
            );
            if result.is_ok() {
                let _res = result.ok().unwrap();
                CString::new(filename)
                    .expect("Failed to serialize result")
                    .into_raw()
            } else {
                trace!(
                    "wnfsError occured in readFilestreamToPathNative on result: {:?}",
                    result.err().unwrap()
                );
                CString::new("")
                    .expect("Failed to serialize result")
                    .into_raw()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn read_file_to_path_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        filename: *const c_char,
    ) -> *mut c_char {
        trace!("wnfs11 **********************readFileToPathNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest = helper.synced_load_forest(cid).unwrap();
            let root_dir = helper
                .synced_get_root_dir(forest.to_owned(), private_ref)
                .unwrap();
            let path_segments = prepare_path_segments(path_segments);
            let filename: String = CStr::from_ptr(filename)
                .to_str()
                .expect("Failed to parse input path segments")
                .into();

            trace!(
                "wnfs11 **********************readFileToPathNative filename created**************"
            );
            let result = helper.synced_read_file_to_path(
                forest.to_owned(),
                root_dir,
                &path_segments,
                &filename,
            );
            trace!("wnfs11 **********************readFileToPathNative finished**************");
            if result.is_ok() {
                let _res = result.ok().unwrap();
                CString::new(filename)
                    .expect("Failed to serialize result")
                    .into_raw()
            } else {
                trace!(
                    "wnfsError occured in readFileToPathNative {:?}",
                    result.err().unwrap()
                );
                CString::new("")
                    .expect("Failed to serialize result")
                    .into_raw()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn write_file_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        content_arr_size: libc::size_t,
        content_arr_pointer: *const libc::uint8_t,
    ) -> *mut Config {
        trace!("**********************writeFileNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest = helper.synced_load_forest(cid).unwrap();
            let root_dir = helper
                .synced_get_root_dir(forest.to_owned(), private_ref)
                .unwrap();
            let path_segments = prepare_path_segments(path_segments);
            let content = c_array_to_vec(content_arr_size, content_arr_pointer);
            //let (cid, private_ref) =
            let write_file_res =
                helper.synced_write_file(forest.to_owned(), root_dir, &path_segments, content, 0);
            trace!("**********************writeFileNative finished**************");
            if write_file_res.is_ok() {
                let (cid, private_ref) = write_file_res.ok().unwrap();
                let config = serialize_config(cid, private_ref);
                return config;
            } else {
                let msg = write_file_res.err().unwrap();
                trace!("wnfsError in writeFileNative: {:?}", msg);
                return null_mut();
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn read_file_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        len: *mut i32,
        capacity: *mut i32,
    ) -> *mut u8 {
        trace!("**********************readFileNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest = helper.synced_load_forest(cid).unwrap();
            let root_dir = helper
                .synced_get_root_dir(forest.to_owned(), private_ref)
                .unwrap();
            let path_segments = prepare_path_segments(path_segments);
            trace!("**********************readFileNative finished**************");
            let result = helper.synced_read_file(forest.to_owned(), root_dir, &path_segments);
            if result.is_err() {
                let empty_vec = &mut Vec::new();
                return vec_to_c_array(empty_vec, len, capacity);
            }
            vec_to_c_array(&mut result.ok().unwrap(), len, capacity)
        }
    }

    #[no_mangle]
    pub extern "C" fn mkdir_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
    ) -> *mut Config {
        trace!("**********************mkDirNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest_res = helper.synced_load_forest(cid);
            if forest_res.is_ok() {
                let forest = forest_res.ok().unwrap();
                let root_dir_res = helper.synced_get_root_dir(forest.to_owned(), private_ref);
                if root_dir_res.is_ok() {
                    let root_dir = root_dir_res.ok().unwrap();
                    let path_segments = prepare_path_segments(path_segments);
                    let mkdir_res =
                        helper.synced_mkdir(forest.to_owned(), root_dir, &path_segments);
                    if mkdir_res.is_ok() {
                        let (cid, private_ref) = mkdir_res.ok().unwrap();
                        trace!("**********************mkDirNative finished**************");
                        serialize_config(cid, private_ref)
                    } else {
                        let msg = mkdir_res.err().unwrap();
                        trace!("wnfsError in mkdirNative: {:?}", msg);
                        return null_mut();
                    }
                } else {
                    let msg = root_dir_res.err().unwrap();
                    trace!("wnfsError in mkdirNative: {:?}", msg);
                    return null_mut();
                }
            } else {
                let msg = forest_res.err().unwrap();
                trace!("wnfsError in mkdirNative: {:?}", msg);
                return null_mut();
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn mv_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        source_path_segments: *const c_char,
        target_path_segments: *const c_char,
    ) -> *mut Config {
        trace!("**********************mvNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest = helper.synced_load_forest(cid).unwrap();
            let root_dir = helper
                .synced_get_root_dir(forest.to_owned(), private_ref)
                .unwrap();
            let source_path_segments = prepare_path_segments(source_path_segments);
            let target_path_segments = prepare_path_segments(target_path_segments);
            let result = helper.synced_mv(
                forest.to_owned(),
                root_dir,
                &source_path_segments,
                &target_path_segments,
            );
            trace!("**********************mvNative finished**************");
            if result.is_ok() {
                let (cid, private_ref) = result.ok().unwrap();
                return serialize_config(cid, private_ref);
            } else {
                trace!("wnfsError occured in mvNative: {:?}", result.err().unwrap());
                return null_mut();
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn cp_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        source_path_segments: *const c_char,
        target_path_segments: *const c_char,
    ) -> *mut Config {
        trace!("**********************cpNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest = helper.synced_load_forest(cid).unwrap();
            let root_dir = helper
                .synced_get_root_dir(forest.to_owned(), private_ref)
                .unwrap();
            let source_path_segments = prepare_path_segments(source_path_segments);
            let target_path_segments = prepare_path_segments(target_path_segments);
            let result = helper.synced_cp(
                forest.to_owned(),
                root_dir,
                &source_path_segments,
                &target_path_segments,
            );
            trace!("**********************mvNative finished**************");
            if result.is_ok() {
                let (cid, private_ref) = result.ok().unwrap();
                return serialize_config(cid, private_ref);
            } else {
                trace!("wnfsError occured in cpNative: {:?}", result.err().unwrap());
                return null_mut();
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn rm_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
    ) -> *mut Config {
        trace!("**********************rmNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest = helper.synced_load_forest(cid).unwrap();
            let root_dir = helper
                .synced_get_root_dir(forest.to_owned(), private_ref)
                .unwrap();
            let path_segments = prepare_path_segments(path_segments);
            let result = helper.synced_rm(forest.to_owned(), root_dir, &path_segments);
            trace!("**********************rmNative finished**************");
            if result.is_ok() {
                let (cid, private_ref) = result.ok().unwrap();
                return serialize_config(cid, private_ref);
            } else {
                trace!("wnfsError occured in rmNative: {:?}", result.err().unwrap());
                return null_mut();
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn ls_native(
        db_path: *const c_char,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        len: *mut i32,
        capacity: *mut i32,
    ) -> *mut u8 {
        trace!("**********************lsNative started**************");
        unsafe {
            let store = KVBlockStore::new(
                CStr::from_ptr(db_path).to_str().unwrap().into(),
                libipld::IpldCodec::DagCbor,
            );
            let block_store = FFIFriendlyBlockStore::new(Box::new(store));
            let helper = &mut PrivateDirectoryHelper::new(block_store);

            let cid = deserialize_cid(cid);
            let private_ref = deserialize_private_ref(private_ref);

            let forest_res = helper.synced_load_forest(cid);
            if forest_res.is_ok() {
                let forest = forest_res.ok().unwrap();
                let root_dir_res = helper.synced_get_root_dir(forest.to_owned(), private_ref);
                if root_dir_res.is_ok() {
                    let root_dir = root_dir_res.ok().unwrap();
                    let path_segments = prepare_path_segments(path_segments);
                    let ls_res =
                        helper.synced_ls_files(forest.to_owned(), root_dir, &path_segments);
                    if ls_res.is_ok() {
                        let output = prepare_ls_output(ls_res.ok().unwrap());
                        trace!("**********************lsNative finished**************");
                        if output.is_ok() {
                            let res = &mut output.ok().unwrap();
                            return vec_to_c_array(res, len, capacity);
                        } else {
                            trace!(
                                "wnfsError occured in lsNative output: {:?}",
                                output.err().unwrap().to_string()
                            );
                            let empty_bytes = &mut vec![0];
                            return vec_to_c_array(&mut empty_bytes.to_owned(), len, capacity);
                        }
                    } else {
                        trace!(
                            "wnfsError occured in lsNative ls_res: {:?}",
                            ls_res.err().unwrap().to_string()
                        );
                        let empty_bytes: Vec<u8> = vec![0];
                        return vec_to_c_array(&mut empty_bytes.to_owned(), len, capacity);
                    }
                } else {
                    trace!(
                        "wnfsError occured in lsNative root_dir_res: {:?}",
                        root_dir_res.err().unwrap().to_string()
                    );
                    let empty_bytes: Vec<u8> = vec![0];
                    return vec_to_c_array(&mut empty_bytes.to_owned(), len, capacity);
                }
            } else {
                trace!(
                    "wnfsError occured in lsNative forest_res: {:?}",
                    forest_res.err().unwrap().to_string()
                );
                let empty_bytes: Vec<u8> = vec![0];
                return vec_to_c_array(&mut empty_bytes.to_owned(), len, capacity);
            }
        }
    }

    pub fn serialize_config(cid: Cid, private_ref: PrivateRef) -> *mut Config {
        trace!("**********************serialize_config started**************");
        Box::into_raw(Box::new(Config {
            cid: serialize_cid(cid),
            private_ref: serialize_private_ref(private_ref),
        }))
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

    pub fn serialize_private_ref(private_ref: PrivateRef) -> *mut c_char {
        CString::new(serde_json::to_string(&private_ref).unwrap())
            .expect("Failed to create private ref string")
            .into_raw()
    }

    pub unsafe fn deserialize_private_ref(private_ref: *const c_char) -> PrivateRef {
        let private_ref: String = CStr::from_ptr(private_ref)
            .to_str()
            .expect("Failed to parse private ref")
            .into();
        let pref = serde_json::from_str::<PrivateRef>(&private_ref).unwrap();
        trace!("**********************deserialize_pref started**************");
        trace!("**********************deserialize_pref pref={:?}", pref);
        pref
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

    pub unsafe fn c_array_to_vec(
        size: libc::size_t,
        array_pointer: *const libc::uint8_t,
    ) -> Vec<u8> {
        std::slice::from_raw_parts(array_pointer as *const u8, size as usize).to_vec()
    }

    pub fn vec_to_c_array(buf: &mut Vec<u8>, len: *mut i32, capacity: *mut i32) -> *mut u8 {
        unsafe {
            *len = buf.len() as i32;
            *capacity = buf.capacity() as i32;
        }
        let ptr = buf.as_mut_ptr();

        std::mem::forget(ptr); // so that it is not destructed at the end of the scope
        ptr
    }

    #[no_mangle]
    pub extern "C" fn config_free(ptr: *mut Config) {
        if ptr.is_null() {
            return;
        }
        unsafe {
            Box::from_raw(ptr);
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
}

#[cfg(test)]
mod ios_tests {
    use std::ffi::CString;

    use crate::ios::create_private_forest_native;

    unsafe fn test_overall(){
        let forest_cid = create_private_forest_native(CString::new("./tmp/test_db").unwrap().into_raw());
    }
    
}