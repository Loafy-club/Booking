use chrono::{Duration, Utc};
use loafy_db::{
    models::{Booking, transaction_types},
    queries::{bookings, config, sessions, subscriptions, ticket_transactions},
    PgPool,
};
use loafy_types::AppError;
use uuid::Uuid;

use super::utils::generate_booking_code;

/// Create booking with race condition protection
/// CRITICAL: Uses SELECT FOR UPDATE to prevent overselling
///
/// Ticket Logic:
/// - Subscribers with tickets: Use 1 ticket for user slot (user pays 0)
/// - Subscribers without tickets: Apply out-of-ticket discount (10%)
/// - Non-subscribers: Pay full price
/// - Guests ALWAYS pay full price regardless of subscription
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

    // Check if user already has an active booking for this session
    let has_existing = bookings::has_active_booking_for_session(pool, user_id, session_id)
        .await
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))))?;

    if has_existing {
        tx.rollback().await.ok();
        return Err(AppError::Conflict("You already have a booking for this session".to_string()));
    }

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

    // Check subscription status and calculate pricing
    let subscription = subscriptions::get_active_for_booking(&mut tx, user_id)
        .await
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))))?;

    // Determine ticket usage, discount, and user price
    let (tickets_used, discount_applied, user_price_vnd, subscription_id) =
        if let Some(sub) = subscription {
            if sub.tickets_remaining > 0 {
                // Has tickets - use 1 for user's slot
                let new_balance = subscriptions::deduct_ticket(&mut tx, sub.id)
                    .await
                    .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    ))))?;

                // Log ticket transaction (booking_id will be updated after insert)
                ticket_transactions::create(
                    &mut tx,
                    user_id,
                    Some(sub.id),
                    None, // booking_id set after booking created
                    transaction_types::USED,
                    -1,
                    new_balance,
                    Some("Used for booking"),
                    None,
                )
                .await
                .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                ))))?;

                (1, "ticket", 0, Some(sub.id)) // User pays 0 VND
            } else {
                // Subscriber but out of tickets - apply discount
                let discount_percent = config::get_out_of_ticket_discount(&mut tx)
                    .await
                    .unwrap_or(10);
                let discounted_price = base_price_vnd * (100 - discount_percent) / 100;
                (0, "out_of_ticket", discounted_price, Some(sub.id))
            }
        } else {
            // Not a subscriber - full price
            (0, "none", base_price_vnd, None)
        };

    // Guests ALWAYS pay full price (no subscription benefit)
    let guest_price_vnd = base_price_vnd * guest_count;

    // Calculate total amount
    let total_amount = user_price_vnd + guest_price_vnd;

    // If total is 0 (fully covered by ticket), auto-confirm the booking
    let payment_status = if total_amount == 0 { "confirmed" } else { "pending" };

    // Generate unique booking code
    let booking_code = generate_booking_code();

    // Calculate payment deadline (30 minutes from now, only relevant if payment needed)
    let payment_deadline = if total_amount > 0 {
        Some(Utc::now() + Duration::minutes(30))
    } else {
        None // No deadline needed for free bookings
    };

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
    .bind(tickets_used)
    .bind(discount_applied)
    .bind(user_price_vnd)
    .bind(guest_price_vnd)
    .bind(payment_method)
    .bind(payment_status)
    .bind(payment_deadline)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| AppError::Database(e))?;

    // Update ticket transaction with booking_id if ticket was used
    if tickets_used > 0 {
        if let Some(sub_id) = subscription_id {
            sqlx::query(
                r#"
                UPDATE ticket_transactions
                SET booking_id = $1
                WHERE id = (
                    SELECT id FROM ticket_transactions
                    WHERE user_id = $2
                      AND subscription_id = $3
                      AND transaction_type = 'used'
                      AND booking_id IS NULL
                    ORDER BY created_at DESC
                    LIMIT 1
                )
                "#
            )
            .bind(booking.id)
            .bind(user_id)
            .bind(sub_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Database(e))?;
        }
    }

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
