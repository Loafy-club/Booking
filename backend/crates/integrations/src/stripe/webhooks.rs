use anyhow::{anyhow, Result};
use loafy_db::{queries::bookings, PgPool};
use stripe::{Event, EventObject, EventType, Webhook};
use uuid::Uuid;

/// Handle Stripe webhook event
pub async fn handle_stripe_webhook(
    payload: &str,
    signature: &str,
    webhook_secret: &str,
    pool: &PgPool,
) -> Result<()> {
    // Verify webhook signature
    let event = Webhook::construct_event(payload, signature, webhook_secret)
        .map_err(|e| anyhow!("Failed to verify webhook signature: {}", e))?;

    // TODO: Check if event already processed (idempotency)

    // Handle event based on type
    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            handle_payment_succeeded(event, pool).await?;
        }
        EventType::PaymentIntentPaymentFailed => {
            handle_payment_failed(event, pool).await?;
        }
        EventType::PaymentIntentCanceled => {
            handle_payment_canceled(event, pool).await?;
        }
        _ => {
            // Ignore other events
            tracing::debug!("Unhandled webhook event type: {:?}", event.type_);
        }
    }

    // TODO: Mark event as processed

    Ok(())
}

async fn handle_payment_succeeded(event: Event, pool: &PgPool) -> Result<()> {
    if let EventObject::PaymentIntent(payment_intent) = event.data.object {
        // Get booking ID from metadata
        let booking_id = payment_intent
            .metadata
            .as_ref()
            .and_then(|m| m.get("booking_id"))
            .ok_or_else(|| anyhow!("No booking_id in payment intent metadata"))?;

        let booking_uuid = Uuid::parse_str(booking_id)
            .map_err(|e| anyhow!("Invalid booking ID: {}", e))?;

        // Update booking payment status
        bookings::update_payment_status(
            pool,
            booking_uuid,
            "confirmed",
            Some(&payment_intent.id.to_string()),
        )
        .await?;

        tracing::info!(
            "Payment succeeded for booking {}: {}",
            booking_id,
            payment_intent.id
        );
    }

    Ok(())
}

async fn handle_payment_failed(event: Event, pool: &PgPool) -> Result<()> {
    if let EventObject::PaymentIntent(payment_intent) = event.data.object {
        // Get booking ID from metadata
        let booking_id = payment_intent
            .metadata
            .as_ref()
            .and_then(|m| m.get("booking_id"))
            .ok_or_else(|| anyhow!("No booking_id in payment intent metadata"))?;

        tracing::warn!(
            "Payment failed for booking {}: {}",
            booking_id,
            payment_intent.id
        );

        // Note: We don't update booking status here
        // Let the background job handle timeout and cancellation
    }

    Ok(())
}

async fn handle_payment_canceled(event: Event, pool: &PgPool) -> Result<()> {
    if let EventObject::PaymentIntent(payment_intent) = event.data.object {
        // Get booking ID from metadata
        let booking_id = payment_intent
            .metadata
            .as_ref()
            .and_then(|m| m.get("booking_id"))
            .ok_or_else(|| anyhow!("No booking_id in payment intent metadata"))?;

        tracing::info!(
            "Payment canceled for booking {}: {}",
            booking_id,
            payment_intent.id
        );

        // Note: We don't update booking status here
        // Let the background job handle timeout and cancellation
    }

    Ok(())
}
