use crate::error::AppError;
use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn healthy() -> Result<impl IntoResponse, AppError> {
    let response = json!({
        "status":"success",
        "message":"health is working"
    });

    Ok(Json(response))
}
