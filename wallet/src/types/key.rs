use crate::types::{Address, ChainId};
use crate::types::Key;
use anyhow::Result;
use async_trait::async_trait;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address as EthAddress;

/// Ethereum key implementation
pub struct EthereumKey {
    wallet: LocalWallet,
}

impl EthereumKey {
    pub fn from_private_key(private_key: &str) -> Result<Self> {
        let wallet: LocalWallet = private_key.parse()?;
        Ok(Self { wallet })
    }

    pub fn eth_address(&self) -> EthAddress {
        self.wallet.address()
    }
}

#[async_trait]
impl Key for EthereumKey {
    fn address(&self, chain_id: ChainId) -> Result<Address> {
        match chain_id {
            ChainId::Ethereum => {
                let addr = self.wallet.address();
                Ok(Address::new(chain_id, format!("{:?}", addr)))
            }
            _ => Err(anyhow::anyhow!("Unsupported chain for Ethereum key")),
        }
    }

    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        // For Ethereum, we typically sign a hash of the data
        // This is a simplified version - in production you'd want
        // to use proper message signing (EIP-191) or transaction signing
        use ethers::core::utils::keccak256;
        let hash = keccak256(data);
        let signature = self.wallet.sign_message(&hash).await?;
        Ok(signature.to_vec())
    }

    fn supported_chains(&self) -> Vec<ChainId> {
        vec![ChainId::Ethereum]
    }
}

