use crate::error::AppError;
use crate::model::app_model::AppState;
use crate::service::erc20_service::ERC20Service;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

pub struct ERC20Handler;

#[derive(Deserialize)]
pub struct ERC20BalanceRequest {
    pub address: String,
    pub contract_address: String,
}

#[derive(Deserialize)]
pub struct SendTxRequest {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub contract: String,
}

impl ERC20Handler {
    pub async fn get_balance(
        State(app_state): State<Arc<AppState>>,
        Query(req): Query<ERC20BalanceRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status": 200,
            "message": "success",
            "data": {
                "balance": ERC20Service::new(&app_state.eth, app_state.eth_ws.as_ref(), &app_state.mem.keyring, &app_state.mem.listening)?
                    .get_balance(&req.address, &req.contract_address).await?
            }
        });
        Ok(Json(response))
    }

    pub async fn send_transaction(
        State(app_state): State<Arc<AppState>>,
        Json(send_tx_req): Json<SendTxRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status": 200,
            "message": "success",
            "data": {
                "transaction_hash": ERC20Service::new(&app_state.eth, app_state.eth_ws.as_ref(), &app_state.mem.keyring, &app_state.mem.listening)?
                    .send_transaction(&send_tx_req.from, &send_tx_req.to, &send_tx_req.amount, &send_tx_req.contract).await?
            }
        });
        Ok(Json(response))
    }

    pub async fn get_info(
        State(app_state): State<Arc<AppState>>,
        Path(contract_address): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status": 200,
            "message": "success",
            "data": ERC20Service::new(&app_state.eth, app_state.eth_ws.as_ref(), &app_state.mem.keyring, &app_state.mem.listening)?
                .get_info(&contract_address).await?
        });
        Ok(Json(response))
    }

    pub async fn listen(
        State(app_state): State<Arc<AppState>>,
        Path(contract_address): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status": 200,
            "message": "success",
            "data": {
                "status": ERC20Service::new(&app_state.eth, app_state.eth_ws.as_ref(), &app_state.mem.keyring, &app_state.mem.listening)?
                    .listen(&contract_address).await?
            }
        });
        Ok(Json(response))
    }
}
