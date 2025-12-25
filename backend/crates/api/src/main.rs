mod middleware;
mod response;
mod routes;

use axum::{routing::{get, post, put, delete}, Router};
use loafy_integrations::supabase::SupabaseAuth;
use middleware::AppState;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "loafy_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Loafy Club API server...");

    // Get configuration from environment
    let port = std::env::var("API_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    // Initialize database pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = loafy_db::create_pool(&database_url).await?;

    tracing::info!("✓ Database connection established");

    // Initialize Supabase client
    let supabase_url = std::env::var("SUPABASE_URL")
        .expect("SUPABASE_URL must be set");
    let supabase_anon_key = std::env::var("SUPABASE_ANON_KEY")
        .expect("SUPABASE_ANON_KEY must be set");
    let supabase_service_key = std::env::var("SUPABASE_SERVICE_KEY")
        .expect("SUPABASE_SERVICE_KEY must be set");

    let supabase = SupabaseAuth::new(
        supabase_url,
        supabase_anon_key,
        supabase_service_key,
    );

    tracing::info!("✓ Supabase client initialized");

    // Create app state
    let state = AppState {
        supabase,
        db: pool,
    };

    // Build application router
    let app = Router::new()
        .route("/health", get(health_check))
        // Auth routes
        .route("/api/auth/callback", post(routes::auth::handle_callback))
        .route("/api/auth/me", get(routes::auth::get_current_user))
        .route("/api/auth/logout", post(routes::auth::logout))
        // User routes
        .route("/api/users/me", put(routes::users::update_profile).delete(routes::users::delete_account))
        // Session routes
        .route("/api/sessions", get(routes::sessions::list_sessions))
        .route("/api/sessions/:id", get(routes::sessions::get_session))
        .route("/api/sessions/:id/participants", get(routes::sessions::get_session_participants))
        .route("/api/sessions", post(routes::sessions::create_session))
        .route("/api/sessions/:id", put(routes::sessions::update_session))
        .route("/api/sessions/:id", delete(routes::sessions::delete_session))
        // Booking routes
        .route("/api/bookings", get(routes::bookings::list_my_bookings))
        .route("/api/bookings/:id", get(routes::bookings::get_booking))
        .route("/api/bookings", post(routes::bookings::create_booking))
        .route("/api/bookings/:id", delete(routes::bookings::cancel_booking_route))
        // Payment routes
        .route("/api/payments/stripe/intent", post(routes::payments::create_payment_intent))
        .route("/api/webhooks/stripe", post(routes::payments::stripe_webhook))
        // Admin routes
        .route("/api/admin/stats", get(routes::admin::get_stats))
        .route("/api/admin/users", get(routes::admin::list_users))
        .route("/api/admin/users/:id", put(routes::admin::update_user).delete(routes::admin::delete_user))
        .route("/api/admin/users/:id/role", put(routes::admin::update_user_role))
        .route("/api/admin/users/:id/suspend", post(routes::admin::suspend_user))
        .route("/api/admin/users/:id/unsuspend", post(routes::admin::unsuspend_user))
        .route("/api/admin/bookings", get(routes::admin::list_bookings))
        .route("/api/admin/bookings/:id", get(routes::admin::get_booking).put(routes::admin::update_booking))
        .route("/api/admin/sessions", get(routes::admin::list_sessions))
        .route("/api/admin/roles", get(routes::admin::list_roles))
        // Admin profit routes
        .route("/api/admin/stats/profit", get(routes::admin::get_profit_stats))
        .route("/api/admin/sessions/profit", get(routes::admin::get_sessions_profit))
        .route("/api/admin/expenses/by-category", get(routes::admin::get_expenses_by_category))
        .route("/api/admin/profit/daily", get(routes::admin::get_daily_profit_data))
        .layer(
            CorsLayer::new()
                .allow_origin(frontend_url.parse::<axum::http::HeaderValue>()?)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("✓ Server listening on {}", addr);
    tracing::info!("📡 API ready at http://{}:{}/api", addr.ip(), port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
