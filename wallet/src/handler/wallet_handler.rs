use crate::error::AppError;
use crate::model::app_model::AppState;
use crate::service::wallet_service::WalletService;
use axum::extract::{Path, State};
use axum::{response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

pub struct WalletHandler;

impl WalletHandler {
    pub async fn get_balance(
        State(app_state): State<Arc<AppState>>,
        Path(address): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status":200,
            "message":"success",
            "data":{
                "balance": WalletService::new(&app_state.eth)?.get_balance(&address).await?
            }
        });
        Ok(Json(response))
    }

    pub async fn get_transaction(
        State(app_state): State<Arc<AppState>>,
        Path(tx_hash): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status":200,
            "message":"success",
            "data":{
                "transaction": WalletService::new(&app_state.eth)?.get_transaction(&tx_hash).await?
            }
        });
        Ok(Json(response))
    }
}
