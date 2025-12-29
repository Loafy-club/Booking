use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::enums::{DiscountType, PaymentMethod, PaymentStatus, VerificationStatus};
use super::admin::PageInfo;

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct BookingResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub booking_code: String,
    pub guest_count: i32,
    /// Number of tickets used for this booking (0 or 1)
    pub tickets_used: i32,
    /// Type of discount applied: "ticket", "out_of_ticket", or "none"
    pub discount_applied: DiscountType,
    /// Base price per person from the session
    pub session_price_vnd: i32,
    /// Price user pays for their slot (0 if ticket used, discounted if out_of_ticket)
    pub price_paid_vnd: i32,
    /// Total price for all guests (full price per guest)
    pub guest_price_paid_vnd: i32,
    /// Total amount to pay (price_paid_vnd + guest_price_paid_vnd)
    pub total_paid_vnd: i32,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,
    pub verification_status: Option<VerificationStatus>,
    pub payment_deadline: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    // Session details for display
    pub session_title: String,
    pub session_date: NaiveDate,
    pub session_time: NaiveTime,
    pub session_end_time: Option<NaiveTime>,
    pub session_location: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookingRequest {
    pub session_id: Uuid,
    #[validate(range(min = 0, max = 10))]
    pub guest_count: i32,
    pub payment_method: PaymentMethod,
}

/// Paginated response for user bookings
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct UserBookingsResponse {
    pub data: Vec<BookingResponse>,
    pub page_info: PageInfo,
}
