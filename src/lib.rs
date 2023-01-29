pub mod store;
pub mod ios {
    extern crate libc;
    use anyhow::Result;
    use libc::c_void;

    use ::core::mem::MaybeUninit as MU;
    use libipld::Cid;
    use log::trace;
    use std::boxed::Box;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    use std::ptr::null_mut;
    use wnfs::private::PrivateRef;
    use wnfs::Metadata;
    use wnfsutils::blockstore::FFIFriendlyBlockStore;

    use wnfsutils::private_forest::PrivateDirectoryHelper;
    use crate::store::BridgedStore;


    #[repr(C)]
    pub struct Config {
        pub cid: *const c_char,
        pub private_ref: *const c_char,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct BlockStoreInterface {
        pub putdata: *mut c_void,
        pub getdata: *mut c_void,
        pub put_fn: extern "C" fn(putdata: *mut c_void, bytes: *const u8, bytes_size: *const libc::size_t, result_size: *mut libc::size_t, codec: i64) -> *const u8,
        pub get_fn: extern "C" fn(getdata: *mut c_void, cid: *const u8, cid_size: *const libc::size_t , result_size: *mut libc::size_t) -> *const u8,
    }

    unsafe impl Send for BlockStoreInterface {}

    impl BlockStoreInterface {
        pub fn put(self,  bytes: *const u8, bytes_size: *const libc::size_t, result_size: *mut libc::size_t, codec: i64) -> *const u8 {
            let result =  (self.put_fn)(self.putdata,  bytes, bytes_size, result_size, codec);
            std::mem::forget(self);
            result
        }
        pub fn get(self, cid: *const u8, cid_size: *const libc::size_t , result_size: *mut libc::size_t) -> *const u8 {
            let result = (self.get_fn)(self.getdata, cid, cid_size, result_size);
            std::mem::forget(self);
            result
        }
    }

    // TODO: fixme
    // impl Drop for BlockStoreInterface {
    //     fn drop(&mut self) {
    //         panic!("BlockStoreInterface must have explicit put or get call")
    //     }
    // }

    #[no_mangle]
    pub unsafe extern "C" fn create_private_forest_native(block_store_interface: BlockStoreInterface) -> *mut c_char {
        trace!("**********************createPrivateForest started**************");
        let store = BridgedStore::new(
            block_store_interface
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
        block_store_interface: BlockStoreInterface,
        wnfs_key_arr_size: libc::size_t,
        wnfs_key_arr_pointer: *const u8,
        cid: *const c_char,
    ) -> *mut c_char {
        trace!("**********************getPrivateRefNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
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
        block_store_interface: BlockStoreInterface,
        wnfs_key_arr_size: libc::size_t,
        wnfs_key_arr_pointer: *const u8,
        cid: *const c_char,
    ) -> *mut Config {
        trace!("**********************createRootDirNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface

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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        filename: *const c_char,
    ) -> *mut Config {
        trace!("**********************writeFileFromPathNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        filename: *const c_char,
    ) -> *mut c_char {
        trace!("wnfs11 **********************readFilestreamToPathNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        filename: *const c_char,
    ) -> *mut c_char {
        trace!("wnfs11 **********************readFileToPathNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        content_arr_size: libc::size_t,
        content_arr_pointer: *const u8,
    ) -> *mut Config {
        trace!("**********************writeFileNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        len: *mut libc::size_t,
        capacity: *mut libc::size_t,
    ) -> *mut u8 {
        trace!("**********************readFileNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
    ) -> *mut Config {
        trace!("**********************mkDirNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        source_path_segments: *const c_char,
        target_path_segments: *const c_char,
    ) -> *mut Config {
        trace!("**********************mvNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        source_path_segments: *const c_char,
        target_path_segments: *const c_char,
    ) -> *mut Config {
        trace!("**********************cpNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
    ) -> *mut Config {
        trace!("**********************rmNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
                
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
        block_store_interface: BlockStoreInterface,
        cid: *const c_char,
        private_ref: *const c_char,
        path_segments: *const c_char,
        len: *mut libc::size_t,
        capacity: *mut libc::size_t,
    ) -> *mut u8 {
        trace!("**********************lsNative started**************");
        unsafe {
            let store = BridgedStore::new(
                block_store_interface
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

    pub unsafe fn c_array_to_vec(size: libc::size_t, array_pointer: *const u8) -> Vec<u8> {
        std::slice::from_raw_parts(array_pointer as *const u8, size as usize).to_vec()
    }

    pub unsafe fn vec_to_c_array(
        buf: &mut Vec<u8>,
        len: *mut usize,
        capacity: *mut usize,
    ) -> *mut u8 {
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
    pub extern "C" fn config_free(ptr: *mut Config) {
        if ptr.is_null() {
            return;
        }
        unsafe {
            drop(Box::from_raw(ptr));
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
    use crate::ios::*;
    use libc::c_void;
    use once_cell::sync::Lazy;
    use sha256::digest;
    use wnfsutils::{kvstore::KVBlockStore, blockstore::FFIStore};
    use std::{
        default,
        ffi::{CStr, CString},
        fs,
        os::raw::c_char,
    };
    use libipld::{cbor::DagCborCodec, codec::Encode, IpldCodec};


    fn string_to_cstring(str_in: String) -> *const c_char {
        return CString::new(str_in).unwrap().into_raw().cast_const();
    }

    unsafe fn cstring_to_string(str_in: *mut c_char) -> String {
        return CStr::from_ptr(str_in).to_str().unwrap().to_string();
    }

    unsafe fn test_cfg(cfg: *mut Config) -> (*const c_char, *const c_char) {
        assert!(!cfg.is_null(), "config should not be null");
        assert!(!(*cfg).cid.is_null(), "cid should not be null");
        assert!(
            !(*cfg).private_ref.is_null(),
            "private_ref should not be null"
        );
        let cid: String = CStr::from_ptr((*cfg).cid).to_str().unwrap().into();
        let private_ref: String = CStr::from_ptr((*cfg).private_ref).to_str().unwrap().into();
        println!("cid: {:?}", cid);
        println!("private_ref: {:?}", private_ref);
        (string_to_cstring(cid), string_to_cstring(private_ref))
    }

    #[test]
    fn test_c_array() {
        unsafe {
            let mut test_content: String = "Hello, World!".into();
            let mut len: usize = 0;
            let mut capacity: usize = 0;
            let content_from_path =
                vec_to_c_array(test_content.as_mut_vec(), &mut len, &mut capacity);
            println!("len: {}, cap: {}", len, capacity);
            let content =
                String::from_utf8(c_array_to_vec(len as libc::size_t, content_from_path)).unwrap();
            assert_eq!(content, test_content.to_owned().to_string());
        }
    }

    
    static store: Lazy<KVBlockStore> = Lazy::new(|| KVBlockStore::new(String::from("./tmp/test_db"), IpldCodec::DagCbor));
    

    extern "C" fn getdata(_cid: *const u8, cid_size: *const libc::size_t , result_size: *mut libc::size_t) -> *const u8{
        let mut capacity: usize = 0;
        let mut size: usize = 0;
        unsafe{
            let cid = c_array_to_vec(*cid_size ,_cid);
            let result = &mut store.get_block(cid).unwrap();
            vec_to_c_array(result, result_size, &mut capacity)
        }
    }

    extern "C" fn putdata(_bytes: *const u8, bytes_size: *const libc::size_t, result_size: *mut libc::size_t, codec: i64) -> *const u8{
        let mut capacity: usize = 0;
        let mut size: usize = 0;
        unsafe{
            let bytes = c_array_to_vec(*bytes_size ,_bytes);
            let result = &mut store.put_block(bytes, codec).unwrap();
            vec_to_c_array(result, result_size, &mut capacity)
        }
    }

    extern fn get(fngetdata: *mut c_void, _cid: *const u8, cid_size: *const libc::size_t , result_size: *mut libc::size_t) -> *const u8{
            getdata(_cid, cid_size, result_size)
    }

    extern fn put(fnputdata: *mut c_void, _bytes: *const u8, bytes_size: *const libc::size_t, result_size: *mut libc::size_t, codec: i64) -> *const u8{
            putdata(_bytes, bytes_size, result_size, codec)
    }

    fn get_block_store_interface() -> BlockStoreInterface{
        let result = BlockStoreInterface{
            putdata: &mut putdata  as *mut _ as *mut c_void,
            getdata: &mut getdata  as *mut _ as *mut c_void,
            put_fn: put,
            get_fn: get,
        };
        std::mem::forget(&result);
        result
    }

    #[test]
    fn test_overall() {
        unsafe {
            let wnfs_key_string = digest("test");
            let wnfs_key = CString::new(wnfs_key_string.to_owned())
                .unwrap()
                .into_raw()
                .cast_const();
            let forest_cid = create_private_forest_native(get_block_store_interface());

            let mut cfg = create_root_dir_native(
                get_block_store_interface(),
                wnfs_key_string.to_owned().as_bytes().len() as libc::size_t,
                wnfs_key as *const u8,
                forest_cid,
            );
            let (mut cid, mut private_ref) = test_cfg(cfg);

            /*
            let mut len: usize = 0;
            let mut capacity: usize = 0;
            let filenames_initial = ls_native(
                get_block_store_interface()
                ,string_to_cstring("bafyreieqp253whdfdrky7hxpqezfwbkjhjdbxcq4mcbp6bqf4jdbncbx4y".into())
                ,string_to_cstring("{\"saturated_name_hash\":[229,31,96,28,24,238,207,22,36,150,191,37,235,68,191,144,219,250,5,97,85,208,156,134,137,74,25,209,6,66,250,127],\"content_key\":[172,199,245,151,207,21,26,76,52,109,93,57,118,232,9,230,149,46,37,137,174,42,119,29,102,175,25,149,213,204,45,15],\"revision_key\":[17,5,78,59,8,135,144,240,41,248,135,168,222,186,158,240,100,10,129,4,180,55,126,115,146,239,22,177,207,118,169,51]}".into())
                ,string_to_cstring("root/".into()),
                &mut len, &mut capacity
            );
            let names = String::from_raw_parts(filenames_initial, len as usize, capacity as usize);
            println!("ls_initial. filenames_initial={}", names);
            cbytes_free(filenames_initial, len, capacity);
            */
            // Write file
            let test_content = "Hello, World!";
            fs::write("./tmp/test.txt", test_content.to_owned()).expect("Unable to write file");

            // Read file
            {
                cfg = write_file_from_path_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompath.txt".into()),
                    string_to_cstring("./tmp/test.txt".into()),
                );
                (cid, private_ref) = test_cfg(cfg);

                let mut len: usize = 0;
                let mut capacity: usize = 0;
                let content_from_path = read_file_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompath.txt".into()),
                    &mut len,
                    &mut capacity,
                );

                let content =
                    String::from_utf8(c_array_to_vec(len as libc::size_t, content_from_path))
                        .unwrap();
                assert_eq!(content, test_content.to_owned().to_string());
                println!("read_file_from_path. content={}", content);
            }
            // Read content from path to path
            {
                let content_from_path_topath = read_file_to_path_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompath.txt".into()),
                    string_to_cstring("./tmp/test2.txt".into()),
                );
                let content_str = cstring_to_string(content_from_path_topath);
                println!("content_from_path_topath={}", content_str);
                assert!(
                    !content_from_path_topath.is_null(),
                    "content_from_path_topath should not be null"
                );
                let read_content = fs::read_to_string(content_str).expect("Unable to read file");
                assert_eq!(read_content, test_content.to_string());
                println!("read_file_from_path_of_read_to. content={}", read_content);
            }
            // Read content from file stream to path
            {
                let content_stream_from_path_topath = read_filestream_to_path_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompath.txt".into()),
                    string_to_cstring("./tmp/teststream.txt".into()),
                );
                let content_str = cstring_to_string(content_stream_from_path_topath);
                println!("content_stream_from_path_topath={}", content_str);
                assert!(
                    !content_stream_from_path_topath.is_null(),
                    "content_stream_from_path_topath should not be null"
                );
                let read_content = fs::read_to_string(content_str).expect("Unable to read file");
                assert_eq!(read_content, test_content.to_string());
                println!("read_file_from_path_of_read_to. content={}", read_content);
            }
            // CP: target folder must exists
            {
                let mut len: usize = 0;
                let mut capacity: usize = 0;
                cfg = cp_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompath.txt".into()),
                    string_to_cstring("root/testfrompathcp.txt".into()),
                );
                (cid, private_ref) = test_cfg(cfg);
                let content_cp = read_file_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompathcp.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content =
                    String::from_utf8(c_array_to_vec(len as libc::size_t, content_cp)).unwrap();
                println!("cp. content_cp={}", content);
                assert_eq!(content, test_content.to_string());
            }
            // MV: target folder must exists
            {
                let mut len: usize = 0;
                let mut capacity: usize = 0;
                cfg = mv_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompath.txt".into()),
                    string_to_cstring("root/testfrompathmv.txt".into()),
                );
                (cid, private_ref) = test_cfg(cfg);
                let content_mv = read_file_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompathmv.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content =
                    String::from_utf8(c_array_to_vec(len as libc::size_t, content_mv)).unwrap();
                println!("mv. content_mv={}", content);
                assert_eq!(content, test_content.to_string());
            }
            // RM#1
            {
                let mut len: usize = 0;
                let mut capacity: usize = 0;
                cfg = rm_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompathmv.txt".into()),
                );
                (cid, private_ref) = test_cfg(cfg);
                let content_rm1 = read_file_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompathmv.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content =
                    String::from_utf8(c_array_to_vec(len as libc::size_t, content_rm1)).unwrap();
                println!("rm#1. content_rm#1={}", content);
                assert_eq!(content, "".to_string());
            }
            // RM#2
            {
                let mut len: usize = 0;
                let mut capacity: usize = 0;
                cfg = rm_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompathcp.txt".into()),
                );
                (cid, private_ref) = test_cfg(cfg);
                let content_rm2 = read_file_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/testfrompathcp.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content =
                    String::from_utf8(c_array_to_vec(len as libc::size_t, content_rm2)).unwrap();
                println!("rm#1. content_rm#1={}", content);
                assert_eq!(content, "".to_string());
            }
            //
            {
                let mut len: usize = 0;
                let mut capacity: usize = 0;
                println!(
                    "********************** test content: {}",
                    test_content.to_owned()
                );
                let test_content_ptr = vec_to_c_array(
                    test_content.to_string().as_mut_vec(),
                    &mut len,
                    &mut capacity,
                );
                cfg = write_file_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/test.txt".into()),
                    len,
                    test_content_ptr,
                );
                (cid, private_ref) = test_cfg(cfg);

                cfg = mkdir_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/test1".into()),
                );
                (cid, private_ref) = test_cfg(cfg);

                let content_ls = ls_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let file_names =
                    String::from_utf8(c_array_to_vec(len as libc::size_t, content_ls)).unwrap();
                println!("ls. fileNames={}", file_names);

                let content_test = read_file_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/test.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content =
                    String::from_utf8(c_array_to_vec(len as libc::size_t, content_test)).unwrap();
                println!("read. content={}", content);
                assert_eq!(content, test_content.to_string());
            }
            println!("All tests before reload passed");

            // Testing reload Directory
            {
                let mut len: usize = 0;
                let mut capacity: usize = 0;
                println!(
                    "wnfs12 Testing reload with cid={} & wnfsKey={}",
                    cstring_to_string(cid.cast_mut()),
                    wnfs_key_string
                );
                let private_ref_reload = get_private_ref_native(
                    get_block_store_interface(),
                    wnfs_key_string.to_owned().as_bytes().len() as libc::size_t,
                    wnfs_key as *const u8,
                    cid,
                );
                println!(
                    "wnfs12 original PrivateRef. private_ref={}",
                    cstring_to_string(private_ref.cast_mut())
                );
                println!(
                    "wnfs12 getPrivateRef. private_ref={}",
                    cstring_to_string(private_ref_reload)
                );
                assert!(
                    !private_ref_reload.is_null(),
                    "private_ref should not be null"
                );

                let content_reloaded = read_file_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/test.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content =
                    String::from_utf8(c_array_to_vec(len as libc::size_t, content_reloaded))
                        .unwrap();
                println!("read. content={}", content);
                assert_eq!(content, test_content.to_string());
            }
            // Read content from path to path (reloaded)
            {
                let content_from_path_topath_reloaded = read_file_to_path_native(
                    get_block_store_interface(),
                    cid,
                    private_ref,
                    string_to_cstring("root/test.txt".into()),
                    string_to_cstring("./tmp/test2.txt".into()),
                );
                let content_str = cstring_to_string(content_from_path_topath_reloaded);
                println!("content_from_path_topath_reloaded={}", content_str);
                assert!(
                    !content_from_path_topath_reloaded.is_null(),
                    "content_from_path_topath_reloaded should not be null"
                );
                let read_content = fs::read_to_string(content_str).expect("Unable to read file");
                assert_eq!(read_content, test_content.to_string());
                println!("read_file_from_path_of_read_to. content={}", read_content);
            }
        }
    }
}
