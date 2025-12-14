use crate::handler::block_handler::BlockHandler;
use crate::handler::erc20_handler::ERC20Handler;
use crate::handler::ether_handler::EtherHandler;
use crate::handler::healthy_handler::healthy;
use crate::handler::wallet_handler::WalletHandler;
use crate::model::app_model::AppState;
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;

pub fn create_route(app_state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/health", post(healthy))
        .route("/block/height", get(BlockHandler::get_block_height))
        .route("/block/latest", get(BlockHandler::get_latest_block))
        .route("/wallet/import", post(WalletHandler::import_private_key))
        .route("/wallet/balance/{address}", get(EtherHandler::get_balance))
        .route("/wallet/transaction/{tx_hash}", get(EtherHandler::get_transaction))
        .route("/wallet/send", post(EtherHandler::send_transaction))
        .route("/erc20/balance", get(ERC20Handler::get_balance))
        .route("/erc20/send", post(ERC20Handler::send_transaction))
        .route("/erc20/info/{contract_address}", get(ERC20Handler::get_info))
        .route("/erc20/listen/{contract_address}", get(ERC20Handler::listen))
        .with_state(app_state.clone());
    router
}
