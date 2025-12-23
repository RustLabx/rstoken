use crate::config::server_config::Config;
use crate::model::keyring::Keyring;
use ethers::providers::{Http, Provider, Ws};
use ethers::types::Address;
use sqlx::{MySql, Pool};
use std::collections::HashSet;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Pool<MySql>,
    pub env: Config,
    pub eth: Provider<Http>,
    pub eth_ws: Option<Provider<Ws>>,
    pub mem: MemoryStorage,
}

pub struct MemoryStorage {
    pub keyring: RwLock<Keyring>,
    pub listening: RwLock<HashSet<Address>>,
}
