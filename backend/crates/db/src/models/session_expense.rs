use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SessionExpense {
    pub id: Uuid,
    pub session_id: Uuid,
    pub category: String,
    pub description: Option<String>,
    pub cost_type: String,
    pub amount_vnd: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
