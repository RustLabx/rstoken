use crate::config::server_config::Config;
use crate::model::keyring::Keyring;
use ethers::providers::{Http, Provider};
use sqlx::{MySql, Pool};
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Pool<MySql>,
    pub env: Config,
    pub eth: Provider<Http>,
    pub mem: MemoryStorage,
}

pub struct MemoryStorage {
    pub keyring: RwLock<Keyring>,
}
