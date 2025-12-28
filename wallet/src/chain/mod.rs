use crate::types::*;
use anyhow::Result;
use async_trait::async_trait;

/// Chain adapter trait - unified interface for all blockchain operations
/// This trait defines the complete transaction lifecycle
#[async_trait]
pub trait ChainAdapter: Send + Sync {
    /// Get the chain identifier
    fn chain_id(&self) -> ChainId;

    /// Get balance for an address
    async fn get_balance(&self, address: &Address) -> Result<Balance>;

    /// Build an unsigned transaction from a request
    async fn build_tx(&self, req: TxRequest) -> Result<UnsignedTx>;

    /// Sign an unsigned transaction with a key
    async fn sign_tx(&self, tx: UnsignedTx, key: &dyn Key) -> Result<SignedTx>;

    /// Send a signed transaction to the network
    async fn send_tx(&self, tx: SignedTx) -> Result<TxHash>;

    /// Watch transaction status by hash
    async fn watch_tx(&self, hash: TxHash) -> Result<TxStatusInfo>;
}

pub mod eth;
pub mod sol;
pub mod btc;
pub mod sui;

