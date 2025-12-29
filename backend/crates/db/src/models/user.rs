use chrono::{DateTime, NaiveDate, Utc};
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
    pub birthday: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_until: Option<DateTime<Utc>>,
    pub suspension_reason: Option<String>,
    pub suspended_by: Option<Uuid>,
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
    pub birthday: Option<NaiveDate>,
    pub user_created_at: DateTime<Utc>,
    pub user_updated_at: DateTime<Utc>,
    pub user_deleted_at: Option<DateTime<Utc>>,
    pub user_suspended_at: Option<DateTime<Utc>>,
    pub user_suspended_until: Option<DateTime<Utc>>,
    pub user_suspension_reason: Option<String>,
    pub user_suspended_by: Option<Uuid>,
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

    pub fn is_deleted(&self) -> bool {
        self.user_deleted_at.is_some()
    }

    /// Check if user is currently suspended
    pub fn is_suspended(&self) -> bool {
        if self.user_suspended_at.is_none() {
            return false;
        }
        // Check if suspension has expired
        if let Some(until) = self.user_suspended_until {
            return Utc::now() < until;
        }
        true // No expiration = indefinitely suspended
    }

    /// Get suspension reason if currently suspended
    pub fn suspension_reason(&self) -> Option<&str> {
        if self.is_suspended() {
            self.user_suspension_reason.as_deref()
        } else {
            None
        }
    }

    /// Get suspension expiration if currently suspended
    pub fn suspension_until(&self) -> Option<DateTime<Utc>> {
        if self.is_suspended() {
            self.user_suspended_until
        } else {
            None
        }
    }
}
