use crate::config::server_config::Config;
use ethers::providers::{Http, Provider};
use sqlx::{MySql, Pool};

pub struct AppState {
    pub db: Pool<MySql>,
    pub env: Config,
    pub eth: Provider<Http>,
}
