use crate::error::AppError;
use crate::model::app_model::AppState;
use crate::service::wallet_service::WalletService;
use axum::extract::State;
use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

pub struct WalletHandler;

#[derive(Deserialize)]
pub struct ImportPriKeyRequest {
    pub private_key: String,
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
                "address": WalletService::new(&app_state.mem.keyring)?
                    .import_private_key(&import_key_req.private_key).await?
            }
        });
        Ok(Json(response))
    }
}
