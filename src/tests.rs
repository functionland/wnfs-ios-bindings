#[cfg(test)]
mod ios_tests {
    use crate::{
        blockstore_interface::{BlockStoreInterface, SwiftData},
        c_types::{
            deserialize_string, ffi_input_array_to_vec, serialize_string, vec_to_c_array,
            RustResult,
        },
        ios::*,
    };
    use libc::c_void;
    use once_cell::sync::Lazy;
    use sha256::digest;
    use std::{
        ffi::{CStr, CString},
        fs,
        os::raw::c_char,
        ptr,
    };
    use wnfs::common::CODEC_DAG_CBOR;
    use wnfsutils::{blockstore::FFIStore, kvstore::KVBlockStore};

    unsafe fn test_cfg(cfg: *mut RustResult<c_char>) -> *const c_char {
        assert!(!cfg.is_null(), "config should not be null");
        assert!(!(*cfg).result.is_null(), "cid should not be null");
        let cid: String = CStr::from_ptr((*cfg).result).to_str().unwrap().into();
        println!("cid: {:?}", cid);
        serialize_string(cid)
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
            let content = String::from_utf8(ffi_input_array_to_vec(
                len as libc::size_t,
                content_from_path,
            ))
            .unwrap();
            assert_eq!(content, test_content.to_owned().to_string());
        }
    }

    static STORE: Lazy<KVBlockStore> =
        Lazy::new(|| KVBlockStore::new(String::from("./tmp/test_db"), CODEC_DAG_CBOR));

    extern "C" fn get(
        _userdata: *mut c_void,
        _cid: *const u8,
        cid_len: *const libc::size_t,
    ) -> *const SwiftData {
        let mut capacity: usize = 0;
        let mut len: usize = 0;
        unsafe {
            let cid = ffi_input_array_to_vec(*cid_len, _cid);
            let data = &mut STORE.get_block(cid).unwrap();
            let tmp1 = vec_to_c_array(data, &mut len, &mut capacity);
            let result = Box::into_raw(Box::new(SwiftData {
                err: serialize_string(String::new()),
                result_ptr: tmp1,
                result_count: len,
            }));
            std::mem::forget(&result);
            result
        }
    }

    extern "C" fn put(
        _userdata: *mut c_void,
        _cid: *const u8,
        cid_len: *const libc::size_t,
        _bytes: *const u8,
        bytes_len: *const libc::size_t,
    ) -> *const SwiftData {
        let mut capacity: usize = 0;
        let mut len: usize = 0;
        unsafe {
            let bytes = ffi_input_array_to_vec(*bytes_len, _bytes);
            let cid = ffi_input_array_to_vec(*cid_len, _cid);
            STORE.put_block(cid, bytes).unwrap();
            let tmp1 = vec_to_c_array(&mut Vec::new(), &mut len, &mut capacity);
            let result = Box::into_raw(Box::new(SwiftData {
                err: serialize_string(String::new()),
                result_ptr: tmp1,
                result_count: len,
            }));
            std::mem::forget(&result);
            result
        }
    }

    extern "C" fn dealloc(_: *const SwiftData) {}

    fn get_block_store_interface() -> BlockStoreInterface {
        let userdata: *mut c_void = ptr::null_mut();
        let result = BlockStoreInterface {
            userdata: userdata,
            put_fn: put,
            get_fn: get,
            dealloc: dealloc,
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

            let mut cfg = init_native(
                get_block_store_interface(),
                wnfs_key_string.to_owned().as_bytes().len() as libc::size_t,
                wnfs_key as *const u8,
            );
            let mut cid = test_cfg(cfg);

            /*
            let mut len: usize = 0;
            let mut capacity: usize = 0;
            let filenames_initial = ls_native(
                get_block_store_interface()
                ,serialize_string("bafyreieqp253whdfdrky7hxpqezfwbkjhjdbxcq4mcbp6bqf4jdbncbx4y".into())
                ,serialize_string("{\"saturated_name_hash\":[229,31,96,28,24,238,207,22,36,150,191,37,235,68,191,144,219,250,5,97,85,208,156,134,137,74,25,209,6,66,250,127],\"content_key\":[172,199,245,151,207,21,26,76,52,109,93,57,118,232,9,230,149,46,37,137,174,42,119,29,102,175,25,149,213,204,45,15],\"revision_key\":[17,5,78,59,8,135,144,240,41,248,135,168,222,186,158,240,100,10,129,4,180,55,126,115,146,239,22,177,207,118,169,51]}".into())
                ,serialize_string("root/".into()),
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
                    serialize_string("root/testfrompath.txt".into()),
                    serialize_string("./tmp/test.txt".into()),
                );
                cid = test_cfg(cfg);

                let mut len: usize = 0;
                let mut capacity: usize = 0;
                let content_from_path = read_file_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/testfrompath.txt".into()),
                    &mut len,
                    &mut capacity,
                );

                let content = String::from_utf8(ffi_input_array_to_vec(
                    len as libc::size_t,
                    Box::from_raw(content_from_path).result,
                ))
                .unwrap();
                assert_eq!(content, test_content.to_owned().to_string());
                println!("read_file_from_path. content={}", content);
            }
            // Read content from path to path
            {
                let content_from_path_topath = read_file_to_path_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/testfrompath.txt".into()),
                    serialize_string("./tmp/test2.txt".into()),
                );
                let content_str =
                    deserialize_string(Box::from_raw(content_from_path_topath).result);
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
                    serialize_string("root/testfrompath.txt".into()),
                    serialize_string("./tmp/teststream.txt".into()),
                );
                let content_str =
                    deserialize_string(Box::from_raw(content_stream_from_path_topath).result);
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

                cfg = mkdir_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/test1".into()),
                );
                cid = test_cfg(cfg);

                cfg = cp_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/testfrompath.txt".into()),
                    serialize_string("root/testfrompathcp.txt".into()),
                );
                cid = test_cfg(cfg);

                let content_cp = read_file_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/testfrompathcp.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content = String::from_utf8(ffi_input_array_to_vec(
                    len as libc::size_t,
                    Box::from_raw(content_cp).result,
                ))
                .unwrap();
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
                    serialize_string("root/testfrompath.txt".into()),
                    serialize_string("root/testfrompathmv.txt".into()),
                );
                cid = test_cfg(cfg);
                let content_mv = read_file_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/testfrompathmv.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content = String::from_utf8(ffi_input_array_to_vec(
                    len as libc::size_t,
                    Box::from_raw(content_mv).result,
                ))
                .unwrap();
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
                    serialize_string("root/testfrompathmv.txt".into()),
                );
                cid = test_cfg(cfg);
                let content_rm1 = read_file_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/testfrompathmv.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content = String::from_utf8(ffi_input_array_to_vec(
                    len as libc::size_t,
                    Box::from_raw(content_rm1).result,
                ))
                .unwrap();
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
                    serialize_string("root/testfrompathcp.txt".into()),
                );
                cid = test_cfg(cfg);
                let content_rm2 = read_file_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/testfrompathcp.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content = String::from_utf8(ffi_input_array_to_vec(
                    len as libc::size_t,
                    Box::from_raw(content_rm2).result,
                ))
                .unwrap();
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
                    serialize_string("root/test.txt".into()),
                    len,
                    test_content_ptr,
                );
                cid = test_cfg(cfg);

                cfg = mkdir_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/test1".into()),
                );
                cid = test_cfg(cfg);

                let content_ls = ls_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let file_names = String::from_utf8(ffi_input_array_to_vec(
                    len as libc::size_t,
                    Box::from_raw(content_ls).result,
                ))
                .unwrap();
                println!("ls. fileNames={}", file_names);

                let content_test = read_file_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/test.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content = String::from_utf8(ffi_input_array_to_vec(
                    len as libc::size_t,
                    Box::from_raw(content_test).result,
                ))
                .unwrap();
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
                    deserialize_string(cid.cast_mut()),
                    wnfs_key_string
                );
                load_with_wnfs_key_native(
                    get_block_store_interface(),
                    wnfs_key_string.to_owned().as_bytes().len() as libc::size_t,
                    wnfs_key as *const u8,
                    cid,
                );

                let content_reloaded = read_file_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/test.txt".into()),
                    &mut len,
                    &mut capacity,
                );
                println!("len: {}, cap: {}", len, capacity);
                let content = String::from_utf8(ffi_input_array_to_vec(
                    len as libc::size_t,
                    Box::from_raw(content_reloaded).result,
                ))
                .unwrap();
                println!("read. content={}", content);
                assert_eq!(content, test_content.to_string());
            }
            // Read content from path to path (reloaded)
            {
                let content_from_path_topath_reloaded = read_file_to_path_native(
                    get_block_store_interface(),
                    cid,
                    serialize_string("root/test.txt".into()),
                    serialize_string("./tmp/test2.txt".into()),
                );
                let content_str = deserialize_string(
                    Box::from_raw(content_from_path_topath_reloaded).result as *mut _,
                );
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
