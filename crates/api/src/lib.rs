pub mod controllers;
pub mod middleware;
pub mod response;
pub mod routes;
pub mod state;

use axum::Router;
use middleware::{cors_layer, logging_layer};
use sqlx::postgres::Postgres;
use sqlx::Pool;
use state::AppState;

pub fn create_app(db: Pool<Postgres>) -> Router {
    let app_state = AppState::new(db);
    Router::new()
        .merge(routes::api_router())
        .layer(cors_layer())
        .layer(logging_layer())
        .with_state(app_state)
}

pub async fn start_server(port: u16, db: Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app(db);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await?;

    tracing::info!("Server listening on port {}", port);
    
    axum::serve(listener, app)
        .await?;
    
    Ok(())
}