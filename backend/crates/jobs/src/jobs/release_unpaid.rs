use chrono::Utc;
use loafy_db::{queries::{bookings, sessions}, PgPool};

/// Release unpaid bookings past their payment deadline
/// Runs every 1 minute
pub async fn release_unpaid_bookings(pool: &PgPool) -> anyhow::Result<()> {
    let now = Utc::now().naive_utc();

    // Find bookings past deadline
    let expired_bookings = bookings::find_unpaid_expired_bookings(pool, now).await?;

    if expired_bookings.is_empty() {
        return Ok(());
    }

    tracing::info!(
        "Found {} expired unpaid bookings to release",
        expired_bookings.len()
    );

    for booking in expired_bookings {
        tracing::info!(
            "Releasing booking {} (session: {}, deadline: {:?})",
            booking.booking_code,
            booking.session_id,
            booking.payment_deadline
        );

        // Cancel booking
        match bookings::cancel_booking(pool, booking.id).await {
            Ok(_) => {
                // Return slots to session
                let slots_to_return = 1 + booking.guest_count;

                if let Err(e) = sessions::increment_available_slots(
                    pool,
                    booking.session_id,
                    slots_to_return,
                )
                .await
                {
                    tracing::error!(
                        "Failed to return slots for booking {}: {}",
                        booking.booking_code,
                        e
                    );
                }

                tracing::info!(
                    "✓ Released booking {} - returned {} slots",
                    booking.booking_code,
                    slots_to_return
                );
            }
            Err(e) => {
                tracing::error!(
                    "Failed to cancel booking {}: {}",
                    booking.booking_code,
                    e
                );
            }
        }
    }

    Ok(())
}
