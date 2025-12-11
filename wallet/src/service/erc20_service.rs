use anyhow::Result;
use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::utils::format_units;
use std::sync::Arc;

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

pub struct ERC20Service<'a> {
    eth_provider: &'a Provider<Http>,
}

impl<'a> ERC20Service<'a> {
    pub fn new(eth: &'a Provider<Http>) -> Result<Self> {
        Ok(Self { eth_provider: eth })
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
}
