use chrono::{Duration, Utc};
use loafy_db::{models::Booking, queries::sessions, PgPool};
use loafy_types::AppError;
use sqlx::Postgres;
use uuid::Uuid;

use super::utils::generate_booking_code;

/// Create booking with race condition protection
/// CRITICAL: Uses SELECT FOR UPDATE to prevent overselling
pub async fn create_booking_with_lock(
    pool: &PgPool,
    user_id: Uuid,
    session_id: Uuid,
    guest_count: i32,
    payment_method: &str,
) -> Result<Booking, AppError> {
    // Start transaction
    let mut tx = pool.begin().await
        .map_err(|e| AppError::Database(e))?;

    // Lock session row (CRITICAL: prevents concurrent bookings)
    let session = sessions::find_by_id_for_update(&mut tx, session_id)
        .await
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))))?
        .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;

    // Check if session is cancelled
    if session.cancelled {
        tx.rollback().await.ok();
        return Err(AppError::BadRequest("Session is cancelled".to_string()));
    }

    // Check if session is in the past
    let now_date = chrono::Local::now().naive_local().date();
    if session.date < now_date {
        tx.rollback().await.ok();
        return Err(AppError::BadRequest("Session is in the past".to_string()));
    }

    // Calculate required slots (1 for user + guests)
    let slots_needed = 1 + guest_count;

    // Check availability
    if session.available_slots < slots_needed {
        tx.rollback().await.ok();
        return Err(AppError::Conflict(format!(
            "Not enough slots available. Need {}, have {}",
            slots_needed, session.available_slots
        )));
    }

    // Get base price (from session or global default)
    let base_price_vnd = session.price_vnd.unwrap_or(100000);

    // Calculate prices
    // Phase 1: Simple calculation (no subscriptions yet)
    // User pays base price, each guest pays base price
    let user_price_vnd = base_price_vnd;
    let guest_price_vnd = base_price_vnd * guest_count;
    let total_price_vnd = user_price_vnd + guest_price_vnd;

    // Generate unique booking code
    let booking_code = generate_booking_code();

    // Calculate payment deadline (30 minutes from now)
    let payment_deadline = Utc::now() + Duration::minutes(30);

    // Create booking
    let booking = sqlx::query_as::<_, Booking>(
        r#"
        INSERT INTO bookings (
            user_id, session_id, booking_code, guest_count,
            tickets_used, discount_applied,
            price_paid_vnd, guest_price_paid_vnd,
            payment_method, payment_status, payment_deadline
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(session_id)
    .bind(&booking_code)
    .bind(guest_count)
    .bind(0) // Phase 1: no tickets used
    .bind("none") // Phase 1: no discount
    .bind(total_price_vnd)
    .bind(guest_price_vnd)
    .bind(payment_method)
    .bind("pending")
    .bind(payment_deadline.naive_utc())
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| AppError::Database(e))?;

    // Decrement available slots atomically
    sessions::decrement_available_slots(&mut tx, session_id, slots_needed)
        .await
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))))?;

    // Commit transaction
    tx.commit().await
        .map_err(|e| AppError::Database(e))?;

    Ok(booking)
}
