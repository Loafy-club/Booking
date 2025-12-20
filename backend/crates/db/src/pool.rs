use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub use sqlx::PgPool as Pool;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await
}
