use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[default]
    User,
    Organizer,
    Admin,
    Moderator,
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Self::Admin),
            "organizer" => Ok(Self::Organizer),
            "moderator" => Ok(Self::Moderator),
            "user" | _ => Ok(Self::User),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    #[default]
    Stripe,
    QrTransfer,
}

impl FromStr for PaymentMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stripe" => Ok(Self::Stripe),
            "qr_transfer" | _ => Ok(Self::QrTransfer),
        }
    }
}

impl PaymentMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stripe => "stripe",
            Self::QrTransfer => "qr_transfer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    #[default]
    Pending,
    Confirmed,
    Refunded,
    Cancelled,
}

impl FromStr for PaymentStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "confirmed" => Ok(Self::Confirmed),
            "refunded" => Ok(Self::Refunded),
            "cancelled" => Ok(Self::Cancelled),
            "pending" | _ => Ok(Self::Pending),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    #[default]
    Pending,
    AutoConfirmed,
    PendingReview,
    Confirmed,
    Rejected,
}

impl FromStr for VerificationStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto_confirmed" => Ok(Self::AutoConfirmed),
            "pending_review" => Ok(Self::PendingReview),
            "confirmed" => Ok(Self::Confirmed),
            "rejected" => Ok(Self::Rejected),
            "pending" | _ => Ok(Self::Pending),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    #[default]
    Active,
    Expired,
    Cancelled,
    PastDue,
}

impl FromStr for SubscriptionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "expired" => Ok(Self::Expired),
            "cancelled" => Ok(Self::Cancelled),
            "past_due" => Ok(Self::PastDue),
            "active" | _ => Ok(Self::Active),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    Ticket,
    OutOfTicket,
    #[default]
    None,
}

impl FromStr for DiscountType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ticket" => Ok(Self::Ticket),
            "out_of_ticket" => Ok(Self::OutOfTicket),
            "none" | _ => Ok(Self::None),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum BonusTicketType {
    Referral,
    Birthday,
    Manual,
}

impl FromStr for BonusTicketType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "birthday" => Ok(Self::Birthday),
            "manual" => Ok(Self::Manual),
            "referral" | _ => Ok(Self::Referral),
        }
    }
}
