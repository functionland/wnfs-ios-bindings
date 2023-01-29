use anyhow::Ok;
use anyhow::Result;
use wnfsutils::blockstore::FFIStore;
use crate::ios::BlockStoreInterface;
use crate::ios::c_array_to_vec;
use crate::ios::cbytes_free;
use crate::ios::vec_to_c_array;

pub struct BridgedStore {
    block_store_interface: BlockStoreInterface
}

impl<'a> BridgedStore {
    pub fn new(block_store_interface: BlockStoreInterface) -> BridgedStore {
        let block_store_interface = block_store_interface;
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
            let result = {
                let cid_size: *const libc::size_t = &mut len;
                let result_size: *mut libc::size_t = &mut size;
                let result = self.block_store_interface.to_owned().get(cid, cid_size, result_size);
                result
            };
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
            let result = {
                let bytes_size: *const libc::size_t = &mut len;
                let result_size: *mut libc::size_t = &mut size;
                let result =  self.block_store_interface.to_owned().put(bytes, bytes_size, result_size, _codec);
                result
            };
            cbytes_free(bytes, len as i32, capacity as i32);            
            let out = c_array_to_vec(size, result);
            Ok(out)
        }
    }
}
