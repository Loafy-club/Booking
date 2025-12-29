use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Basic participant info for session previews
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct ParticipantInfo {
    pub id: Uuid,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub guest_count: i32,
}

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SessionResponse {
    pub id: Uuid,
    pub organizer_id: Uuid,
    pub organizer_name: Option<String>,
    pub title: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    #[ts(optional)]
    pub end_time: Option<NaiveTime>,
    pub location: String,
    pub courts: i32,
    pub max_players_per_court: i32,
    pub total_slots: i32,
    pub available_slots: i32,
    pub price_vnd: i32,
    pub price_usd: Option<String>,
    pub cancelled: bool,
    #[ts(optional)]
    pub expenses: Option<Vec<ExpenseResponse>>,
    #[ts(optional)]
    pub total_expenses_vnd: Option<i64>,
    /// Preview of confirmed participants (max 5)
    #[ts(optional)]
    pub participants_preview: Option<Vec<ParticipantInfo>>,
    /// Total count of confirmed participants
    #[ts(optional)]
    pub confirmed_count: Option<i32>,
}

/// Expense input for creating/updating session expenses
#[derive(Debug, Clone, Deserialize, Serialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct ExpenseInput {
    /// Category: court_rental, equipment, instructor, custom
    pub category: String,
    /// Description (required for custom category)
    pub description: Option<String>,
    /// Cost type: per_court or total
    pub cost_type: String,
    /// Amount in VND
    pub amount_vnd: i32,
}

/// Expense response from API
#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct ExpenseResponse {
    pub id: Uuid,
    pub category: String,
    pub description: Option<String>,
    pub cost_type: String,
    pub amount_vnd: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSessionRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub description: Option<String>,
    #[validate(length(min = 1))]
    pub location: String,
    /// ISO 8601 datetime string (e.g., "2025-12-29T10:00")
    pub start_time: String,
    /// ISO 8601 datetime string (e.g., "2025-12-29T12:00")
    pub end_time: String,
    /// Total maximum slots for this session
    #[validate(range(min = 1, max = 100))]
    pub max_slots: i32,
    pub price_vnd: Option<i32>,
    pub early_access_ends_at: Option<String>,
    /// Optional expenses for this session
    pub expenses: Option<Vec<ExpenseInput>>,
}

/// Response for session participants list
#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SessionParticipantsResponse {
    pub session_id: Uuid,
    pub participants: Vec<ParticipantInfo>,
    pub total_count: i32,
}
