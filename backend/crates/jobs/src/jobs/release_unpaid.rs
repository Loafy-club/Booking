use chrono::{DateTime, Utc};
use loafy_db::{
    models::transaction_types,
    queries::{bookings, sessions, subscriptions, ticket_transactions},
    PgPool,
};

/// Release unpaid bookings past their payment deadline
/// Runs every 1 minute
///
/// This job:
/// 1. Finds bookings past their payment deadline
/// 2. Restores any tickets used for the booking
/// 3. Cancels the booking
/// 4. Returns slots to the session
pub async fn release_unpaid_bookings(pool: &PgPool) -> anyhow::Result<()> {
    let now: DateTime<Utc> = Utc::now();

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

        // Restore ticket if one was used for this booking
        if booking.tickets_used > 0 {
            if let Ok(Some(subscription)) =
                subscriptions::find_by_user_id(pool, booking.user_id).await
            {
                match subscriptions::restore_ticket(pool, subscription.id).await {
                    Ok(new_balance) => {
                        // Log the ticket restoration transaction
                        let _ = ticket_transactions::create_with_pool(
                            pool,
                            booking.user_id,
                            Some(subscription.id),
                            Some(booking.id),
                            transaction_types::RESTORED,
                            1,
                            new_balance,
                            Some("Restored from expired unpaid booking"),
                            None,
                        )
                        .await;

                        tracing::info!(
                            "Restored ticket for booking {} - new balance: {}",
                            booking.booking_code,
                            new_balance
                        );
                    }
                    Err(e) => {
                        tracing::error!(
                            "Failed to restore ticket for booking {}: {}",
                            booking.booking_code,
                            e
                        );
                    }
                }
            }
        }

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
