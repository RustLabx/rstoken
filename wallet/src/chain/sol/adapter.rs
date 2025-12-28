use crate::chain::ChainAdapter;
use crate::types::*;
use anyhow::Result;
use async_trait::async_trait;

/// Solana chain adapter (placeholder implementation)
pub struct SolanaAdapter {
    #[allow(dead_code)]
    rpc_url: String,
}

impl SolanaAdapter {
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }
}

#[async_trait]
impl ChainAdapter for SolanaAdapter {
    fn chain_id(&self) -> ChainId {
        ChainId::Solana
    }

    async fn get_balance(&self, _address: &Address) -> Result<Balance> {
        Err(anyhow::anyhow!(
            "Solana adapter not yet implemented. Add solana-sdk dependency."
        ))
    }

    async fn build_tx(&self, _req: TxRequest) -> Result<UnsignedTx> {
        Err(anyhow::anyhow!(
            "Solana adapter not yet implemented. Add solana-sdk dependency."
        ))
    }

    async fn sign_tx(&self, _tx: UnsignedTx, _key: &dyn Key) -> Result<SignedTx> {
        Err(anyhow::anyhow!(
            "Solana adapter not yet implemented. Add solana-sdk dependency."
        ))
    }

    async fn send_tx(&self, _tx: SignedTx) -> Result<TxHash> {
        Err(anyhow::anyhow!(
            "Solana adapter not yet implemented. Add solana-sdk dependency."
        ))
    }

    async fn watch_tx(&self, _hash: TxHash) -> Result<TxStatusInfo> {
        Err(anyhow::anyhow!(
            "Solana adapter not yet implemented. Add solana-sdk dependency."
        ))
    }
}

