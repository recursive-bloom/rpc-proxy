

//! rpc interface.
use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_derive::rpc;
use ethereum_types::{H64, H160, H256, U64, U256};

/// bloom rpc interface.
#[rpc(server)]
pub trait Eth {
    /// RPC Metadata
    type Metadata;

    /// Returns accounts list.
    #[rpc(name = "eth_accounts")]
    fn accounts(&self) -> Result<Vec<H160>>;

    /// Returns balance of the given account.
    #[rpc(name = "eth_getBalance")]
    fn balance(&self, _: H160, _: Option<u64>) -> BoxFuture<U256>;
}