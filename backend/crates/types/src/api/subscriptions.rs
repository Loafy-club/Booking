use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::enums::SubscriptionStatus;
use super::admin::PageInfo;

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

/// User's current ticket balance response
#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct TicketBalanceResponse {
    pub tickets_remaining: i32,
    pub has_active_subscription: bool,
    pub current_period_end: Option<NaiveDateTime>,
}

/// Single ticket transaction in the history
#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct TicketTransactionResponse {
    pub id: Uuid,
    pub transaction_type: String,
    pub amount: i32,
    pub balance_after: i32,
    pub notes: Option<String>,
    pub booking_code: Option<String>,
    pub created_at: NaiveDateTime,
}

/// Paginated list of ticket transactions
#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct TicketTransactionsResponse {
    pub data: Vec<TicketTransactionResponse>,
    pub page_info: PageInfo,
}

/// Admin request to grant or revoke tickets
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdminGrantTicketsRequest {
    #[validate(range(min = 1, max = 100))]
    pub amount: i32,
    pub reason: Option<String>,
}

/// Admin view of user's tickets
#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct AdminUserTicketsResponse {
    pub user_id: Uuid,
    pub tickets_remaining: i32,
    pub has_active_subscription: bool,
    pub current_period_end: Option<NaiveDateTime>,
    pub recent_transactions: Vec<TicketTransactionResponse>,
}

/// Response for subscription checkout session creation
#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct CreateCheckoutResponse {
    pub checkout_url: String,
}

/// Detailed subscription response with Stripe-synced info
#[derive(Debug, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../../frontend/src/lib/types/")]
pub struct SubscriptionDetailResponse {
    pub id: Uuid,
    pub status: SubscriptionStatus,
    pub tickets_remaining: i32,
    pub current_period_start: Option<NaiveDateTime>,
    pub current_period_end: Option<NaiveDateTime>,
    pub auto_renew: bool,
    /// True if user cancelled but period hasn't ended yet
    pub cancel_at_period_end: bool,
    pub created_at: NaiveDateTime,
}
