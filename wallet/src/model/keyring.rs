use anyhow::{anyhow, Result};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;
use std::collections::HashMap;

pub struct Keyring {
    data_mapping: HashMap<Address, LocalWallet>,
}

impl Keyring {
    pub fn new() -> Self {
        Self { data_mapping: HashMap::new() }
    }

    pub fn add_from_private_key(&mut self, private_key: &str) -> Result<Address> {
        let wallet: LocalWallet = private_key.parse()?;
        let addr = wallet.address();
        self.data_mapping.insert(addr, wallet);
        Ok(addr)
    }

    pub fn get_by_address(&self, address: Address) -> Result<LocalWallet> {
        let wallet = self.data_mapping
            .get(&address)
            .ok_or_else(|| anyhow!("key not found"))?;
        Ok(wallet.clone())
    }
}
