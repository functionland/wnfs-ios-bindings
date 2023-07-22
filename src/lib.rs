pub mod blockstore;
pub mod blockstore_interface;
pub mod c_types;
pub mod tests;
pub mod ios {
    extern crate libc;
    use crate::blockstore::BridgedStore;
    use crate::blockstore_interface::BlockStoreInterface;
    use crate::c_types::{
        prepare_ls_output, prepare_path_segments, RustBytes, RustResult, RustString, RustVoid,
    };
    use libipld::Cid;
    use log::trace;
    use std::boxed::Box;

    use wnfsutils::blockstore::FFIFriendlyBlockStore;
    use wnfsutils::private_forest::PrivateDirectoryHelper;

    #[no_mangle]
    pub extern "C" fn load_with_wnfs_key_native(
        block_store_interface: BlockStoreInterface,
        wnfs_key: RustBytes,
        cid: RustString,
    ) -> RustResult<RustVoid> {
        trace!("**********************load_with_wnfs_key_native started**************");
        unsafe {
            let store = BridgedStore::new(block_store_interface);
            let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
            let wnfs_key: Vec<u8> = wnfs_key.into();
            let cid_res: Result<Cid, String> = cid.try_into();
            if cid_res.is_err() {
                RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
            } else {
                let cid = cid_res.unwrap();
                let helper_res =
                    PrivateDirectoryHelper::synced_load_with_wnfs_key(block_store, cid, wnfs_key);
                trace!("**********************load_with_wnfs_key_native finished**************");
                if helper_res.is_ok() {
                    RustResult::ok(RustVoid {})
                } else {
                    let msg = helper_res.err().unwrap();
                    trace!("wnfsError in load_with_wnfs_key_native: {:?}", msg);
                    RustResult::error(msg.into())
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn init_native(
        block_store_interface: BlockStoreInterface,
        wnfs_key: RustBytes,
    ) -> RustResult<RustString> {
        trace!("**********************init_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let wnfs_key: Vec<u8> = wnfs_key.into();
        let helper_res = PrivateDirectoryHelper::synced_init(block_store, wnfs_key);

        if helper_res.is_ok() {
            let (_, _, cid) = helper_res.unwrap();
            RustResult::ok(RustString::from(cid))
        } else {
            let msg = helper_res.err().unwrap();
            trace!("wnfsError in init_native: {:?}", msg.to_owned());
            unsafe { RustResult::error(msg.to_owned().into()) }
        }
    }

    #[no_mangle]
    pub extern "C" fn write_file_from_path_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,

        path_segments: RustString,
        _filename: RustString,
    ) -> RustResult<RustString> {
        trace!("**********************write_file_from_path_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
            let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

            if helper_res.is_ok() {
                let helper = &mut helper_res.ok().unwrap();
                let path_segments = unsafe { prepare_path_segments(path_segments) };

                let filename = unsafe { _filename.into() };
                trace!("filename, path: {:?} -- {:?}", filename, path_segments);
                let write_file_result =
                    helper.synced_write_file_from_path(&path_segments, &filename);
                trace!("**********************write_file_from_path_native finished**************");
                if write_file_result.is_ok() {
                    let cid = write_file_result.ok().unwrap();
                    unsafe {
                        return RustResult::ok(cid.into());
                    }
                } else {
                    let msg = write_file_result.err().unwrap();
                    trace!("wnfsError in write_file_from_path_native: {:?}", msg);
                    unsafe { return RustResult::ok(msg.into()) }
                }
            } else {
                let msg = &mut helper_res.err().unwrap();
                trace!(
                    "wnfsError in write_file_from_path_native: {:?}",
                    msg.to_owned()
                );
                RustResult::error(msg.to_owned().into())
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn read_filestream_to_path_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,

        path_segments: RustString,
        _filename: RustString,
    ) -> RustResult<RustString> {
        trace!("wnfs11 **********************read_filestream_to_path_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
            let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);
            if helper_res.is_ok() {
                let helper = &mut helper_res.ok().unwrap();
                let path_segments = unsafe { prepare_path_segments(path_segments) };
                let filename = unsafe { _filename.into() };

                trace!("wnfs11 **********************read_filestream_to_path_native filename created**************");
                let result = helper.synced_read_filestream_to_path(&filename, &path_segments, 0);
                trace!("wnfs11 **********************read_filestream_to_path_native finished**************");
                if result.is_ok() {
                    unsafe { return RustResult::ok(filename.into()) }
                } else {
                    let err = result.err().unwrap();
                    trace!(
                        "wnfsError occured in read_filestream_to_path_native on result: {:?}",
                        err.to_owned()
                    );
                    unsafe { return RustResult::error(err.to_owned().into()) }
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!(
                    "wnfsError in read_filestream_to_path_native: {:?}",
                    msg.to_owned()
                );
                RustResult::error(msg.to_owned().into())
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn read_file_to_path_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,
        path_segments: RustString,
        _filename: RustString,
    ) -> RustResult<RustString> {
        trace!("wnfs11 **********************read_file_to_path_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
            let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

            if helper_res.is_ok() {
                let helper = &mut helper_res.ok().unwrap();
                let path_segments = unsafe { prepare_path_segments(path_segments) };
                let filename = unsafe { _filename.into() };

                trace!("wnfs11 **********************read_file_to_path_native filename created**************");
                let result = helper.synced_read_file_to_path(&path_segments, &filename);
                trace!(
                    "wnfs11 **********************read_file_to_path_native finished**************"
                );
                if result.is_ok() {
                    unsafe { return RustResult::ok(filename.into()) }
                } else {
                    let err = result.err().unwrap();
                    trace!(
                        "wnfsError occured in read_file_to_path_native on result: {:?}",
                        err.to_owned()
                    );
                    unsafe { return RustResult::error(err.to_owned().into()) }
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!(
                    "wnfsError in read_file_to_path_native: {:?}",
                    msg.to_owned()
                );
                RustResult::error(msg.to_owned().into())
            }
        }
    }
    #[no_mangle]
    pub extern "C" fn write_file_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,
        path_segments: RustString,
        _content: RustBytes,
    ) -> RustResult<RustString> {
        trace!("**********************write_file_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
            let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

            if helper_res.is_ok() {
                let helper = &mut helper_res.ok().unwrap();
                let path_segments = unsafe { prepare_path_segments(path_segments) };
                let content: Vec<u8> = _content.into();
                let write_file_res = helper.synced_write_file(&path_segments, content, 0);
                trace!("**********************write_file_native finished**************");
                if write_file_res.is_ok() {
                    let cid = write_file_res.ok().unwrap();
                    RustResult::ok(RustString::from(cid))
                } else {
                    let msg = write_file_res.err().unwrap();
                    trace!("wnfsError in write_file_native: {:?}", msg);
                    RustResult::error(msg.to_owned().into())
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!("wnfsError in write_file_native: {:?}", msg.to_owned());
                RustResult::error(msg.to_owned().into())
            }
        }
    }
    #[no_mangle]
    pub extern "C" fn read_file_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,
        path_segments: RustString,
    ) -> RustResult<RustBytes> {
        trace!("**********************read_file_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
            let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);
            if helper_res.is_ok() {
                let helper = &mut helper_res.ok().unwrap();
                let path_segments = unsafe { prepare_path_segments(path_segments) };
                trace!("**********************read_file_native finished**************");
                let result = helper.synced_read_file(&path_segments);
                if result.is_ok() {
                    RustResult::ok(result.unwrap().into())
                } else {
                    let msg = result.err().unwrap();
                    trace!("wnfsError in read_file_native: {:?}", msg);
                    RustResult::error(msg.to_owned().into())
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!("wnfsError in read_file_native: {:?}", msg.to_owned());
                RustResult::error(msg.to_owned().into())
            }
        }
    }
    #[no_mangle]
    pub extern "C" fn mkdir_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,

        path_segments: RustString,
    ) -> RustResult<RustString> {
        trace!("**********************mkdir_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
            let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

            if helper_res.is_ok() {
                let helper = &mut helper_res.ok().unwrap();
                let path_segments = unsafe { prepare_path_segments(path_segments) };
                let mkdir_res = helper.synced_mkdir(&path_segments);
                if mkdir_res.is_ok() {
                    let cid = mkdir_res.ok().unwrap();
                    trace!("**********************mkdir_native finished**************");
                    RustResult::ok(cid.into())
                } else {
                    let msg = mkdir_res.err().unwrap();
                    trace!("wnfsError in mkdir_native: {:?}", msg.to_owned());
                    RustResult::error(msg.to_owned().into())
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!("wnfsError in mkdir_native: {:?}", msg.to_owned());
                RustResult::error(msg.to_owned().into())
            }
        }
    }
    #[no_mangle]
    pub extern "C" fn mv_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,

        source_path_segments: RustString,
        target_path_segments: RustString,
    ) -> RustResult<RustString> {
        trace!("**********************mv_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
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
                        return RustResult::ok(cid.into());
                    }
                } else {
                    let msg = result.err().unwrap();
                    trace!("wnfsError occured in mv_native: {:?}", msg.to_owned());
                    RustResult::error(msg.to_owned().into())
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!("wnfsError in mv_native: {:?}", msg.to_owned());
                RustResult::error(msg.to_owned().into())
            }
        }
    }
    #[no_mangle]
    pub extern "C" fn cp_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,

        source_path_segments: RustString,
        target_path_segments: RustString,
    ) -> RustResult<RustString> {
        trace!("**********************cp_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
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
                        return RustResult::ok(cid.into());
                    }
                } else {
                    let msg = result.err().unwrap();
                    trace!("wnfsError occured in cp_native: {:?}", msg.to_owned());
                    RustResult::error(msg.to_owned().into())
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!("wnfsError in cp_native: {:?}", msg.to_owned());
                RustResult::error(msg.to_owned().into())
            }
        }
    }
    #[no_mangle]
    pub extern "C" fn rm_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,

        path_segments: RustString,
    ) -> RustResult<RustString> {
        trace!("**********************rm_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
            let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);

            if helper_res.is_ok() {
                let helper = &mut helper_res.ok().unwrap();
                let path_segments = unsafe { prepare_path_segments(path_segments) };
                let rm_res = helper.synced_rm(&path_segments);
                if rm_res.is_ok() {
                    let cid = rm_res.ok().unwrap();
                    trace!("**********************rm_native finished**************");
                    RustResult::ok(cid.into())
                } else {
                    let msg = rm_res.err().unwrap();
                    trace!("wnfsError in rm_native: {:?}", msg.to_owned());
                    RustResult::error(msg.to_owned().into())
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!("wnfsError in rm_native: {:?}", msg.to_owned());
                RustResult::error(msg.to_owned().into())
            }
        }
    }
    #[no_mangle]
    pub extern "C" fn ls_native(
        block_store_interface: BlockStoreInterface,
        cid: RustString,
        path_segments: RustString,
    ) -> RustResult<RustBytes> {
        trace!("**********************ls_native started**************");
        let store = BridgedStore::new(block_store_interface);
        let block_store = &mut FFIFriendlyBlockStore::new(Box::new(store));
        let cid_res: Result<Cid, String> = cid.try_into();
        if cid_res.is_err() {
            RustResult::error(RustString::from(cid_res.err().unwrap().to_string()))
        } else {
            let cid = cid_res.unwrap();
            let helper_res = PrivateDirectoryHelper::synced_reload(block_store, cid);
            if helper_res.is_ok() {
                let helper = &mut helper_res.ok().unwrap();
                let path_segments = unsafe { prepare_path_segments(path_segments) };
                let ls_res = helper.synced_ls_files(&path_segments);
                if ls_res.is_ok() {
                    let output = prepare_ls_output(ls_res.ok().unwrap());
                    trace!("**********************ls_native finished**************");
                    if output.is_ok() {
                        let res = output.ok().unwrap();
                        RustResult::ok(res.into())
                    } else {
                        let msg = output.err().unwrap().to_string();
                        trace!(
                            "wnfsError occured in ls_native output: {:?}",
                            msg.to_owned()
                        );
                        RustResult::error(msg.to_owned().into())
                    }
                } else {
                    let msg = ls_res.err().unwrap();
                    trace!(
                        "wnfsError occured in ls_native ls_res: {:?}",
                        msg.to_owned()
                    );
                    RustResult::error(msg.to_owned().into())
                }
            } else {
                let msg = helper_res.err().unwrap();
                trace!(
                    "wnfsError occured in ls_native forest_res: {:?}",
                    msg.to_owned()
                );
                RustResult::error(msg.to_owned().into())
            }
        }
    }
}
