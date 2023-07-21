use std::cmp::min;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

use anyhow::Ok;
use anyhow::Result;
use log::trace;
use wnfsutils::blockstore::FFIStore;

use crate::blockstore_interface::BlockStoreInterface;
use crate::c_types::RustBytes;

struct LongVec(Vec<u8>);
impl Display for LongVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        for num in &self.0[0..min(self.0.len() - 1, 50)] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", comma_separated)
    }
}

#[derive(Clone)]
pub struct BridgedStore {
    block_store_interface: BlockStoreInterface,
}

impl<'a> BridgedStore {
    pub fn new(block_store_interface: BlockStoreInterface) -> BridgedStore {
        let block_store_interface = block_store_interface;
        return BridgedStore {
            block_store_interface: block_store_interface,
        };
    }
}

impl<'a> FFIStore<'a> for BridgedStore {
    /// Retrieves an array of bytes from the block store with given CID.
    fn get_block(&self, _cid: Vec<u8>) -> Result<Vec<u8>> {
  
            let cid = RustBytes::from(_cid.to_owned());
            let data = self.block_store_interface.to_owned().get(cid);
            if !data.to_owned().ok {
                let err_str: String = data.to_owned().err.into();
                Err(anyhow::format_err!(err_str))
            }else {
                let result: Vec<u8> = data.to_owned().result.into();
                trace!(
                    "get: cid({:?}) -> data({})",
                    _cid,
                    LongVec(result.to_owned())
                );
                self.block_store_interface
                    .to_owned()
                    .dealloc_after_get(data);
                Ok(result.to_owned())
            }
        
    }

    /// Stores an array of bytes in the block store.
    fn put_block(&self, _cid: Vec<u8>, _bytes: Vec<u8>) -> Result<()> {
       
            let cid = RustBytes::from(_cid.to_owned());
            let bytes = RustBytes::from(_bytes.to_owned());
            let data = self.block_store_interface.to_owned().put(cid, bytes);
            if !data.ok {
                let err_str: String = data.err.into();
                Err(anyhow::format_err!(err_str))
            }else {
                trace!(
                    "get: cid({:?}) -> data({})",
                    _cid,
                    LongVec(_bytes.to_owned())
                );
                self.block_store_interface
                    .to_owned()
                    .dealloc_after_put(data);
                Ok(())
            }
        
    }
}

pub unsafe fn c_array_to_vec(ptr: *const u8, size: libc::size_t) -> Vec<u8> {
    std::slice::from_raw_parts(ptr, size).to_vec()
}

#[cfg(test)]
mod tests {
    use hex;
    use libipld::{
        cid::Version,
        multihash::{MultihashDigest, MultihashGeneric},
        Cid, IpldCodec,
    };

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
        let cid_bytes = hex::decode(input).unwrap();

        let cid_recreated = Cid::try_from(cid_bytes).unwrap();
        assert!(cid_recreated.version() == Version::V1);
        assert!(
            cid_recreated.to_string()
                == "bafyreih2ygae7gvhhfg7qq46bcxsrkl3pio4zbgeznsgs7l37ooss2ueya"
        );
    }
}
