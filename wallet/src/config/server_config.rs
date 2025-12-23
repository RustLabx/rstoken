#[derive(Debug, Clone)]
pub struct Config {
    pub dsn: String,
    pub port: i32,
    pub eth_url: String,
}

impl Config {
    pub fn init() -> Self {
        Self {
            dsn: std::env::var("DSN").expect("Data Source Name must be set"),
            port: std::env::var("PORT").expect("PORT must be set").parse().expect("PORT must be a number"),
            eth_url: std::env::var("ETH_URL").expect("ETH_URL must be set"),
        }
    }

    /// Convert HTTP URL to WebSocket URL
    /// http://localhost:8545 -> ws://localhost:8545
    /// https://mainnet.infura.io/v3/xxx -> wss://mainnet.infura.io/v3/xxx
    pub fn eth_ws_url(&self) -> String {
        self.eth_url
            .replace("http://", "ws://")
            .replace("https://", "wss://")
    }
}
