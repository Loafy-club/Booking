use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Ticket transaction audit record
/// Tracks all ticket operations: grants, usage, restorations, bonuses, expirations, revocations
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TicketTransaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub booking_id: Option<Uuid>,
    pub transaction_type: String,
    pub amount: i32,
    pub balance_after: i32,
    pub notes: Option<String>,
    pub admin_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Bonus ticket award record
/// Records each bonus ticket award with type and metadata
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BonusTicket {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bonus_type: String,
    pub tickets: i32,
    pub note: Option<String>,
    pub referrer_id: Option<Uuid>,
    pub granted_by: Option<Uuid>,
    pub year: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Transaction type constants
pub mod transaction_types {
    pub const SUBSCRIPTION_GRANT: &str = "subscription_grant";
    pub const USED: &str = "used";
    pub const RESTORED: &str = "restored";
    pub const BONUS_REFERRAL: &str = "bonus_referral";
    pub const BONUS_BIRTHDAY: &str = "bonus_birthday";
    pub const BONUS_MANUAL: &str = "bonus_manual";
    pub const EXPIRED: &str = "expired";
    pub const REVOKED: &str = "revoked";
}

/// Bonus type constants
pub mod bonus_types {
    pub const REFERRAL: &str = "referral";
    pub const BIRTHDAY: &str = "birthday";
    pub const MANUAL: &str = "manual";
}
