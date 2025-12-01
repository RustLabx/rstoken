use crate::error::AppError;
use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn healthy() -> Result<impl IntoResponse, AppError> {
    let response = json!({
        "status":"success",
        "message":"health is working"
    });

    Ok(Json(response))
}
