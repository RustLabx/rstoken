use crate::handler::healthy_handler::healthy;
use crate::model::app_model::AppState;
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;
use crate::handler::block_handler::BlockHandler;

pub fn create_route(app_state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/health", post(healthy))
        .route("/block/height", get(BlockHandler::get_block_height))
        .route("/block/latest", get(BlockHandler::get_latest_block))
        .with_state(app_state.clone());
    router
}
