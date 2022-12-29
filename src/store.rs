use std::marker::PhantomData;

use anyhow::Ok;
use anyhow::Result;
use wnfsutils::blockstore::FFIStore;

pub struct BridgedStore<'a> {
    fula_client: PhantomData<&'a u8>,
}

impl<'a> BridgedStore<'a> {
    pub fn new() -> Self {
        Self {
            fula_client: PhantomData,
        }
    }
}

impl<'a> FFIStore<'a> for BridgedStore<'a> {
    /// Retrieves an array of bytes from the block store with given CID.
    fn get_block(&self, _cid: Vec<u8>) -> Result<Vec<u8>> {
        Ok(vec![])
    }

    /// Stores an array of bytes in the block store.
    fn put_block(&self, _bytes: Vec<u8>, _codec: i64) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
