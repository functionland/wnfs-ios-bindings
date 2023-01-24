use anyhow::Ok;
use anyhow::Result;
use wnfsutils::blockstore::FFIStore;
use crate::ios::BlockStoreInterface;
use crate::ios::c_array_to_vec;
use crate::ios::cbytes_free;
use crate::ios::vec_to_c_array;

pub struct BridgedStore {
    block_store_interface: *const BlockStoreInterface
}

impl<'a> BridgedStore {
    pub fn new(block_store_interface: *const BlockStoreInterface) -> BridgedStore {
        return BridgedStore{
            block_store_interface: block_store_interface,
        }
    }
}

impl<'a> FFIStore<'a> for BridgedStore {
    /// Retrieves an array of bytes from the block store with given CID.
    fn get_block(&self, _cid: Vec<u8>) -> Result<Vec<u8>> {
        unsafe{
            let mut len: usize = 0;
            let mut capacity: usize = 0;
            let mut size: usize = 0;
            let cid = vec_to_c_array(_cid.to_owned().as_mut(), &mut len, &mut capacity);
            let get_fn = (*self.block_store_interface).get_fn;
            let result = get_fn(cid, &mut len, &mut size);
            cbytes_free(cid, len as i32, capacity as i32);
            
            let out = c_array_to_vec(size, result);
            Ok(out)
        }
    }

    /// Stores an array of bytes in the block store.
    fn put_block(&self, _bytes: Vec<u8>, _codec: i64) -> Result<Vec<u8>> {
        unsafe{
            let mut len: usize = 0;
            let mut capacity: usize = 0;
            let mut size: usize = 0;
            let bytes = vec_to_c_array(_bytes.to_owned().as_mut(), &mut len, &mut capacity);
            let put_fn = (*self.block_store_interface).put_fn;
            let result = put_fn(bytes, &mut len, &mut size, _codec);
            cbytes_free(bytes, len as i32, capacity as i32);            
            let out = c_array_to_vec(size, result);
            Ok(out)
        }
    }
}
