use anyhow::{anyhow, Result};
use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, Transaction, TransactionRequest, H256};
use ethers::utils::{format_ether, parse_ether};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub struct Keyring {
    key_storage: HashMap<Address, LocalWallet>,
}

impl Keyring {
    pub fn new() -> Result<Self> {
        Ok(Self {
            key_storage: HashMap::new(),
        })
    }

    pub fn add_from_private_key(&mut self, private_key: &str) -> Result<Address> {
        let wallet: LocalWallet = private_key.parse()?;
        let addr = wallet.address();
        self.key_storage.insert(addr, wallet);
        Ok(addr)
    }

    pub fn get_by_address(&self, address: Address) -> Result<LocalWallet> {
        let wallet = self.key_storage
            .get(&address)
            .ok_or_else(|| anyhow!("key not found"))?;
        Ok(wallet.clone())
    }
}

pub struct WalletService<'a> {
    eth_provider: &'a Provider<Http>,
    keyring: Keyring,
}

impl<'a> WalletService<'a> {
    pub fn new(eth: &'a Provider<Http>) -> Result<Self> {
        Ok(Self {
            eth_provider: eth,
            keyring: Keyring::new()?,
        })
    }

    pub async fn import_private_key(&mut self, private_key: &str) -> Result<Address> {
        self.keyring.add_from_private_key(private_key)
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

    pub async fn send_transaction(&self, from: &str, to: &str, amount: &str) -> Result<H256> {
        let from_addr = from.parse::<Address>()?;
        let to_addr = to.parse::<Address>()?;
        let key_entry = self.keyring.get_by_address(to_addr)?;

        let chain_id = self.eth_provider.get_chainid().await?.as_u64();
        let signer = key_entry.clone().with_chain_id(chain_id);
        let client = SignerMiddleware::new(self.eth_provider.clone(), signer);

        let amount = parse_ether(amount)?;
        let tx = TransactionRequest::new()
            .from(from_addr)
            .to(to_addr)
            .value(amount);

        // 发送交易并获取 transaction hash
        let pending_tx = client.send_transaction(tx, None).await?;
        Ok(pending_tx.tx_hash())
    }

    pub async fn create_wallet(&self) -> Result<WalletEntity> {
        Err(anyhow!("not implemented"))
    }
}
