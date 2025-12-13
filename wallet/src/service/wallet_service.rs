use crate::model::keyring::Keyring;
use anyhow::{anyhow, Result};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

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
    keyring: &'a RwLock<Keyring>,
}

impl<'a> WalletService<'a> {
    pub fn new(ring: &'a RwLock<Keyring>) -> Result<Self> {
        Ok(Self { keyring: ring })
    }

    pub async fn import_private_key(&mut self, private_key: &str) -> Result<Address> {
        self.keyring.write().await.add_from_private_key(private_key)
    }

    pub async fn create_wallet(&self) -> Result<WalletEntity> {
        Err(anyhow!("not implemented"))
    }
}
