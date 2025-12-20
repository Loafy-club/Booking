use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct NotificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub title: String,
    pub message: Option<String>,
    pub link: Option<String>,
    pub read: bool,
    pub created_at: NaiveDateTime,
}
