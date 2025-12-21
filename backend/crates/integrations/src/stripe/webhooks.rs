use anyhow::{anyhow, Result};
use loafy_db::{queries::bookings, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// Stub Types (will be replaced with real Stripe types when integrating)
// ============================================================================

#[allow(dead_code)]
#[derive(Debug)]
pub enum EventType {
    PaymentIntentSucceeded,
    PaymentIntentPaymentFailed,
    PaymentIntentCanceled,
    Other,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Event {
    pub type_: EventType,
    pub data: EventData,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventData {
    pub object: EventObject,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EventObject {
    PaymentIntent(PaymentIntentStub),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PaymentIntentStub {
    pub id: String,
    pub metadata: Option<HashMap<String, String>>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Extract booking UUID from payment intent metadata.
/// This is a common operation across all webhook handlers.
#[allow(dead_code)]
fn extract_booking_id(metadata: &Option<HashMap<String, String>>) -> Result<Uuid> {
    let booking_id = metadata
        .as_ref()
        .and_then(|m| m.get("booking_id"))
        .ok_or_else(|| anyhow!("No booking_id in payment intent metadata"))?;

    Uuid::parse_str(booking_id).map_err(|e| anyhow!("Invalid booking UUID: {}", e))
}

// ============================================================================
// Public Webhook Handler
// ============================================================================

/// Handle Stripe webhook event
pub async fn handle_stripe_webhook(
    _payload: &str,
    _signature: &str,
    _webhook_secret: &str,
    _pool: &PgPool,
) -> Result<()> {
    // TODO: Implement proper Stripe webhook verification and handling
    // 1. Verify webhook signature using webhook_secret
    // 2. Parse event from payload
    // 3. Route to appropriate handler based on event type
    tracing::warn!("Using stub Stripe webhook handler - no actual processing");
    Ok(())
}

// ============================================================================
// Internal Webhook Handlers (will be called from handle_stripe_webhook)
// ============================================================================

#[allow(dead_code)]
async fn handle_payment_succeeded(payment_intent: PaymentIntentStub, pool: &PgPool) -> Result<()> {
    let booking_uuid = extract_booking_id(&payment_intent.metadata)?;

    bookings::update_payment_status(pool, booking_uuid, "confirmed", Some(&payment_intent.id))
        .await?;

    tracing::info!(
        "Payment succeeded for booking {} (intent: {})",
        booking_uuid,
        payment_intent.id
    );

    Ok(())
}

#[allow(dead_code)]
async fn handle_payment_failed(payment_intent: PaymentIntentStub, _pool: &PgPool) -> Result<()> {
    let booking_uuid = extract_booking_id(&payment_intent.metadata)?;

    // Note: We don't update booking status on failure
    // Let the background job handle timeout and cancellation
    tracing::warn!(
        "Payment failed for booking {} (intent: {})",
        booking_uuid,
        payment_intent.id
    );

    Ok(())
}

#[allow(dead_code)]
async fn handle_payment_canceled(payment_intent: PaymentIntentStub, _pool: &PgPool) -> Result<()> {
    let booking_uuid = extract_booking_id(&payment_intent.metadata)?;

    // Note: We don't update booking status on cancellation
    // Let the background job handle timeout and cancellation
    tracing::info!(
        "Payment canceled for booking {} (intent: {})",
        booking_uuid,
        payment_intent.id
    );

    Ok(())
}
