use chrono::{NaiveDateTime, Utc};
use loafy_db::{
    models::{Booking, transaction_types},
    queries::{bookings, sessions, subscriptions, ticket_transactions},
    PgPool,
};
use loafy_types::AppError;
use uuid::Uuid;

/// Default cancellation hours if not set on session
const DEFAULT_DROP_IN_CANCELLATION_HOURS: i32 = 48;
const DEFAULT_SUBSCRIBER_CANCELLATION_HOURS: i32 = 24;

/// Cancel booking and return slots
/// If a ticket was used for the booking, it will be restored to the subscription
pub async fn cancel_booking(
    pool: &PgPool,
    booking_id: Uuid,
    user_id: Uuid,
) -> Result<Booking, AppError> {
    // Get booking
    let booking = bookings::find_by_id(pool, booking_id)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Booking not found".to_string()))?;

    // Check ownership
    if booking.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    // Check if already cancelled
    if booking.cancelled_at.is_some() {
        return Err(AppError::BadRequest("Booking already cancelled".to_string()));
    }

    // Get session to check cancellation deadline
    let session = sessions::find_by_id(pool, booking.session_id)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;

    // Check if user has an active subscription
    let is_subscriber = subscriptions::has_active_subscription(pool, user_id)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Get cancellation hours based on subscription status
    let cancellation_hours = if is_subscriber {
        session.subscriber_cancellation_hours
            .unwrap_or(DEFAULT_SUBSCRIBER_CANCELLATION_HOURS)
    } else {
        session.drop_in_cancellation_hours
            .unwrap_or(DEFAULT_DROP_IN_CANCELLATION_HOURS)
    };

    // Calculate session start datetime
    let session_start = NaiveDateTime::new(session.date, session.time)
        .and_utc();

    // Calculate cancellation deadline
    let cancellation_deadline = session_start - chrono::Duration::hours(cancellation_hours as i64);

    // Check if cancellation is still allowed
    let now = Utc::now();
    if now > cancellation_deadline {
        let hours_until_session = (session_start - now).num_hours();
        return Err(AppError::BadRequest(format!(
            "Cancellation deadline has passed. {} must cancel at least {} hours before the session. Session starts in {} hours.",
            if is_subscriber { "Subscribers" } else { "Drop-in players" },
            cancellation_hours,
            hours_until_session.max(0)
        )));
    }

    // Restore ticket if one was used for this booking
    if booking.tickets_used > 0 {
        if let Ok(Some(subscription)) = subscriptions::find_by_user_id(pool, user_id).await {
            // Restore the ticket
            if let Ok(new_balance) = subscriptions::restore_ticket(pool, subscription.id).await {
                // Log the ticket restoration transaction
                let _ = ticket_transactions::create_with_pool(
                    pool,
                    user_id,
                    Some(subscription.id),
                    Some(booking_id),
                    transaction_types::RESTORED,
                    1, // positive for restoration
                    new_balance,
                    Some("Restored from cancelled booking"),
                    None,
                )
                .await;
            }
        }
    }

    // Cancel booking
    let cancelled_booking = bookings::cancel_booking(pool, booking_id)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Return slots to session
    let slots_to_return = 1 + booking.guest_count;
    sessions::increment_available_slots(pool, booking.session_id, slots_to_return)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Note: Stripe refund is handled in the API layer (routes/bookings.rs)
    // after this function returns successfully

    Ok(cancelled_booking)
}
