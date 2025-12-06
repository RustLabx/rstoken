use anyhow::Result;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::Method;
use ethers::providers::{Http, Provider};
use sqlx::mysql::MySqlPoolOptions;
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

    let mem_store = MemoryStorage {
        keyring: RwLock::new(Keyring::new()?),
    };

    let app_state = Arc::new(AppState {
        db: pool,
        env: config,
        eth: eth_provider,
        mem: mem_store,
    });

    run(app_state).await?;

    Ok(())
}

async fn run(app_state: Arc<AppState>) -> Result<()> {
    // 跨域
    let orgins = [
        "http://localhost:3000".parse()?,
        // 添加其他允许的域名
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
