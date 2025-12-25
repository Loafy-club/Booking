//! Conversion implementations from DB models to API response types.
//!
//! These From implementations centralize the conversion logic that was
//! previously duplicated across multiple route handlers.

use crate::models::{Booking, Session, SessionExpense, UserWithRole};
use loafy_types::{
    api::{
        admin::{AdminUserRestriction, AdminUserResponse},
        AuthUser, BookingResponse, ExpenseResponse, ParticipantInfo, SessionResponse,
    },
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
// UserWithRole -> AdminUserResponse
// ============================================================================

impl From<UserWithRole> for AdminUserResponse {
    fn from(u: UserWithRole) -> Self {
        let is_suspended = u.is_suspended();
        Self {
            id: u.id,
            email: u.email,
            name: u.name,
            avatar_url: u.avatar_url,
            phone: u.phone,
            role: u.role_name,
            auth_provider: u.auth_provider,
            created_at: u.user_created_at,
            restriction: AdminUserRestriction {
                is_suspended,
                suspended_at: u.user_suspended_at,
                suspended_until: u.user_suspended_until,
                suspension_reason: u.user_suspension_reason,
                suspended_by_name: None,
            },
        }
    }
}

impl From<&UserWithRole> for AdminUserResponse {
    fn from(u: &UserWithRole) -> Self {
        let is_suspended = u.is_suspended();
        Self {
            id: u.id,
            email: u.email.clone(),
            name: u.name.clone(),
            avatar_url: u.avatar_url.clone(),
            phone: u.phone.clone(),
            role: u.role_name.clone(),
            auth_provider: u.auth_provider.clone(),
            created_at: u.user_created_at,
            restriction: AdminUserRestriction {
                is_suspended,
                suspended_at: u.user_suspended_at,
                suspended_until: u.user_suspended_until,
                suspension_reason: u.user_suspension_reason.clone(),
                suspended_by_name: None,
            },
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
            expenses: None, // Must be set explicitly if needed
            total_expenses_vnd: None, // Must be set explicitly if needed
            participants_preview: None, // Must be set explicitly if needed
            confirmed_count: None, // Must be set explicitly if needed
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
            expenses: None, // Must be set explicitly if needed
            total_expenses_vnd: None, // Must be set explicitly if needed
            participants_preview: None, // Must be set explicitly if needed
            confirmed_count: None, // Must be set explicitly if needed
        }
    }
}

// ============================================================================
// SessionExpense -> ExpenseResponse
// ============================================================================

impl From<SessionExpense> for ExpenseResponse {
    fn from(e: SessionExpense) -> Self {
        Self {
            id: e.id,
            category: e.category,
            description: e.description,
            cost_type: e.cost_type,
            amount_vnd: e.amount_vnd,
        }
    }
}

impl From<&SessionExpense> for ExpenseResponse {
    fn from(e: &SessionExpense) -> Self {
        Self {
            id: e.id,
            category: e.category.clone(),
            description: e.description.clone(),
            cost_type: e.cost_type.clone(),
            amount_vnd: e.amount_vnd,
        }
    }
}

/// Extension trait for SessionResponse to set organizer name, expenses, and participants
pub trait SessionResponseExt {
    fn with_organizer_name(self, name: Option<String>) -> Self;
    fn with_expenses(self, expenses: Vec<ExpenseResponse>, total: i64) -> Self;
    fn with_participants(self, participants: Vec<ParticipantInfo>, count: i32) -> Self;
}

impl SessionResponseExt for SessionResponse {
    fn with_organizer_name(mut self, name: Option<String>) -> Self {
        self.organizer_name = name;
        self
    }

    fn with_expenses(mut self, expenses: Vec<ExpenseResponse>, total: i64) -> Self {
        self.expenses = if expenses.is_empty() { None } else { Some(expenses) };
        self.total_expenses_vnd = if total == 0 { None } else { Some(total) };
        self
    }

    fn with_participants(mut self, participants: Vec<ParticipantInfo>, count: i32) -> Self {
        self.participants_preview = if participants.is_empty() { None } else { Some(participants) };
        self.confirmed_count = Some(count);
        self
    }
}





