use anyhow::{Result, anyhow};
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

pub struct WalletService;

impl WalletService {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn create_wallet(&self) -> Result<WalletEntity> {
        Err(anyhow!("not implemented"))
    }
}
