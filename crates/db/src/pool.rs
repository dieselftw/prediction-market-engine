use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::sync::OnceLock;

static DB_POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

pub fn set_pool(pool: Pool<Postgres>) -> Result<(), Pool<Postgres>> {
    DB_POOL.set(pool)
}

pub fn get_pool() -> &'static Pool<Postgres> {
    DB_POOL.get().expect("Database pool not initialized. Call db::init::init_db() first.")
}

