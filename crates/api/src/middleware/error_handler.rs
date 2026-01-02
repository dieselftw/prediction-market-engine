use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum::middleware::Next;

pub async fn error_handler(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    if response.status() == StatusCode::NOT_FOUND {
        return (
            StatusCode::NOT_FOUND,
            axum::Json(serde_json::json!({"error": "Not Found"})),
        )
            .into_response();
    }

    response
}

