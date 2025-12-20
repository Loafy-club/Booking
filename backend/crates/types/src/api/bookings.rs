use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::enums::{DiscountType, PaymentMethod, PaymentStatus, VerificationStatus};

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct BookingResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub booking_code: String,
    pub guest_count: i32,
    pub tickets_used: i32,
    pub discount_applied: DiscountType,
    pub price_paid_vnd: i32,
    pub guest_price_paid_vnd: i32,
    pub total_paid_vnd: i32,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,
    pub verification_status: Option<VerificationStatus>,
    pub payment_deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookingRequest {
    pub session_id: Uuid,
    #[validate(range(min = 0, max = 10))]
    pub guest_count: i32,
    pub payment_method: PaymentMethod,
}
