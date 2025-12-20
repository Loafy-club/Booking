use tokio::time::{sleep, Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "loafy_jobs=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Loafy Club background jobs...");

    // TODO: Initialize database pool
    // let database_url = std::env::var("DATABASE_URL")?;
    // let pool = loafy_db::create_pool(&database_url).await?;

    // TODO: Initialize job scheduler
    // let scheduler = tokio_cron_scheduler::JobScheduler::new().await?;

    // Keep the process running
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
