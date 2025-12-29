use axum::{extract::{Query, State}, Json};
use loafy_db::queries::{bookings, subscriptions, ticket_transactions, users};
use loafy_integrations::stripe::StripeSubscriptions;
use loafy_types::api::{
    CreateCheckoutResponse, PageInfo, SubscriptionDetailResponse, TicketBalanceResponse,
    TicketTransactionResponse, TicketTransactionsResponse,
};
use loafy_types::enums::SubscriptionStatus;
use serde::Deserialize;

use crate::middleware::AppState;
use crate::response::{self, ApiError};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

/// GET /api/subscriptions/tickets
/// Get current user's ticket balance
pub async fn get_ticket_balance(
    State(state): State<AppState>,
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
) -> Result<Json<TicketBalanceResponse>, ApiError> {
    let subscription = subscriptions::find_by_user_id(&state.db, user.id)
        .await
        .map_err(response::db_error)?;

    Ok(Json(TicketBalanceResponse {
        tickets_remaining: subscription.as_ref().map(|s| s.tickets_remaining).unwrap_or(0),
        has_active_subscription: subscription.as_ref().map(|s| s.is_active()).unwrap_or(false),
        current_period_end: subscription.and_then(|s| s.current_period_end.map(|dt| dt.naive_utc())),
    }))
}

/// GET /api/subscriptions/tickets/history
/// Get current user's ticket transaction history
pub async fn get_ticket_history(
    State(state): State<AppState>,
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<TicketTransactionsResponse>, ApiError> {
    let page = query.page.max(1);
    let per_page = query.per_page.clamp(1, 50);

    let (transactions, total) =
        ticket_transactions::list_user_transactions(&state.db, user.id, page, per_page)
            .await
            .map_err(response::db_error)?;

    // Convert to response format with booking codes
    let mut responses = Vec::with_capacity(transactions.len());
    for tx in transactions {
        let booking_code = if let Some(booking_id) = tx.booking_id {
            bookings::find_by_id(&state.db, booking_id)
                .await
                .ok()
                .flatten()
                .map(|b| b.booking_code)
        } else {
            None
        };

        responses.push(TicketTransactionResponse {
            id: tx.id,
            transaction_type: tx.transaction_type,
            amount: tx.amount,
            balance_after: tx.balance_after,
            notes: tx.notes,
            booking_code,
            created_at: tx.created_at.naive_utc(),
        });
    }

    let total_pages = (total as f64 / per_page as f64).ceil() as i32;

    Ok(Json(TicketTransactionsResponse {
        data: responses,
        page_info: PageInfo {
            page: page as i32,
            per_page: per_page as i32,
            total,
            total_pages,
        },
    }))
}

/// Helper to get Stripe subscriptions client
fn get_stripe_subscriptions() -> Result<StripeSubscriptions, ApiError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY")
        .map_err(|_| response::internal_error("Stripe not configured"))?;
    let price_id = std::env::var("STRIPE_SUBSCRIPTION_PRICE_ID")
        .map_err(|_| response::internal_error("Subscription price not configured"))?;

    Ok(StripeSubscriptions::new(secret_key, price_id))
}

/// POST /api/subscriptions/purchase
/// Create a Stripe Checkout session for subscription purchase
pub async fn create_checkout_session(
    State(state): State<AppState>,
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
) -> Result<Json<CreateCheckoutResponse>, ApiError> {
    // Check if user already has active subscription
    let existing = subscriptions::find_by_user_id(&state.db, user.id)
        .await
        .map_err(response::db_error)?;

    if let Some(sub) = existing {
        if sub.status == "active" {
            return Err(response::bad_request("You already have an active subscription"));
        }
    }

    // Get user details for Stripe customer
    let user_details = users::find_by_id(&state.db, user.id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    // Get Stripe client
    let stripe = get_stripe_subscriptions()?;

    // Get or create Stripe customer
    let customer = stripe
        .get_or_create_customer(
            &user.id.to_string(),
            &user_details.email,
            user_details.name.as_deref(),
        )
        .await
        .map_err(|e| response::internal_error_msg("Failed to create customer", e))?;

    // Build URLs
    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());
    let success_url = format!("{}/subscriptions/success?session_id={{CHECKOUT_SESSION_ID}}", frontend_url);
    let cancel_url = format!("{}/subscriptions/cancelled", frontend_url);

    // Create checkout session
    let session = stripe
        .create_checkout_session(
            customer.id.as_str(),
            &user.id.to_string(),
            &success_url,
            &cancel_url,
        )
        .await
        .map_err(|e| response::internal_error_msg("Failed to create checkout session", e))?;

    let checkout_url = session
        .url
        .ok_or_else(|| response::internal_error("No URL in checkout session"))?;

    Ok(Json(CreateCheckoutResponse { checkout_url }))
}

/// GET /api/subscriptions/current
/// Get user's current subscription details
pub async fn get_current_subscription(
    State(state): State<AppState>,
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
) -> Result<Json<Option<SubscriptionDetailResponse>>, ApiError> {
    let subscription = subscriptions::find_by_user_id(&state.db, user.id)
        .await
        .map_err(response::db_error)?;

    let response = subscription.map(|sub| {
        let status = match sub.status.as_str() {
            "active" => SubscriptionStatus::Active,
            "expired" => SubscriptionStatus::Expired,
            "cancelled" => SubscriptionStatus::Cancelled,
            "past_due" => SubscriptionStatus::PastDue,
            _ => SubscriptionStatus::Expired,
        };

        SubscriptionDetailResponse {
            id: sub.id,
            status,
            tickets_remaining: sub.tickets_remaining,
            current_period_start: sub.current_period_start.map(|dt| dt.naive_utc()),
            current_period_end: sub.current_period_end.map(|dt| dt.naive_utc()),
            auto_renew: sub.auto_renew,
            cancel_at_period_end: !sub.auto_renew && sub.status == "active",
            created_at: sub.created_at.naive_utc(),
        }
    });

    Ok(Json(response))
}

/// POST /api/subscriptions/cancel
/// Cancel auto-renewal (subscription continues until period end)
pub async fn cancel_subscription(
    State(state): State<AppState>,
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
) -> Result<Json<SubscriptionDetailResponse>, ApiError> {
    // Get subscription
    let subscription = subscriptions::find_by_user_id(&state.db, user.id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Subscription"))?;

    // Check if already cancelled
    if !subscription.auto_renew {
        return Err(response::bad_request("Subscription auto-renewal is already cancelled"));
    }

    // Check if subscription has a Stripe ID
    let stripe_sub_id = subscription
        .stripe_subscription_id
        .as_ref()
        .ok_or_else(|| response::bad_request("No Stripe subscription linked"))?;

    // Cancel in Stripe
    let stripe = get_stripe_subscriptions()?;
    stripe
        .cancel_at_period_end(stripe_sub_id)
        .await
        .map_err(|e| response::internal_error_msg("Failed to cancel subscription", e))?;

    // Update our database
    subscriptions::update_auto_renew(&state.db, subscription.id, false)
        .await
        .map_err(response::db_error)?;

    // Fetch updated subscription
    let updated = subscriptions::find_by_user_id(&state.db, user.id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Subscription"))?;

    let status = match updated.status.as_str() {
        "active" => SubscriptionStatus::Active,
        "expired" => SubscriptionStatus::Expired,
        "cancelled" => SubscriptionStatus::Cancelled,
        "past_due" => SubscriptionStatus::PastDue,
        _ => SubscriptionStatus::Expired,
    };

    Ok(Json(SubscriptionDetailResponse {
        id: updated.id,
        status,
        tickets_remaining: updated.tickets_remaining,
        current_period_start: updated.current_period_start.map(|dt| dt.naive_utc()),
        current_period_end: updated.current_period_end.map(|dt| dt.naive_utc()),
        auto_renew: updated.auto_renew,
        cancel_at_period_end: !updated.auto_renew && updated.status == "active",
        created_at: updated.created_at.naive_utc(),
    }))
}

/// POST /api/subscriptions/resume
/// Resume auto-renewal before period ends
pub async fn resume_subscription(
    State(state): State<AppState>,
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
) -> Result<Json<SubscriptionDetailResponse>, ApiError> {
    // Get subscription
    let subscription = subscriptions::find_by_user_id(&state.db, user.id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Subscription"))?;

    // Check status
    if subscription.status != "active" {
        return Err(response::bad_request("Subscription is not active"));
    }

    // Check if not cancelled
    if subscription.auto_renew {
        return Err(response::bad_request("Subscription is not set to cancel"));
    }

    // Check if subscription has a Stripe ID
    let stripe_sub_id = subscription
        .stripe_subscription_id
        .as_ref()
        .ok_or_else(|| response::bad_request("No Stripe subscription linked"))?;

    // Resume in Stripe
    let stripe = get_stripe_subscriptions()?;
    stripe
        .resume_subscription(stripe_sub_id)
        .await
        .map_err(|e| response::internal_error_msg("Failed to resume subscription", e))?;

    // Update our database
    subscriptions::update_auto_renew(&state.db, subscription.id, true)
        .await
        .map_err(response::db_error)?;

    // Fetch updated subscription
    let updated = subscriptions::find_by_user_id(&state.db, user.id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Subscription"))?;

    let status = match updated.status.as_str() {
        "active" => SubscriptionStatus::Active,
        "expired" => SubscriptionStatus::Expired,
        "cancelled" => SubscriptionStatus::Cancelled,
        "past_due" => SubscriptionStatus::PastDue,
        _ => SubscriptionStatus::Expired,
    };

    Ok(Json(SubscriptionDetailResponse {
        id: updated.id,
        status,
        tickets_remaining: updated.tickets_remaining,
        current_period_start: updated.current_period_start.map(|dt| dt.naive_utc()),
        current_period_end: updated.current_period_end.map(|dt| dt.naive_utc()),
        auto_renew: updated.auto_renew,
        cancel_at_period_end: !updated.auto_renew && updated.status == "active",
        created_at: updated.created_at.naive_utc(),
    }))
}
