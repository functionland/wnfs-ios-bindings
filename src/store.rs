use std::any::Any;
use anyhow::Result;
use anyhow::Ok;
use wnfsutils::blockstore::FFIStore;

pub struct BridgedStore<'a> {

}

impl<'a> BridgedStore<'a> {
    fn new(fula_client: dyn Any) -> Self {
    }
}

impl<'a> FFIStore<'a> for BridgedStore<'a> {
    /// Retrieves an array of bytes from the block store with given CID.
    fn get_block(&self, cid: Vec<u8>) -> Result<Vec<u8>> {
        Ok(vec![])
    }

    /// Stores an array of bytes in the block store.
    fn put_block(&self, bytes: Vec<u8>, codec: i64) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
