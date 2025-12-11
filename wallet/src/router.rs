use crate::handler::block_handler::BlockHandler;
use crate::handler::healthy_handler::healthy;
use crate::handler::wallet_handler::WalletHandler;
use crate::model::app_model::AppState;
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;
use crate::handler::erc20_handler::ERC20Handler;

pub fn create_route(app_state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/health", post(healthy))
        .route("/block/height", get(BlockHandler::get_block_height))
        .route("/block/latest", get(BlockHandler::get_latest_block))
        .route("/wallet/import", post(WalletHandler::import_private_key))
        .route("/wallet/balance/{address}", get(WalletHandler::get_balance))
        .route("/wallet/transaction/{tx_hash}", get(WalletHandler::get_transaction))
        .route("/wallet/send", post(WalletHandler::send_transaction))
        .route("/erc20/balance", get(ERC20Handler::get_balance))
        .with_state(app_state.clone());
    router
}
