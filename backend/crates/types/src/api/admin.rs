use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

use super::sessions::ParticipantInfo;

// =============================================================================
// Pagination Types
// =============================================================================

/// Page information for paginated responses
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct PageInfo {
    pub page: i32,
    pub per_page: i32,
    pub total: i64,
    pub total_pages: i32,
}

/// Macro to generate paginated response types with ts-rs export.
///
/// Since ts-rs doesn't work well with generic types, we use this macro
/// to generate concrete types while eliminating the boilerplate.
macro_rules! paginated_response {
    ($name:ident, $item_type:ty, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
        #[ts(export, export_to = "../../../../frontend/src/lib/types/")]
        pub struct $name {
            pub data: Vec<$item_type>,
            pub page_info: PageInfo,
        }
    };
}

/// User restriction info for admin view
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct AdminUserRestriction {
    pub is_suspended: bool,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_until: Option<DateTime<Utc>>,
    pub suspension_reason: Option<String>,
    pub suspended_by_name: Option<String>,
}

/// User response for admin view (ts-rs exported)
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub role: String,
    pub auth_provider: String,
    pub created_at: DateTime<Utc>,
    pub restriction: AdminUserRestriction,
}

paginated_response!(
    PaginatedUsersResponse,
    AdminUserResponse,
    "Paginated users response"
);

/// Booking response for admin view (ts-rs exported)
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct AdminBookingResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub booking_code: String,
    pub guest_count: i32,
    pub total_price_vnd: i32,
    pub payment_method: String,
    pub payment_status: String,
    pub payment_deadline: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub user_email: String,
    pub user_name: Option<String>,
    pub session_title: String,
    pub session_date: NaiveDate,
    pub session_time: NaiveTime,
}

paginated_response!(
    PaginatedBookingsResponse,
    AdminBookingResponse,
    "Paginated bookings response"
);

/// Session response for admin view (ts-rs exported)
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct AdminSessionResponse {
    pub id: Uuid,
    pub organizer_id: Uuid,
    pub organizer_name: Option<String>,
    pub title: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub location: String,
    pub courts: i32,
    pub total_slots: i32,
    pub available_slots: i32,
    pub price_vnd: Option<i32>,
    pub cancelled: bool,
    pub created_at: DateTime<Utc>,
    /// Preview of confirmed participants (max 5)
    #[ts(optional)]
    pub participants_preview: Option<Vec<ParticipantInfo>>,
    /// Total count of confirmed participants
    #[ts(optional)]
    pub confirmed_count: Option<i32>,
}

paginated_response!(
    PaginatedSessionsResponse,
    AdminSessionResponse,
    "Paginated sessions response"
);

// =============================================================================
// Profit & Stats Types
// =============================================================================

/// Profit statistics response
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct ProfitStatsResponse {
    pub total_revenue_vnd: i64,
    pub total_expenses_vnd: i64,
    pub net_profit_vnd: i64,
    pub profit_margin_percent: f64,
}

/// Per-session profit summary
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SessionProfitResponse {
    pub session_id: Uuid,
    pub title: String,
    pub date: NaiveDate,
    pub revenue_vnd: i64,
    pub expenses_vnd: i64,
    pub profit_vnd: i64,
    pub profit_margin_percent: f64,
}

/// Expense breakdown by category
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct ExpenseCategoryResponse {
    pub category: String,
    pub total_vnd: i64,
    pub percentage: f64,
}

/// Daily profit data point for charts
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct DailyProfitDataPoint {
    pub date: String,
    pub revenue: i64,
    pub expenses: i64,
    pub profit: i64,
}

/// Request to suspend a user
#[derive(Debug, Clone, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SuspendUserRequest {
    pub reason: String,
    #[serde(default)]
    pub until: Option<DateTime<Utc>>,
}

/// User restriction info for admin view
#[derive(Debug, Clone, Serialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct UserRestrictionInfo {
    pub is_suspended: bool,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_until: Option<DateTime<Utc>>,
    pub suspension_reason: Option<String>,
    pub suspended_by_name: Option<String>,
}

/// Error response for suspended users
#[derive(Debug, Clone, Serialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SuspendedUserError {
    pub error: String,
    pub reason: String,
    pub until: Option<DateTime<Utc>>,
}

// =============================================================================
// Admin User Edit Types
// =============================================================================

/// Request to update a user (admin only)
/// All fields are optional - only provided fields will be updated
#[derive(Debug, Clone, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct UpdateUserRequest {
    /// Update user's display name
    #[serde(default)]
    pub name: Option<String>,
    /// Update user's phone number
    #[serde(default)]
    pub phone: Option<String>,
    /// Update user's role (user, organizer, admin)
    #[serde(default)]
    pub role: Option<String>,
}

// =============================================================================
// Admin Booking Edit Types
// =============================================================================

/// Request to update a booking (admin only)
/// All fields are optional - only provided fields will be updated
#[derive(Debug, Clone, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct UpdateBookingRequest {
    /// Change the number of guests (affects slot availability)
    #[serde(default)]
    pub guest_count: Option<i32>,
    /// Override the price paid in VND
    #[serde(default)]
    pub price_paid_vnd: Option<i32>,
    /// Override the guest price paid in VND
    #[serde(default)]
    pub guest_price_paid_vnd: Option<i32>,
    /// Change the payment method (qr, stripe, cash, etc.)
    #[serde(default)]
    pub payment_method: Option<String>,
    /// Change the payment status (pending, confirmed, failed, refunded)
    #[serde(default)]
    pub payment_status: Option<String>,
    /// Admin notes for this edit (for audit purposes)
    #[serde(default)]
    pub admin_notes: Option<String>,
}
