use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Booking {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub booking_code: String,
    pub guest_count: i32,
    pub tickets_used: i32,
    pub discount_applied: String,
    pub price_paid_vnd: i32,
    pub price_paid_usd: Option<rust_decimal::Decimal>,
    pub guest_price_paid_vnd: i32,
    pub guest_price_paid_usd: Option<rust_decimal::Decimal>,
    pub payment_method: String,
    pub payment_status: String,
    pub verification_status: Option<String>,
    pub payment_screenshot_url: Option<String>,
    pub stripe_payment_id: Option<String>,
    pub payment_deadline: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Booking with session details for display purposes
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BookingWithSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub booking_code: String,
    pub guest_count: i32,
    pub tickets_used: i32,
    pub discount_applied: String,
    pub price_paid_vnd: i32,
    pub guest_price_paid_vnd: i32,
    pub payment_method: String,
    pub payment_status: String,
    pub verification_status: Option<String>,
    pub payment_deadline: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    // Session fields
    pub session_title: String,
    pub session_date: NaiveDate,
    pub session_time: NaiveTime,
    pub session_end_time: Option<NaiveTime>,
    pub session_location: String,
    pub session_price_vnd: i32,
}
