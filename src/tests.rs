#[cfg(test)]
mod ios_tests {
    use crate::{
        blockstore_interface::BlockStoreInterface,
        c_types::{RustBytes, RustResult, RustString, RustVoid, prepare_ls_output},
        ios::*,
    };
    use libc::c_void;
    use libipld::Cid;
    use once_cell::sync::Lazy;
    use sha256::digest;
    use std::{fs, ptr};
    use wnfs::common::CODEC_DAG_CBOR;
    use wnfsutils::{blockstore::FFIStore, kvstore::KVBlockStore};

    unsafe fn test_cfg(cfg: RustResult<RustString>) -> Cid {
        assert!(cfg.ok, "config should not be null");
        let cid = cfg.result.try_into().unwrap();
        println!("cid: {:?}", cid);
        cid
    }

    static STORE: Lazy<KVBlockStore> =
        Lazy::new(|| KVBlockStore::new(String::from("./tmp/test_db"), CODEC_DAG_CBOR));

    extern "C" fn get(_userdata: *mut c_void, _cid: RustBytes) -> RustResult<RustBytes> {
        let cid = _cid.into();
        let data = STORE.get_block(cid).unwrap();
        let tmp1 = RustBytes::from(data);
        RustResult::ok(tmp1)
    }

    extern "C" fn put(
        _userdata: *mut c_void,
        _cid: RustBytes,
        _bytes: RustBytes,
    ) -> RustResult<RustVoid> {
        let cid: Vec<u8> = _cid.into();
        let bytes = _bytes.into();
        let _data = STORE.put_block(cid.into(), bytes).unwrap();
        RustResult::ok(RustVoid::void())
    }

    extern "C" fn dealloc_after_get(obj: RustResult<RustBytes>) {
        println!("obj: {}", obj.ok);
    }
    extern "C" fn dealloc_after_put(obj: RustResult<RustVoid>) {
        println!("obj: {}", obj.ok);
    }

    fn get_block_store_interface() -> BlockStoreInterface {
        let userdata: *mut c_void = ptr::null_mut();
        let result = BlockStoreInterface {
            userdata: userdata,
            put_fn: put,
            get_fn: get,
            dealloc_after_get: dealloc_after_get,
            dealloc_after_put: dealloc_after_put,
        };
        std::mem::forget(&result);
        result
    }

    #[test]
    fn test_overall() {
        unsafe {
            let wnfs_key = &mut digest("test").as_bytes().to_vec();
            let wnfs_key_string = wnfs_key[..32].to_vec();
            println!("wnfs_key length: {}", wnfs_key_string.len());

            let mut cfg = init_native(
                get_block_store_interface(),
                wnfs_key_string.to_owned().into(),
            );
            let mut cid = test_cfg(cfg);

            
            let filenames_initial = ls_native(
                get_block_store_interface()
                ,cid.into()
                ,RustString::from(String::from("root/"))
            );

            let names: Vec<u8> = filenames_initial.result.into();
            println!("ls_initial. filenames_initial={:?}", names);
            // Write file
            let test_content = "Hello, World!";
            fs::write("./tmp/test.txt", test_content.to_owned()).expect("Unable to write file");

            // Read large file
            {
                let large_data = vec![0u8; 500 * 1024 * 1024];
                cfg = write_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/test_large.bin".to_string()),
                    large_data.to_owned().into(),
                );
                cid = test_cfg(cfg);

                let content_large = read_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/test_large.bin".to_string()),
                );

                let content: Vec<u8> = content_large.result.into();
                assert_eq!(content.to_owned(), large_data.to_owned());
                if true {
                    return 
                }
            }
            // Read file
            {
                cfg = write_file_from_path_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompath.txt".to_string()),
                    RustString::from("./tmp/test.txt".to_string()),
                );
                cid = test_cfg(cfg);

                let content_from_path = read_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompath.txt".to_string()),
                );

                let content = String::from_utf8(content_from_path.result.into()).unwrap();
                assert_eq!(content, test_content.to_owned().to_string());
                println!("read_file_from_path. content={}", content);
            }
            // Read content from path to path
            {
                let content_from_path_topath = read_file_to_path_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompath.txt".to_string()),
                    RustString::from("./tmp/test2.txt".to_string()),
                );
                let content_str: String = (content_from_path_topath).result.into();
                println!("content_from_path_topath={}", content_str);
                let read_content = fs::read_to_string(content_str).expect("Unable to read file");
                assert_eq!(read_content, test_content.to_string());
                println!("read_file_from_path_of_read_to. content={}", read_content);
            }
            // Read content from file stream to path
            {
                let content_stream_from_path_topath = read_filestream_to_path_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompath.txt".to_string()),
                    RustString::from("./tmp/teststream.txt".to_string()),
                );
                let content_str: String = content_stream_from_path_topath.result.into();
                println!("content_stream_from_path_topath={}", content_str);
                let read_content = fs::read_to_string(content_str).expect("Unable to read file");
                assert_eq!(read_content, test_content.to_string());
                println!("read_file_from_path_of_read_to. content={}", read_content);
            }
            // CP: target folder must exists
            {
                let _len: usize = 0;
                let _capacity: usize = 0;

                cfg = mkdir_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/test1".to_string()),
                );
                cid = test_cfg(cfg);

                cfg = cp_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompath.txt".to_string()),
                    RustString::from("root/testfrompathcp.txt".to_string()),
                );
                cid = test_cfg(cfg);

                let content_cp = read_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompathcp.txt".to_string()),
                );
                let content: String = String::from_utf8(content_cp.result.into()).unwrap();
                println!("cp. content_cp={}", content);
                assert_eq!(content, test_content.to_string());
            }
            // MV: target folder must exists
            {
                let len: usize = 0;
                let capacity: usize = 0;
                cfg = mv_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompath.txt".to_string()),
                    RustString::from("root/testfrompathmv.txt".to_string()),
                );
                cid = test_cfg(cfg);
                let content_mv = read_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompathmv.txt".to_string()),
                );
                println!("len: {}, cap: {}", len, capacity);
                let content: String = String::from_utf8(content_mv.result.into()).unwrap();
                println!("mv. content_mv={}", content);
                assert_eq!(content, test_content.to_string());
            }
            // RM#1
            {
                let len: usize = 0;
                let capacity: usize = 0;
                cfg = rm_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompathmv.txt".to_string()),
                );
                cid = test_cfg(cfg);
                let content_rm1 = read_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompathmv.txt".to_string()),
                );
                println!("len: {}, cap: {}", len, capacity);
                let content: String = String::from_utf8(content_rm1.result.into()).unwrap();
                println!("rm#1. content_rm#1={}", content);
                assert_eq!(content, "".to_string());
            }
            // RM#2
            {
                let len: usize = 0;
                let capacity: usize = 0;
                cfg = rm_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompathcp.txt".to_string()),
                );
                cid = test_cfg(cfg);
                let content_rm2 = read_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/testfrompathcp.txt".to_string()),
                );
                println!("len: {}, cap: {}", len, capacity);
                let content: String = String::from_utf8(content_rm2.result.into()).unwrap();
                println!("rm#1. content_rm#1={}", content);
                assert_eq!(content, "".to_string());
            }
            //
            {
                println!(
                    "********************** test content: {}",
                    test_content.to_owned()
                );
                cfg = write_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/test.txt".to_string()),
                    test_content.as_bytes().to_vec().into(),
                );
                cid = test_cfg(cfg);

                cfg = mkdir_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/test1".to_string()),
                );
                cid = test_cfg(cfg);

                let content_ls = ls_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root".to_string()),
                );

                let file_names = String::from_utf8(content_ls.result.into()).unwrap();
                println!("ls. fileNames={}", file_names);
                let content_test = read_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/test.txt".to_string()),
                );

                let content: String = String::from_utf8(content_test.result.into()).unwrap();
                println!("read. content={}", content);
                assert_eq!(content, test_content.to_string());
            }
            println!("All tests before reload passed");

            // Testing reload Directory
            {
                println!(
                    "wnfs12 Testing reload with cid={} & wnfsKey={:?}",
                    cid.to_string(),
                    wnfs_key_string
                );
                load_with_wnfs_key_native(
                    get_block_store_interface(),
                    wnfs_key_string.to_owned().into(),
                    cid.into(),
                );

                let content_reloaded = read_file_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/test.txt".to_string()),
                );
                let content: String = String::from_utf8(content_reloaded.result.into()).unwrap();
                println!("read. content={}", content);
                assert_eq!(content, test_content.to_string());
            }
            // Read content from path to path (reloaded)
            {
                let content_from_path_topath_reloaded = read_file_to_path_native(
                    get_block_store_interface(),
                    cid.into(),
                    RustString::from("root/test.txt".to_string()),
                    RustString::from("./tmp/test2.txt".to_string()),
                );
                let content_str: String = content_from_path_topath_reloaded.result.into();
                println!("content_from_path_topath_reloaded={}", content_str);
                let read_content = fs::read_to_string(content_str).expect("Unable to read file");
                assert_eq!(read_content, test_content.to_string());
                println!("read_file_from_path_of_read_to. content={}", read_content);
            }
        }
    }
}
