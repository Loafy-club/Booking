mod jobs;

use tokio_cron_scheduler::{JobScheduler, Job};
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

    // Initialize database pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = loafy_db::create_pool(&database_url).await?;

    tracing::info!("✓ Database connection established");

    // Initialize job scheduler
    let scheduler = JobScheduler::new().await?;

    // Job 1: Release unpaid bookings (every 1 minute)
    let pool_clone = pool.clone();
    let release_job = Job::new_async("0 * * * * *", move |_uuid, _l| {
        let pool = pool_clone.clone();
        Box::pin(async move {
            tracing::debug!("Running release_unpaid_bookings job");
            if let Err(e) = jobs::release_unpaid_bookings(&pool).await {
                tracing::error!("release_unpaid_bookings job failed: {}", e);
            }
        })
    })?;

    scheduler.add(release_job).await?;

    tracing::info!("✓ Registered job: release_unpaid_bookings (every 1 minute)");

    // TODO: Phase 2 jobs
    // - Process waitlist (every 15 minutes)
    // - Stripe subscription sync (every hour)
    // - Birthday ticket allocation (daily at 00:01)
    // - Screenshot cleanup (daily at 03:00)
    // - Rate limit cleanup (daily at 04:00)
    // - Monthly OCR counter reset (1st of month)
    // - Daily recap emails (hourly, user-configured time)

    // Start scheduler
    scheduler.start().await?;

    tracing::info!("✓ Job scheduler started");
    tracing::info!("📡 Background jobs running");

    // Keep the process running
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
