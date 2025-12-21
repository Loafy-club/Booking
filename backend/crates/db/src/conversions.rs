//! Conversion implementations from DB models to API response types.
//!
//! These From implementations centralize the conversion logic that was
//! previously duplicated across multiple route handlers.

use crate::models::{Booking, Session, UserWithRole};
use loafy_types::{
    api::{AuthUser, BookingResponse, SessionResponse},
    enums::{DiscountType, PaymentMethod, PaymentStatus, UserRole, VerificationStatus},
};

// ============================================================================
// UserWithRole -> AuthUser
// ============================================================================

impl From<UserWithRole> for AuthUser {
    fn from(user: UserWithRole) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            phone: user.phone,
            avatar_url: user.avatar_url,
            role: user.role_name.parse().unwrap_or(UserRole::User),
        }
    }
}

impl From<&UserWithRole> for AuthUser {
    fn from(user: &UserWithRole) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
            phone: user.phone.clone(),
            avatar_url: user.avatar_url.clone(),
            role: user.role_name.parse().unwrap_or(UserRole::User),
        }
    }
}

// ============================================================================
// Booking -> BookingResponse
// ============================================================================

impl From<Booking> for BookingResponse {
    fn from(b: Booking) -> Self {
        Self {
            id: b.id,
            user_id: b.user_id,
            session_id: b.session_id,
            booking_code: b.booking_code,
            guest_count: b.guest_count,
            tickets_used: b.tickets_used,
            discount_applied: b.discount_applied.parse().unwrap_or(DiscountType::None),
            price_paid_vnd: b.price_paid_vnd,
            guest_price_paid_vnd: b.guest_price_paid_vnd,
            total_paid_vnd: b.price_paid_vnd + b.guest_price_paid_vnd,
            payment_method: b.payment_method.parse().unwrap_or(PaymentMethod::Stripe),
            payment_status: b.payment_status.parse().unwrap_or(PaymentStatus::Pending),
            verification_status: b
                .verification_status
                .map(|s| s.parse().unwrap_or(VerificationStatus::Pending)),
            payment_deadline: b.payment_deadline,
            created_at: b.created_at,
        }
    }
}

impl From<&Booking> for BookingResponse {
    fn from(b: &Booking) -> Self {
        Self {
            id: b.id,
            user_id: b.user_id,
            session_id: b.session_id,
            booking_code: b.booking_code.clone(),
            guest_count: b.guest_count,
            tickets_used: b.tickets_used,
            discount_applied: b.discount_applied.parse().unwrap_or(DiscountType::None),
            price_paid_vnd: b.price_paid_vnd,
            guest_price_paid_vnd: b.guest_price_paid_vnd,
            total_paid_vnd: b.price_paid_vnd + b.guest_price_paid_vnd,
            payment_method: b.payment_method.parse().unwrap_or(PaymentMethod::Stripe),
            payment_status: b.payment_status.parse().unwrap_or(PaymentStatus::Pending),
            verification_status: b
                .verification_status
                .as_ref()
                .map(|s| s.parse().unwrap_or(VerificationStatus::Pending)),
            payment_deadline: b.payment_deadline,
            created_at: b.created_at,
        }
    }
}

// ============================================================================
// Session -> SessionResponse
// ============================================================================

/// Default values for session fields
const DEFAULT_MAX_PLAYERS_PER_COURT: i32 = 6;
const DEFAULT_PRICE_VND: i32 = 100_000;

impl From<Session> for SessionResponse {
    fn from(s: Session) -> Self {
        Self {
            id: s.id,
            organizer_id: s.organizer_id,
            organizer_name: None, // Must be set explicitly if needed
            title: s.title,
            date: s.date,
            time: s.time,
            location: s.location,
            courts: s.courts,
            max_players_per_court: s.max_players_per_court.unwrap_or(DEFAULT_MAX_PLAYERS_PER_COURT),
            total_slots: s.total_slots,
            available_slots: s.available_slots,
            price_vnd: s.price_vnd.unwrap_or(DEFAULT_PRICE_VND),
            price_usd: s.price_usd.map(|d| d.to_string()),
            cancelled: s.cancelled,
        }
    }
}

impl From<&Session> for SessionResponse {
    fn from(s: &Session) -> Self {
        Self {
            id: s.id,
            organizer_id: s.organizer_id,
            organizer_name: None, // Must be set explicitly if needed
            title: s.title.clone(),
            date: s.date,
            time: s.time,
            location: s.location.clone(),
            courts: s.courts,
            max_players_per_court: s.max_players_per_court.unwrap_or(DEFAULT_MAX_PLAYERS_PER_COURT),
            total_slots: s.total_slots,
            available_slots: s.available_slots,
            price_vnd: s.price_vnd.unwrap_or(DEFAULT_PRICE_VND),
            price_usd: s.price_usd.map(|d| d.to_string()),
            cancelled: s.cancelled,
        }
    }
}

/// Extension trait for SessionResponse to set organizer name
pub trait SessionResponseExt {
    fn with_organizer_name(self, name: Option<String>) -> Self;
}

impl SessionResponseExt for SessionResponse {
    fn with_organizer_name(mut self, name: Option<String>) -> Self {
        self.organizer_name = name;
        self
    }
}


