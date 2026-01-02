#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Initialize tracing first (before other operations)
    // RUST_LOG env var controls log levels (e.g., RUST_LOG=info, RUST_LOG=debug)
    // Defaults to "info" if not set
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    println!("Tracing initialized");
    // Initialize Database
    let db = db::init::init_db().await?;
    println!("Connected to database");

    // Start the API server
    api::start_server(3000, db).await?;
    println!("API server started");

    Ok(())
}
