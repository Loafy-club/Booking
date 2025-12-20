use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::enums::SubscriptionStatus;

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SubscriptionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tickets_remaining: i32,
    pub status: SubscriptionStatus,
    pub current_period_start: Option<NaiveDateTime>,
    pub current_period_end: Option<NaiveDateTime>,
    pub auto_renew: bool,
    pub created_at: NaiveDateTime,
}
