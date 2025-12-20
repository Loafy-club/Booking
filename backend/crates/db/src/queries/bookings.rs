use crate::models::Booking;
use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

/// Find booking by ID
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Booking>> {
    let booking = sqlx::query_as::<_, Booking>(
        "SELECT * FROM bookings WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(booking)
}

/// Find booking by code
pub async fn find_by_code(pool: &PgPool, code: &str) -> Result<Option<Booking>> {
    let booking = sqlx::query_as::<_, Booking>(
        "SELECT * FROM bookings WHERE booking_code = $1"
    )
    .bind(code)
    .fetch_optional(pool)
    .await?;

    Ok(booking)
}

/// List user's bookings
pub async fn list_user_bookings(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Booking>> {
    let bookings = sqlx::query_as::<_, Booking>(
        r#"
        SELECT * FROM bookings
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(bookings)
}

/// List bookings for a session
pub async fn list_session_bookings(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Vec<Booking>> {
    let bookings = sqlx::query_as::<_, Booking>(
        r#"
        SELECT * FROM bookings
        WHERE session_id = $1
          AND cancelled_at IS NULL
        ORDER BY created_at ASC
        "#
    )
    .bind(session_id)
    .fetch_all(pool)
    .await?;

    Ok(bookings)
}

/// Cancel booking
pub async fn cancel_booking(pool: &PgPool, id: Uuid) -> Result<Booking> {
    let booking = sqlx::query_as::<_, Booking>(
        r#"
        UPDATE bookings
        SET cancelled_at = NOW(),
            payment_status = 'cancelled',
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(booking)
}

/// Update payment status
pub async fn update_payment_status(
    pool: &PgPool,
    id: Uuid,
    status: &str,
    stripe_payment_id: Option<&str>,
) -> Result<Booking> {
    let booking = sqlx::query_as::<_, Booking>(
        r#"
        UPDATE bookings
        SET payment_status = $2,
            stripe_payment_id = COALESCE($3, stripe_payment_id),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(status)
    .bind(stripe_payment_id)
    .fetch_one(pool)
    .await?;

    Ok(booking)
}

/// Find unpaid expired bookings (for background job)
pub async fn find_unpaid_expired_bookings(
    pool: &PgPool,
    before: NaiveDateTime,
) -> Result<Vec<Booking>> {
    let bookings = sqlx::query_as::<_, Booking>(
        r#"
        SELECT * FROM bookings
        WHERE payment_status = 'pending'
          AND payment_deadline < $1
          AND cancelled_at IS NULL
        "#
    )
    .bind(before)
    .fetch_all(pool)
    .await?;

    Ok(bookings)
}
