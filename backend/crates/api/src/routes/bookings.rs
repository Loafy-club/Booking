use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use loafy_core::booking::{cancel_booking, create_booking_with_lock};
use loafy_db::queries::bookings;
use loafy_types::{
    api::bookings::{BookingResponse, CreateBookingRequest},
    enums::{DiscountType, PaymentMethod, PaymentStatus},
};
use uuid::Uuid;
use validator::Validate;

use crate::middleware::{AppState, AuthUser};

/// List my bookings
pub async fn list_my_bookings(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<BookingResponse>>, (StatusCode, String)> {
    let db_bookings = bookings::list_user_bookings(&state.db, user.id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch bookings: {}", e),
            )
        })?;

    let response: Vec<BookingResponse> = db_bookings
        .into_iter()
        .map(|b| BookingResponse {
            id: b.id,
            user_id: b.user_id,
            session_id: b.session_id,
            booking_code: b.booking_code,
            guest_count: b.guest_count,
            tickets_used: b.tickets_used,
            discount_applied: match b.discount_applied.as_str() {
                "ticket" => DiscountType::Ticket,
                "out_of_ticket" => DiscountType::OutOfTicket,
                _ => DiscountType::None,
            },
            price_paid_vnd: b.price_paid_vnd,
            guest_price_paid_vnd: b.guest_price_paid_vnd,
            total_paid_vnd: b.price_paid_vnd + b.guest_price_paid_vnd,
            payment_method: if b.payment_method == "stripe" {
                PaymentMethod::Stripe
            } else {
                PaymentMethod::QrTransfer
            },
            payment_status: match b.payment_status.as_str() {
                "confirmed" => PaymentStatus::Confirmed,
                "refunded" => PaymentStatus::Refunded,
                "cancelled" => PaymentStatus::Cancelled,
                _ => PaymentStatus::Pending,
            },
            verification_status: b.verification_status.map(|s| {
                match s.as_str() {
                    "auto_confirmed" => loafy_types::enums::VerificationStatus::AutoConfirmed,
                    "pending_review" => loafy_types::enums::VerificationStatus::PendingReview,
                    "confirmed" => loafy_types::enums::VerificationStatus::Confirmed,
                    "rejected" => loafy_types::enums::VerificationStatus::Rejected,
                    _ => loafy_types::enums::VerificationStatus::Pending,
                }
            }),
            payment_deadline: b.payment_deadline,
            created_at: b.created_at,
        })
        .collect();

    Ok(Json(response))
}

/// Get booking by ID
pub async fn get_booking(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, (StatusCode, String)> {
    let booking = bookings::find_by_id(&state.db, id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, "Booking not found".to_string())
        })?;

    // Check ownership
    if booking.user_id != user.id {
        return Err((
            StatusCode::FORBIDDEN,
            "You can only view your own bookings".to_string(),
        ));
    }

    let response = BookingResponse {
        id: booking.id,
        user_id: booking.user_id,
        session_id: booking.session_id,
        booking_code: booking.booking_code,
        guest_count: booking.guest_count,
        tickets_used: booking.tickets_used,
        discount_applied: match booking.discount_applied.as_str() {
            "ticket" => DiscountType::Ticket,
            "out_of_ticket" => DiscountType::OutOfTicket,
            _ => DiscountType::None,
        },
        price_paid_vnd: booking.price_paid_vnd,
        guest_price_paid_vnd: booking.guest_price_paid_vnd,
        total_paid_vnd: booking.price_paid_vnd + booking.guest_price_paid_vnd,
        payment_method: if booking.payment_method == "stripe" {
            PaymentMethod::Stripe
        } else {
            PaymentMethod::QrTransfer
        },
        payment_status: match booking.payment_status.as_str() {
            "confirmed" => PaymentStatus::Confirmed,
            "refunded" => PaymentStatus::Refunded,
            "cancelled" => PaymentStatus::Cancelled,
            _ => PaymentStatus::Pending,
        },
        verification_status: booking.verification_status.map(|s| {
            match s.as_str() {
                "auto_confirmed" => loafy_types::enums::VerificationStatus::AutoConfirmed,
                "pending_review" => loafy_types::enums::VerificationStatus::PendingReview,
                "confirmed" => loafy_types::enums::VerificationStatus::Confirmed,
                "rejected" => loafy_types::enums::VerificationStatus::Rejected,
                _ => loafy_types::enums::VerificationStatus::Pending,
            }
        }),
        payment_deadline: booking.payment_deadline,
        created_at: booking.created_at,
    };

    Ok(Json(response))
}

/// Create booking
pub async fn create_booking(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateBookingRequest>,
) -> Result<Json<BookingResponse>, (StatusCode, String)> {
    // Validate input
    payload.validate().map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Validation error: {}", e))
    })?;

    // Convert payment method enum to string
    let payment_method_str = match payload.payment_method {
        PaymentMethod::Stripe => "stripe",
        PaymentMethod::QrTransfer => "qr_transfer",
    };

    // Create booking with race condition protection
    let booking = create_booking_with_lock(
        &state.db,
        user.id,
        payload.session_id,
        payload.guest_count,
        payment_method_str,
    )
    .await
    .map_err(|e| {
        let status = e.status_code();
        (
            StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            e.to_string(),
        )
    })?;

    let response = BookingResponse {
        id: booking.id,
        user_id: booking.user_id,
        session_id: booking.session_id,
        booking_code: booking.booking_code,
        guest_count: booking.guest_count,
        tickets_used: booking.tickets_used,
        discount_applied: DiscountType::None, // Phase 1: no discounts
        price_paid_vnd: booking.price_paid_vnd,
        guest_price_paid_vnd: booking.guest_price_paid_vnd,
        total_paid_vnd: booking.price_paid_vnd + booking.guest_price_paid_vnd,
        payment_method: payload.payment_method,
        payment_status: PaymentStatus::Pending,
        verification_status: None,
        payment_deadline: booking.payment_deadline,
        created_at: booking.created_at,
    };

    Ok(Json(response))
}

/// Cancel booking
pub async fn cancel_booking_route(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, (StatusCode, String)> {
    let cancelled_booking = cancel_booking(&state.db, id, user.id)
        .await
        .map_err(|e| {
            let status = e.status_code();
            (
                StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                e.to_string(),
            )
        })?;

    let response = BookingResponse {
        id: cancelled_booking.id,
        user_id: cancelled_booking.user_id,
        session_id: cancelled_booking.session_id,
        booking_code: cancelled_booking.booking_code,
        guest_count: cancelled_booking.guest_count,
        tickets_used: cancelled_booking.tickets_used,
        discount_applied: DiscountType::None,
        price_paid_vnd: cancelled_booking.price_paid_vnd,
        guest_price_paid_vnd: cancelled_booking.guest_price_paid_vnd,
        total_paid_vnd: cancelled_booking.price_paid_vnd + cancelled_booking.guest_price_paid_vnd,
        payment_method: if cancelled_booking.payment_method == "stripe" {
            PaymentMethod::Stripe
        } else {
            PaymentMethod::QrTransfer
        },
        payment_status: PaymentStatus::Cancelled,
        verification_status: None,
        payment_deadline: cancelled_booking.payment_deadline,
        created_at: cancelled_booking.created_at,
    };

    Ok(Json(response))
}
