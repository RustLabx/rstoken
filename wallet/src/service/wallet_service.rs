use anyhow::{anyhow, Result};
use ethers::addressbook::Address;
use ethers::middleware::Middleware;
use ethers::providers::{Http, Provider};
use ethers::types::{Transaction, H256};
use ethers::utils::format_ether;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WalletEntity {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnemonic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

pub struct WalletService<'a> {
    eth_provider: &'a Provider<Http>,
}

impl<'a> WalletService<'a> {
    pub fn new(eth: &'a Provider<Http>) -> Result<Self> {
        Ok(Self { eth_provider: eth })
    }

    pub async fn get_balance(&self, address: &str) -> Result<String> {
        let address = address.parse::<Address>()?;
        let balance = self.eth_provider.get_balance(address, None).await?;
        Ok(format_ether(balance))
    }

    pub async fn get_transaction(&self, hash: &str) -> Result<Option<Transaction>> {
        let tx_hash = hash.parse::<H256>()?;
        let transaction = self.eth_provider.get_transaction(tx_hash).await?;
        Ok(transaction)
    }

    pub async fn create_wallet(&self) -> Result<WalletEntity> {
        Err(anyhow!("not implemented"))
    }
}
