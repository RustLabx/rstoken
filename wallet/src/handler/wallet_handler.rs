use crate::error::AppError;
use crate::model::app_model::AppState;
use crate::service::wallet_service::WalletService;
use axum::extract::{Path, State};
use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

pub struct WalletHandler;

#[derive(Deserialize)]
pub struct ImportPriKeyRequest {
    pub private_key: String,
}

#[derive(Deserialize)]
pub struct SendTxRequest {
    pub from: String,
    pub to: String,
    pub amount: String,
}

impl WalletHandler {
    pub async fn import_private_key(
        State(app_state): State<Arc<AppState>>,
        Json(import_key_req): Json<ImportPriKeyRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status":200,
            "message":"success",
            "data":{
                "address": WalletService::new(&app_state.eth, &app_state.mem.keyring)?
                    .import_private_key(&import_key_req.private_key).await?
            }
        });
        Ok(Json(response))
    }

    pub async fn get_balance(
        State(app_state): State<Arc<AppState>>,
        Path(address): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status":200,
            "message":"success",
            "data":{
                "balance": WalletService::new(&app_state.eth, &app_state.mem.keyring)?
                    .get_balance(&address).await?
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
                "transaction": WalletService::new(&app_state.eth, &app_state.mem.keyring)?
                    .get_transaction(&tx_hash).await?
            }
        });
        Ok(Json(response))
    }

    pub async fn send_transaction(
        State(app_state): State<Arc<AppState>>,
        Json(send_tx_req): Json<SendTxRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status":200,
            "message":"success",
            "data":{
                "transaction_hash": WalletService::new(&app_state.eth, &app_state.mem.keyring)?
                    .send_transaction(&send_tx_req.from, &send_tx_req.to, &send_tx_req.amount).await?
            }
        });
        Ok(Json(response))
    }
}
