use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::enums::UserRole;

#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub provider: String,
    pub token: String,
}
