use crate::error::AppError;
use crate::model::app_model::AppState;
use crate::service::erc20_service::ERC20Service;
use axum::Json;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use serde_json::json;
use std::sync::Arc;
use serde::Deserialize;

pub struct ERC20Handler;

#[derive(Deserialize)]
pub struct ERC20BalanceRequest {
    pub address: String,
    pub contract_address: String,
}

impl ERC20Handler {
    pub async fn get_balance(
        State(app_state): State<Arc<AppState>>,
        Query(req): Query<ERC20BalanceRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status":200,
            "message":"success",
            "data":{
                "balance": ERC20Service::new(&app_state.eth)?
                .get_balance(&req.address, &req.contract_address).await?
            }
        });
        Ok(Json(response))
    }
}
