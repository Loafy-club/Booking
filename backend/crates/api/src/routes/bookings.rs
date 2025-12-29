use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use loafy_core::booking::{cancel_booking, create_booking_with_lock};
use loafy_db::queries::bookings;
use loafy_integrations::stripe::StripePayments;
use loafy_types::api::admin::PageInfo;
use loafy_types::api::bookings::{BookingResponse, CreateBookingRequest, UserBookingsResponse};
use serde::Deserialize;
use stripe::PaymentIntentId;
use uuid::Uuid;
use validator::Validate;

use crate::middleware::{AppState, AuthUser};
use crate::response::{self, ApiError};

/// Query parameters for bookings list endpoint
#[derive(Deserialize)]
pub struct BookingsQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    10
}

/// List my bookings with pagination
pub async fn list_my_bookings(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<BookingsQuery>,
) -> Result<Json<UserBookingsResponse>, ApiError> {
    let page = query.page.max(1);
    let per_page = query.per_page.clamp(1, 50);

    let (db_bookings, total) = bookings::list_user_bookings_paginated(&state.db, user.id, page, per_page)
        .await
        .map_err(|e| response::internal_error_msg("Failed to fetch bookings", e))?;

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    let data: Vec<BookingResponse> = db_bookings.into_iter().map(Into::into).collect();

    Ok(Json(UserBookingsResponse {
        data,
        page_info: PageInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}

/// Get booking by ID
pub async fn get_booking(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, ApiError> {
    let booking = bookings::find_by_id_with_session(&state.db, id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Booking"))?;

    // Check ownership
    if booking.user_id != user.id {
        return Err(response::forbidden("You can only view your own bookings"));
    }

    Ok(Json(booking.into()))
}

/// Create booking
pub async fn create_booking(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateBookingRequest>,
) -> Result<Json<BookingResponse>, ApiError> {
    // Validate input
    payload.validate().map_err(|e| response::bad_request(format!("Validation error: {}", e)))?;

    // Create booking with race condition protection
    let booking = create_booking_with_lock(
        &state.db,
        user.id,
        payload.session_id,
        payload.guest_count,
        payload.payment_method.as_str(),
    )
    .await
    .map_err(|e| {
        let status = e.status_code();
        (
            StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            e.to_string(),
        )
    })?;

    Ok(Json(booking.into()))
}

/// Cancel booking
pub async fn cancel_booking_route(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, ApiError> {
    // Get original booking to check payment status before cancellation
    let original_booking = bookings::find_by_id(&state.db, id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Booking"))?;

    // Check if refund will be needed (before status changes to 'cancelled')
    let needs_refund = original_booking.payment_method == "stripe"
        && original_booking.payment_status == "confirmed"
        && original_booking.stripe_payment_id.is_some();

    let cancelled_booking = cancel_booking(&state.db, id, user.id)
        .await
        .map_err(|e| {
            let status = e.status_code();
            (
                StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                e.to_string(),
            )
        })?;

    // Process Stripe refund if payment was confirmed
    if needs_refund {
        if let Some(ref payment_intent_id) = original_booking.stripe_payment_id {
            // Parse the payment intent ID
            let intent_id: PaymentIntentId = payment_intent_id.parse().map_err(|_| {
                response::internal_error("Invalid payment intent ID stored in booking")
            })?;

            // Get Stripe client
            let stripe_key = std::env::var("STRIPE_SECRET_KEY")
                .map_err(|_| response::internal_error("Stripe not configured"))?;
            let stripe = StripePayments::new(stripe_key);

            // Process refund
            stripe
                .refund_payment(&intent_id)
                .await
                .map_err(|e| response::internal_error_msg("Failed to process refund", e))?;

            tracing::info!(
                "Processed refund for cancelled booking {} (PaymentIntent: {})",
                cancelled_booking.booking_code,
                payment_intent_id
            );
        }
    }

    Ok(Json(cancelled_booking.into()))
}
