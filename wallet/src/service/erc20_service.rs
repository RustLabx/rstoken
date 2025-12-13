use crate::model::keyring::Keyring;
use anyhow::Result;
use ethers::contract::abigen;
use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;
use ethers::types::{Address, H256};
use ethers::utils::{format_units, parse_units};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

abigen!(
    ERC20,
    r#"[
        function name() view returns (string)
        function symbol() view returns (string)
        function decimals() view returns (uint8)
        function totalSupply() view returns (uint256)
        function balanceOf(address) view returns (uint256)
        function transfer(address to, uint amount) returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
    ]"#,
);

#[derive(Serialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: String,
}

pub struct ERC20Service<'a> {
    eth_provider: &'a Provider<Http>,
    keyring: &'a RwLock<Keyring>,
}

impl<'a> ERC20Service<'a> {
    pub fn new(eth: &'a Provider<Http>, ring: &'a RwLock<Keyring>) -> Result<Self> {
        Ok(Self {
            eth_provider: eth,
            keyring: ring,
        })
    }

    pub async fn get_balance(&self, address: &str, contract_address: &str) -> Result<String> {
        let address = address.parse::<Address>()?;
        let contract_address = contract_address.parse::<Address>()?;

        let contract = ERC20::new(contract_address, Arc::new(self.eth_provider.clone()));

        let balance = contract.balance_of(address).call().await?;
        let decimals = contract.decimals().call().await?;
        let balance_formatted = format_units(balance, decimals as u32)?;

        Ok(balance_formatted)
    }

    pub async fn send_transaction(&self, from: &str, to: &str, amount: &str, contract_address: &str) -> Result<H256> {
        let from_addr = from.parse::<Address>()?;
        let to_addr = to.parse::<Address>()?;
        let contract_addr = contract_address.parse::<Address>()?;
        let key_entry = self.keyring.read().await.get_by_address(from_addr)?;

        let chain_id = self.eth_provider.get_chainid().await?.as_u64();
        let signer = key_entry.clone().with_chain_id(chain_id);
        let client = Arc::new(SignerMiddleware::new(self.eth_provider.clone(), signer));

        let contract = ERC20::new(contract_addr, client);

        let decimals = contract.decimals().call().await?;
        let amount_formatted = parse_units(amount, decimals as u32)?.into();

        let call = contract.transfer(to_addr, amount_formatted);
        let pending_tx = call.send().await?;
        Ok(pending_tx.tx_hash())
    }

    pub async fn get_info(&self, eoa_address: &str, contract_address: &str) -> Result<TokenInfo> {
        let eoa_addr = eoa_address.parse::<Address>()?;
        let contract_addr = contract_address.parse::<Address>()?;
        let key_entry = self.keyring.read().await.get_by_address(eoa_addr)?;

        let chain_id = self.eth_provider.get_chainid().await?.as_u64();
        let signer = key_entry.clone().with_chain_id(chain_id);
        let client = Arc::new(SignerMiddleware::new(self.eth_provider.clone(), signer));

        let contract = ERC20::new(contract_addr, client);

        let name = contract.name().call().await?;
        let symbol = contract.symbol().call().await?;
        let decimals = contract.decimals().call().await?;
        let total_supply = contract.total_supply().call().await?;
        let total_supply_formatted = format_units(total_supply, decimals as u32)?;

        Ok(TokenInfo {
            name,
            symbol,
            decimals,
            total_supply: total_supply_formatted,
        })
    }
}
