use anyhow::{Result, anyhow};
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Block, BlockNumber, H256};

pub struct BlockService<'a> {
    eth_provider: &'a Provider<Http>,
}

impl<'a> BlockService<'a> {
    pub fn new(eth: &'a Provider<Http>) -> Result<Self> {
        Ok(Self { eth_provider: eth })
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
