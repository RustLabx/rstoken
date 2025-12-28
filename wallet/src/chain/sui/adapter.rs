use crate::chain::ChainAdapter;
use crate::types::*;
use anyhow::Result;
use async_trait::async_trait;

/// Sui chain adapter (placeholder implementation)
pub struct SuiAdapter {
    #[allow(dead_code)]
    rpc_url: String,
}

impl SuiAdapter {
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }
}

#[async_trait]
impl ChainAdapter for SuiAdapter {
    fn chain_id(&self) -> ChainId {
        ChainId::Sui
    }

    async fn get_balance(&self, _address: &Address) -> Result<Balance> {
        Err(anyhow::anyhow!(
            "Sui adapter not yet implemented. Add sui-sdk or similar dependency."
        ))
    }

    async fn build_tx(&self, _req: TxRequest) -> Result<UnsignedTx> {
        Err(anyhow::anyhow!(
            "Sui adapter not yet implemented. Add sui-sdk or similar dependency."
        ))
    }

    async fn sign_tx(&self, _tx: UnsignedTx, _key: &dyn Key) -> Result<SignedTx> {
        Err(anyhow::anyhow!(
            "Sui adapter not yet implemented. Add sui-sdk or similar dependency."
        ))
    }

    async fn send_tx(&self, _tx: SignedTx) -> Result<TxHash> {
        Err(anyhow::anyhow!(
            "Sui adapter not yet implemented. Add sui-sdk or similar dependency."
        ))
    }

    async fn watch_tx(&self, _hash: TxHash) -> Result<TxStatusInfo> {
        Err(anyhow::anyhow!(
            "Sui adapter not yet implemented. Add sui-sdk or similar dependency."
        ))
    }
}

