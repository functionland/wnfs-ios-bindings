use std::marker::PhantomData;

use anyhow::Ok;
use anyhow::Result;
use wnfsutils::blockstore::FFIStore;
use wnfsutils::kvstore::KVBlockStore;

pub struct BridgedStore<'a> {
    dummy: PhantomData<&'a u8>,
}

impl<'a> BridgedStore<'a> {
    pub fn new(db_path: &str) -> KVBlockStore {
        return KVBlockStore::new(db_path.into(), libipld::IpldCodec::DagCbor);
    }
}

// impl<'a> FFIStore<'a> for BridgedStore<'a> {
//     /// Retrieves an array of bytes from the block store with given CID.
//     fn get_block(&self, _cid: Vec<u8>) -> Result<Vec<u8>> {
//         Ok(vec![])
//     }

//     /// Stores an array of bytes in the block store.
//     fn put_block(&self, _bytes: Vec<u8>, _codec: i64) -> Result<Vec<u8>> {
//         Ok(vec![])
//     }
// }
