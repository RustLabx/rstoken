use crate::chain::ChainAdapter;
use crate::types::*;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// Unified runtime for managing all chain adapters
/// Provides a single entry point for multi-chain operations
pub struct ChainRuntime {
    adapters: HashMap<ChainId, Arc<dyn ChainAdapter>>,
}

impl ChainRuntime {
    /// Create a new chain runtime
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    /// Register a chain adapter
    pub fn register_adapter(&mut self, adapter: Arc<dyn ChainAdapter>) {
        let chain_id = adapter.chain_id();
        self.adapters.insert(chain_id, adapter);
    }

    /// Get adapter for a specific chain
    pub fn get_adapter(&self, chain_id: ChainId) -> Result<Arc<dyn ChainAdapter>> {
        self.adapters
            .get(&chain_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Chain adapter not found for: {}", chain_id))
    }

    /// Get balance for an address
    pub async fn get_balance(&self, address: &Address) -> Result<Balance> {
        let adapter = self.get_adapter(address.chain_id)?;
        adapter.get_balance(address).await
    }

    /// Build a transaction
    pub async fn build_tx(&self, req: TxRequest) -> Result<UnsignedTx> {
        let chain_id = req.from.chain_id;
        let adapter = self.get_adapter(chain_id)?;
        adapter.build_tx(req).await
    }

    /// Sign a transaction
    pub async fn sign_tx(&self, tx: UnsignedTx, key: &dyn crate::types::Key) -> Result<SignedTx> {
        let adapter = self.get_adapter(tx.chain_id)?;
        adapter.sign_tx(tx, key).await
    }

    /// Send a transaction
    pub async fn send_tx(&self, tx: SignedTx) -> Result<TxHash> {
        let adapter = self.get_adapter(tx.chain_id)?;
        adapter.send_tx(tx).await
    }

    /// Watch transaction status
    pub async fn watch_tx(&self, hash: TxHash) -> Result<TxStatusInfo> {
        let adapter = self.get_adapter(hash.chain_id)?;
        adapter.watch_tx(hash).await
    }

    /// Complete transaction flow: build -> sign -> send
    pub async fn send_transaction(
        &self,
        req: TxRequest,
        key: &dyn crate::types::Key,
    ) -> Result<TxHash> {
        let unsigned_tx = self.build_tx(req).await?;
        let signed_tx = self.sign_tx(unsigned_tx, key).await?;
        self.send_tx(signed_tx).await
    }

    /// Get list of supported chains
    pub fn supported_chains(&self) -> Vec<ChainId> {
        self.adapters.keys().copied().collect()
    }
}

impl Default for ChainRuntime {
    fn default() -> Self {
        Self::new()
    }
}

