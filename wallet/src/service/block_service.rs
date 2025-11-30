use crate::config::server_config::Config;
use anyhow::{Result, anyhow};
use ethers::prelude::{Block, BlockNumber, H256};
use ethers::providers::{Http, Middleware, Provider};

pub struct BlockService {
    eth_provider: Provider<Http>,
}

impl BlockService {
    pub fn new(config: Config) -> Result<Self> {
        let provider = Provider::<Http>::try_from(&config.eth_url)?;
        Ok(Self {
            eth_provider: provider,
        })
    }

    pub async fn get_block_height(&self) -> Result<u64> {
        let block_height = self.eth_provider.get_block_number().await?;
        Ok(block_height.as_u64())
    }

    pub async fn get_latest_block(&self) -> Result<Block<H256>> {
        let latest_block = self.eth_provider.get_block(BlockNumber::Latest).await?;
        latest_block.ok_or_else(|| anyhow!("latest block not found"))
    }
}
