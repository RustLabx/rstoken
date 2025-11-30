use crate::error::AppError;
use crate::model::app_model::AppState;
use crate::service::block_service::BlockService;
use axum::extract::State;
use axum::{Json, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;

pub struct BlockHandler;

impl BlockHandler {
    pub async fn get_block_height(
        State(app_state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status":200,
            "message":"success",
            "data":{
                "block_height": BlockService::new(app_state.env.clone())?.get_block_height().await?
            }
        });
        Ok(Json(response))
    }

    pub async fn get_latest_block(
        State(app_state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = json!({
            "status":200,
            "message":"success",
            "data":{
                "latest_block": BlockService::new(app_state.env.clone())?.get_latest_block().await?
            }
        });
        Ok(Json(response))
    }
}
