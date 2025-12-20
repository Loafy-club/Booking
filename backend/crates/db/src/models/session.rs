use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub organizer_id: Uuid,
    pub title: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub location: String,
    pub courts: i32,
    pub max_players_per_court: Option<i32>,
    pub total_slots: i32,
    pub available_slots: i32,
    pub price_vnd: Option<i32>,
    pub price_usd: Option<rust_decimal::Decimal>,
    pub subscriber_early_access_hours: Option<i32>,
    pub drop_in_cancellation_hours: Option<i32>,
    pub subscriber_cancellation_hours: Option<i32>,
    pub qr_code_url: Option<String>,
    pub cancelled: bool,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
