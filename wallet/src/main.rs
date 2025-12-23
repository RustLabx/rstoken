use anyhow::Result;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::Method;
use ethers::providers::{Http, Provider, Ws};
use sqlx::mysql::MySqlPoolOptions;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use wallet::model::app_model::MemoryStorage;
use wallet::model::keyring::Keyring;
use wallet::{config::server_config::Config, model::app_model::AppState, router::create_route};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config = Config::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.dsn)
        .await?;

    let eth_provider = Provider::<Http>::try_from(&config.eth_url)?;

    // Try to initialize WebSocket Provider (for event listening)
    // If connection fails, will use None and fallback to HTTP Provider
    let eth_ws_provider = match Provider::<Ws>::connect(&config.eth_ws_url()).await {
        Ok(provider) => {
            println!("✅ WebSocket Provider connected successfully: {}", config.eth_ws_url());
            Some(provider)
        }
        Err(e) => {
            eprintln!("⚠️  WebSocket Provider connection failed: {}, will use HTTP Provider for event listening: {}", config.eth_ws_url(), e);
            None
        }
    };

    let mem_store = MemoryStorage {
        keyring: RwLock::new(Keyring::new()),
        listening: RwLock::new(HashSet::new()),
    };

    let app_state = Arc::new(AppState {
        db: pool,
        env: config,
        eth: eth_provider,
        eth_ws: eth_ws_provider,
        mem: mem_store,
    });

    run(app_state).await?;

    Ok(())
}

async fn run(app_state: Arc<AppState>) -> Result<()> {
    // CORS configuration
    let orgins = [
        "http://localhost:3000".parse()?,
        // Add other allowed origins here
    ];
    let cors = CorsLayer::new()
        .allow_origin(orgins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, CONTENT_DISPOSITION]);

    let port = app_state.env.port;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    let route = create_route(app_state).layer(cors);

    println!("✅ server start at http://{}", addr);

    axum::serve(listener, route).await?;

    Ok(())
}
