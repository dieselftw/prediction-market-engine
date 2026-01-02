use crate::response::ApiResponse;
use crate::state::AppState;
use axum::{extract::State, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

pub async fn health_check(
    State(_app_state): State<AppState>,
) -> Json<ApiResponse<HealthResponse>> {
    tracing::debug!("Health check endpoint called");
    
    let response = HealthResponse {
        status: "OK".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    tracing::info!("Health check completed successfully");
    Json(ApiResponse::success(response))
}

