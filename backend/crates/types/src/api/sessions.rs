use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SessionResponse {
    pub id: Uuid,
    pub organizer_id: Uuid,
    pub organizer_name: Option<String>,
    pub title: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub location: String,
    pub courts: i32,
    pub max_players_per_court: i32,
    pub total_slots: i32,
    pub available_slots: i32,
    pub price_vnd: i32,
    pub price_usd: Option<String>,
    pub cancelled: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSessionRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    #[validate(length(min = 1))]
    pub location: String,
    #[validate(range(min = 1, max = 10))]
    pub courts: i32,
    pub max_players_per_court: Option<i32>,
    pub price_vnd: Option<i32>,
}
