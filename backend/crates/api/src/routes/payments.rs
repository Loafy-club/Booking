use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use loafy_db::queries::bookings;
use loafy_integrations::stripe::{handle_stripe_webhook, StripePayments};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::{AppState, AuthUser};

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
) -> Result<Json<PaymentIntentResponse>, (StatusCode, String)> {
    // Get booking
    let booking = bookings::find_by_id(&state.db, payload.booking_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Booking not found".to_string()))?;

    // Check ownership
    if booking.user_id != user.id {
        return Err((
            StatusCode::FORBIDDEN,
            "You can only pay for your own bookings".to_string(),
        ));
    }

    // Check if payment already confirmed
    if booking.payment_status == "confirmed" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Booking is already paid".to_string(),
        ));
    }

    // Check if booking is cancelled
    if booking.cancelled_at.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Booking is cancelled".to_string(),
        ));
    }

    // Get Stripe client (will be added to AppState)
    let stripe_key = std::env::var("STRIPE_SECRET_KEY")
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Stripe not configured".to_string(),
            )
        })?;

    let stripe = StripePayments::new(stripe_key);

    // Calculate total amount
    let total_amount_vnd = booking.price_paid_vnd + booking.guest_price_paid_vnd;

    // Create payment intent
    let payment_intent = stripe
        .create_payment_intent(total_amount_vnd, &booking.id.to_string())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create payment intent: {}", e),
            )
        })?;

    let response = PaymentIntentResponse {
        client_secret: payment_intent
            .client_secret
            .ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No client secret in payment intent".to_string(),
                )
            })?
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
) -> Result<StatusCode, (StatusCode, String)> {
    // Get Stripe signature from headers
    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Missing stripe-signature header".to_string(),
            )
        })?;

    // Get webhook secret
    let webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Webhook secret not configured".to_string(),
        )
    })?;

    // Convert body to string
    let payload = String::from_utf8(body.to_vec()).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid UTF-8 in payload: {}", e),
        )
    })?;

    // Handle webhook
    handle_stripe_webhook(&payload, signature, &webhook_secret, &state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Webhook processing failed: {}", e),
            )
        })?;

    Ok(StatusCode::OK)
}
