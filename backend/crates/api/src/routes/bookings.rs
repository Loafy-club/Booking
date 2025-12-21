use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use loafy_core::booking::{cancel_booking, create_booking_with_lock};
use loafy_db::queries::bookings;
use loafy_types::api::bookings::{BookingResponse, CreateBookingRequest};
use uuid::Uuid;
use validator::Validate;

use crate::middleware::{AppState, AuthUser};
use crate::response::{self, ApiError};

/// List my bookings
pub async fn list_my_bookings(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<BookingResponse>>, ApiError> {
    let db_bookings = bookings::list_user_bookings(&state.db, user.id)
        .await
        .map_err(|e| response::internal_error_msg("Failed to fetch bookings", e))?;

    let response: Vec<BookingResponse> = db_bookings.into_iter().map(Into::into).collect();
    Ok(Json(response))
}

/// Get booking by ID
pub async fn get_booking(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, ApiError> {
    let booking = bookings::find_by_id(&state.db, id)
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
    let cancelled_booking = cancel_booking(&state.db, id, user.id)
        .await
        .map_err(|e| {
            let status = e.status_code();
            (
                StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                e.to_string(),
            )
        })?;

    Ok(Json(cancelled_booking.into()))
}
