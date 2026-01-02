use axum::Router;
use crate::controllers::health::health_check;
use crate::state::AppState;

pub fn health_router() -> Router<AppState> {
    Router::new().route("/health", axum::routing::get(health_check))
}