
use crate::rpc::*;

use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_core::futures::future;

use ethereum_types::{H64, H160, H256, U64, U256};


pub struct EthClient {}

impl EthClient {
    pub fn new() -> Self {
        EthClient{}
    }
}

impl Eth for EthClient {
    type Metadata = Metadata;

    fn accounts(&self) -> Result<Vec<H160>> {
        Ok(Vec::new())
    }

    fn balance(&self, address: H160, num: Option<BlockNumber>) -> BoxFuture<U256> {
        let bal = U256::zero();
        Box::new(future::done(Ok(bal)))
    }

    fn send_transaction(&self, tx: TransactionRequest) -> BoxFuture<H256> {
        println!("{}",tx);
        Box::new(future::done(Ok(H256::default())))
    }
}