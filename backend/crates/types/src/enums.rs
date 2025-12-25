use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ts_rs::TS;
use utoipa::ToSchema;

/// Macro to implement `FromStr` for enums with a default fallback.
///
/// This eliminates boilerplate for enum parsing from strings, which is common
/// when reading from database or parsing API parameters.
///
/// # Example
/// ```ignore
/// impl_enum_from_str!(UserRole, User,
///     "admin" => Admin,
///     "organizer" => Organizer
/// );
/// ```
macro_rules! impl_enum_from_str {
    ($enum_name:ident, $default:ident, $($str_val:literal => $variant:ident),+ $(,)?) => {
        impl FromStr for $enum_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($str_val => Ok(Self::$variant),)+
                    _ => Ok(Self::$default),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[default]
    User,
    Organizer,
    Admin,
}

impl_enum_from_str!(UserRole, User,
    "admin" => Admin,
    "organizer" => Organizer,
    "user" => User,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    #[default]
    Stripe,
    QrTransfer,
}

impl_enum_from_str!(PaymentMethod, QrTransfer,
    "stripe" => Stripe,
    "qr_transfer" => QrTransfer,
);

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

impl_enum_from_str!(PaymentStatus, Pending,
    "confirmed" => Confirmed,
    "refunded" => Refunded,
    "cancelled" => Cancelled,
    "pending" => Pending,
);

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

impl_enum_from_str!(VerificationStatus, Pending,
    "auto_confirmed" => AutoConfirmed,
    "pending_review" => PendingReview,
    "confirmed" => Confirmed,
    "rejected" => Rejected,
    "pending" => Pending,
);

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

impl_enum_from_str!(SubscriptionStatus, Active,
    "expired" => Expired,
    "cancelled" => Cancelled,
    "past_due" => PastDue,
    "active" => Active,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema, Default)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    Ticket,
    OutOfTicket,
    #[default]
    None,
}

impl_enum_from_str!(DiscountType, None,
    "ticket" => Ticket,
    "out_of_ticket" => OutOfTicket,
    "none" => None,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../../frontend/src/lib/types/")]
#[serde(rename_all = "snake_case")]
pub enum BonusTicketType {
    Referral,
    Birthday,
    Manual,
}

impl_enum_from_str!(BonusTicketType, Referral,
    "birthday" => Birthday,
    "manual" => Manual,
    "referral" => Referral,
);
