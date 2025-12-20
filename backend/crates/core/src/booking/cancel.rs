use loafy_db::{models::Booking, queries::{bookings, sessions}, PgPool};
use loafy_types::AppError;
use uuid::Uuid;

/// Cancel booking and return slots
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

    // TODO: Check cancellation deadline (Phase 1: allow all cancellations)

    // Cancel booking
    let cancelled_booking = bookings::cancel_booking(pool, booking_id)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Return slots to session
    let slots_to_return = 1 + booking.guest_count;
    sessions::increment_available_slots(pool, booking.session_id, slots_to_return)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // TODO: Process refund if payment was confirmed (Phase 1: Stripe refund)

    Ok(cancelled_booking)
}
