use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use loafy_db::queries::bookings;
use loafy_integrations::stripe::{handle_stripe_webhook, StripePayments};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::{AppState, AuthUser};
use crate::response::{self, ApiError};

#[derive(Debug, Deserialize)]
pub struct CreatePaymentIntentRequest {
    pub booking_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct PaymentIntentResponse {
    pub client_secret: String,
    pub payment_intent_id: String,
}

/// Create Stripe payment intent
pub async fn create_payment_intent(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreatePaymentIntentRequest>,
) -> Result<Json<PaymentIntentResponse>, ApiError> {
    // Get booking
    let booking = bookings::find_by_id(&state.db, payload.booking_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Booking"))?;

    // Check ownership
    if booking.user_id != user.id {
        return Err(response::forbidden("You can only pay for your own bookings"));
    }

    // Check if payment already confirmed
    if booking.payment_status == "confirmed" {
        return Err(response::bad_request("Booking is already paid"));
    }

    // Check if booking is cancelled
    if booking.cancelled_at.is_some() {
        return Err(response::bad_request("Booking is cancelled"));
    }

    // Get Stripe client (will be added to AppState)
    let stripe_key = std::env::var("STRIPE_SECRET_KEY")
        .map_err(|_| response::internal_error("Stripe not configured"))?;

    let stripe = StripePayments::new(stripe_key);

    // Calculate total amount
    let total_amount_vnd = booking.price_paid_vnd + booking.guest_price_paid_vnd;

    // Create payment intent
    let payment_intent = stripe
        .create_payment_intent(total_amount_vnd, &booking.id.to_string())
        .await
        .map_err(|e| response::internal_error_msg("Failed to create payment intent", e))?;

    let response = PaymentIntentResponse {
        client_secret: payment_intent
            .client_secret
            .ok_or_else(|| response::internal_error("No client secret in payment intent"))?
            .to_string(),
        payment_intent_id: payment_intent.id.to_string(),
    };

    Ok(Json(response))
}

/// Stripe webhook handler
pub async fn stripe_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, ApiError> {
    // Get Stripe signature from headers
    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| response::bad_request("Missing stripe-signature header"))?;

    // Get webhook secret
    let webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET")
        .map_err(|_| response::internal_error("Webhook secret not configured"))?;

    // Convert body to string
    let payload = String::from_utf8(body.to_vec())
        .map_err(|e| response::bad_request(format!("Invalid UTF-8 in payload: {}", e)))?;

    // Handle webhook
    handle_stripe_webhook(&payload, signature, &webhook_secret, &state.db)
        .await
        .map_err(|e| response::internal_error_msg("Webhook processing failed", e))?;

    Ok(StatusCode::OK)
}
