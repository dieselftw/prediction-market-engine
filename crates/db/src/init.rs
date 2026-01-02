use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = std::env::var("DB_URL").expect("DB_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    Ok(pool)
}