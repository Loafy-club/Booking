use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub role_id: Uuid,
    pub auth_provider: String,
    pub auth_provider_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserWithRole {
    // User fields
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub role_id: Uuid,
    pub auth_provider: String,
    pub auth_provider_id: String,
    pub user_created_at: DateTime<Utc>,
    pub user_updated_at: DateTime<Utc>,
    pub user_deleted_at: Option<DateTime<Utc>>,
    // Role fields
    pub role_name: String,
}

impl UserWithRole {
    pub fn role(&self) -> &str {
        &self.role_name
    }

    pub fn is_admin(&self) -> bool {
        self.role_name == "admin"
    }

    pub fn is_organizer(&self) -> bool {
        self.role_name == "organizer" || self.is_admin()
    }

    pub fn is_moderator(&self) -> bool {
        self.role_name == "moderator" || self.is_admin()
    }

    pub fn is_deleted(&self) -> bool {
        self.user_deleted_at.is_some()
    }
}
