use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Organizer,
    Admin,
    Moderator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Stripe,
    QrTransfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Confirmed,
    Refunded,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    AutoConfirmed,
    PendingReview,
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Expired,
    Cancelled,
    PastDue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    Ticket,
    OutOfTicket,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum BonusTicketType {
    Referral,
    Birthday,
    Manual,
}
