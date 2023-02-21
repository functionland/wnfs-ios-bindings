use anyhow::Ok;
use anyhow::Result;
use wnfsutils::blockstore::FFIStore;
use crate::ios::BlockStoreInterface;
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
            println!("get(cid): {:?}", _cid);
            let mut len: usize = 0;
            let mut capacity: usize = 0;
            let cid = vec_to_c_array(_cid.to_owned().as_mut(), &mut len, &mut capacity);
            let _data = self.block_store_interface.to_owned().get(cid, &mut len);
            cbytes_free(cid, len as i32, capacity as i32);
            let data = _data.as_ref().unwrap();
            // let data = *_data.to_owned();
            println!("ptr: {:?}, ref: {:?}, obj: {:?}", _data, data.ptr, data);
            let out = c_array_to_vec(data.ptr, data.count).clone();
            println!("111");
            self.block_store_interface.to_owned().dealloc(_data.to_owned());
            Ok(out)
        }
    }

    /// Stores an array of bytes in the block store.
    fn put_block(&self, _bytes: Vec<u8>, _codec: i64) -> Result<Vec<u8>> {
        unsafe{
            let mut len: usize = 0;
            let mut capacity: usize = 0;
            let bytes = vec_to_c_array(_bytes.to_owned().as_mut(), &mut len, &mut capacity);
            let _data = self.block_store_interface.to_owned().put(bytes, &mut len, _codec);
            cbytes_free(bytes, len as i32, capacity as i32);
            let data = _data.as_ref().unwrap();
            // let data = *_data.to_owned();
            println!("count: {:?}, ref: {:?}, obj: {:?}", data.count, data.ptr, data);
            let out = c_array_to_vec(data.ptr as *mut _, data.count).clone();
            println!("111");
            self.block_store_interface.to_owned().dealloc(_data.to_owned());
            Ok(out)
        }
    }
}

pub unsafe fn c_array_to_vec(ptr: *const u8, size: libc::size_t) -> Vec<u8> {
    std::slice::from_raw_parts(ptr, size).to_vec()
}

#[cfg(test)]
mod tests{
    use libipld::{multihash::{MultihashGeneric, MultihashDigest}, IpldCodec, Cid, cid::Version};
    use hex;

    #[test]
    fn test_codec() {
            //let codec_u8_array:[u8;8] = vec_to_array(codec);
        //let codec_u64 = u64::from_be_bytes(codec_u8_array);
        let codec_u64: u64 = u64::try_from(113).unwrap();
        let hash: MultihashGeneric<64> = libipld::multihash::Code::Sha2_256.digest(b"abc");
        let codec = IpldCodec::try_from(codec_u64).unwrap();
        let cid = Cid::new(Version::V1, codec.into(), hash).unwrap();

        assert!(cid.to_string() == "bafyreif2pall7dybz7vecqka3zo24irdwabwdi4wc55jznaq75q7eaavvu");

        let input = "01711220fac1804f9aa7394df8439e08af28a97b7a1dcc84c4cb64697d7bfb9d296a84c0";
        let cid_bytes =  hex::decode(input).unwrap();
    
        let cid_recreated = Cid::try_from(cid_bytes).unwrap();
        assert!(cid_recreated.version() == Version::V1);
        assert!(cid_recreated.to_string() == "bafyreih2ygae7gvhhfg7qq46bcxsrkl3pio4zbgeznsgs7l37ooss2ueya");
    }
}