use crate::chain::ChainAdapter;
use crate::types::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ethers::middleware::Middleware;
use ethers::providers::{Http, Provider, Ws};
use ethers::types::{Address as EthAddress, H256, TransactionRequest, U256};
use ethers::utils::{format_ether, parse_ether};

/// Ethereum chain adapter implementation
pub struct EthereumAdapter {
    http_provider: Provider<Http>,
    ws_provider: Option<Provider<Ws>>,
}

impl EthereumAdapter {
    pub fn new(http_provider: Provider<Http>, ws_provider: Option<Provider<Ws>>) -> Self {
        Self {
            http_provider,
            ws_provider,
        }
    }

    pub fn http_provider(&self) -> &Provider<Http> {
        &self.http_provider
    }

    pub fn ws_provider(&self) -> Option<&Provider<Ws>> {
        self.ws_provider.as_ref()
    }
}

#[async_trait]
impl ChainAdapter for EthereumAdapter {
    fn chain_id(&self) -> ChainId {
        ChainId::Ethereum
    }

    async fn get_balance(&self, address: &Address) -> Result<Balance> {
        if address.chain_id != ChainId::Ethereum {
            return Err(anyhow!("Address chain mismatch"));
        }

        let eth_addr: EthAddress = address.value.parse()?;
        let balance = self.http_provider.get_balance(eth_addr, None).await?;
        let balance_str = format_ether(balance);

        Ok(Balance::new(
            balance_str,
            18, // ETH has 18 decimals
            "ETH".to_string(),
        ))
    }

    async fn build_tx(&self, req: TxRequest) -> Result<UnsignedTx> {
        if req.from.chain_id != ChainId::Ethereum || req.to.chain_id != ChainId::Ethereum {
            return Err(anyhow!("Transaction chain mismatch"));
        }

        let from_addr: EthAddress = req.from.value.parse()?;
        let to_addr: EthAddress = req.to.value.parse()?;
        let amount = parse_ether(&req.amount)?;

        let mut tx = TransactionRequest::new()
            .from(from_addr)
            .to(to_addr)
            .value(amount);

        if let Some(gas_limit) = req.gas_limit {
            tx = tx.gas(U256::from(gas_limit));
        }

        if let Some(gas_price) = req.gas_price {
            let gas_price_parsed = parse_ether(&gas_price)?;
            tx = tx.gas_price(gas_price_parsed);
        }

        if let Some(data) = req.data {
            tx = tx.data(data);
        }

        // Serialize transaction to RLP format
        // Note: This is a simplified version. In production, you'd want to use
        // proper transaction encoding based on EIP-1559 or legacy format
        let tx_json = serde_json::to_value(&tx)?;

        Ok(UnsignedTx {
            chain_id: ChainId::Ethereum,
            raw_data: serde_json::to_vec(&tx_json)?,
            tx_type: "ethereum".to_string(),
            metadata: Some(tx_json),
        })
    }

    async fn sign_tx(&self, tx: UnsignedTx, key: &dyn Key) -> Result<SignedTx> {
        if tx.chain_id != ChainId::Ethereum {
            return Err(anyhow!("Transaction chain mismatch"));
        }

        // For Ethereum, we need the private key to sign
        // This is a simplified version - in production, you'd want a more secure key management
        // This assumes the key can provide a signature directly
        let tx_data = &tx.raw_data;
        let signature = key.sign(tx_data).await?;

        // In a real implementation, you'd construct the full signed transaction
        // including the signature in the proper format (r, s, v)
        Ok(SignedTx {
            chain_id: ChainId::Ethereum,
            raw_data: tx.raw_data.clone(),
            signature,
            tx_type: "ethereum".to_string(),
        })
    }

    async fn send_tx(&self, tx: SignedTx) -> Result<TxHash> {
        if tx.chain_id != ChainId::Ethereum {
            return Err(anyhow!("Transaction chain mismatch"));
        }

        // Deserialize the transaction from metadata
        // In a real implementation, you'd reconstruct the transaction with signature
        // and send it via the provider
        // This requires proper transaction encoding with signature (r, s, v)
        // For now, this is a placeholder that needs proper implementation
        Err(anyhow!("Ethereum transaction sending needs proper signed transaction reconstruction. The signed transaction must be properly encoded with the signature before sending."))
    }

    async fn watch_tx(&self, hash: TxHash) -> Result<TxStatusInfo> {
        if hash.chain_id != ChainId::Ethereum {
            return Err(anyhow!("Hash chain mismatch"));
        }

        let tx_hash: H256 = hash.value.parse()?;

        // Get transaction receipt
        let receipt_opt = self
            .http_provider
            .get_transaction_receipt(tx_hash)
            .await?;

        let status = if let Some(receipt) = receipt_opt {
            let block_number = receipt.block_number.map(|n| n.as_u64());
            let confirmations = block_number.map(|_| 1u64); // Simplified

            TxStatusInfo {
                hash: hash.clone(),
                status: TxStatus::Confirmed,
                block_number,
                confirmations,
                error: None,
            }
        } else {
            // Check if transaction exists (might be pending)
            let tx_opt = self.http_provider.get_transaction(tx_hash).await?;
            if tx_opt.is_some() {
                TxStatusInfo {
                    hash: hash.clone(),
                    status: TxStatus::Pending,
                    block_number: None,
                    confirmations: None,
                    error: None,
                }
            } else {
                TxStatusInfo {
                    hash: hash.clone(),
                    status: TxStatus::NotFound,
                    block_number: None,
                    confirmations: None,
                    error: None,
                }
            }
        };

        Ok(status)
    }
}

