use crate::config::server_config::Config;
use anyhow::Result;
use ethers::providers::{Http, Middleware, Provider};

pub struct BlockService {
    eth_provider: Provider<Http>,
}

impl BlockService {
    pub fn new(config: Config) -> Self {
        Self {
            eth_provider: Provider::try_from(&config.eth_url).unwrap(),
        }
    }

    pub async fn get_block_height(&self) -> Result<u64> {
        let block_height = self.eth_provider.get_block_number().await?;
        Ok(block_height.as_u64())
    }
}
