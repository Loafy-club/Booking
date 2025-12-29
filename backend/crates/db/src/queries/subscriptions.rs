use crate::models::Subscription;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

/// Find subscription by user ID
pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Subscription>> {
    let subscription = sqlx::query_as::<_, Subscription>(
        "SELECT * FROM subscriptions WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(subscription)
}

/// Check if user has an active subscription
pub async fn has_active_subscription(pool: &PgPool, user_id: Uuid) -> Result<bool> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM subscriptions
        WHERE user_id = $1
          AND status = 'active'
        "#
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(count.0 > 0)
}

/// Get active subscription for booking with FOR UPDATE lock
/// CRITICAL: Must be called within a transaction to prevent race conditions
pub async fn get_active_for_booking(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> Result<Option<Subscription>> {
    let subscription = sqlx::query_as::<_, Subscription>(
        r#"
        SELECT * FROM subscriptions
        WHERE user_id = $1
          AND status = 'active'
        FOR UPDATE
        "#
    )
    .bind(user_id)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(subscription)
}

/// Deduct one ticket from subscription atomically
/// Returns the new ticket balance, or error if no tickets available
pub async fn deduct_ticket(
    tx: &mut Transaction<'_, Postgres>,
    subscription_id: Uuid,
) -> Result<i32> {
    let result: (i32,) = sqlx::query_as(
        r#"
        UPDATE subscriptions
        SET tickets_remaining = tickets_remaining - 1,
            updated_at = NOW()
        WHERE id = $1
          AND tickets_remaining > 0
        RETURNING tickets_remaining
        "#
    )
    .bind(subscription_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(result.0)
}

/// Restore one ticket to subscription (for cancellations)
/// Returns the new ticket balance
pub async fn restore_ticket(pool: &PgPool, subscription_id: Uuid) -> Result<i32> {
    let result: (i32,) = sqlx::query_as(
        r#"
        UPDATE subscriptions
        SET tickets_remaining = tickets_remaining + 1,
            updated_at = NOW()
        WHERE id = $1
        RETURNING tickets_remaining
        "#
    )
    .bind(subscription_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}

/// Add bonus tickets to subscription
/// Returns the new ticket balance
pub async fn add_bonus_tickets(pool: &PgPool, subscription_id: Uuid, amount: i32) -> Result<i32> {
    let result: (i32,) = sqlx::query_as(
        r#"
        UPDATE subscriptions
        SET tickets_remaining = tickets_remaining + $2,
            updated_at = NOW()
        WHERE id = $1
        RETURNING tickets_remaining
        "#
    )
    .bind(subscription_id)
    .bind(amount)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}

/// Revoke tickets from subscription (admin action)
/// Returns the new ticket balance
pub async fn revoke_tickets(pool: &PgPool, subscription_id: Uuid, amount: i32) -> Result<i32> {
    let result: (i32,) = sqlx::query_as(
        r#"
        UPDATE subscriptions
        SET tickets_remaining = GREATEST(0, tickets_remaining - $2),
            updated_at = NOW()
        WHERE id = $1
        RETURNING tickets_remaining
        "#
    )
    .bind(subscription_id)
    .bind(amount)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}

/// Create a new subscription record
pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    stripe_subscription_id: &str,
    stripe_customer_id: &str,
    tickets: i32,
    period_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
) -> Result<Subscription> {
    let subscription = sqlx::query_as::<_, Subscription>(
        r#"
        INSERT INTO subscriptions (
            user_id, status, tickets_remaining,
            stripe_subscription_id, stripe_customer_id,
            current_period_start, current_period_end,
            auto_renew
        )
        VALUES ($1, 'active', $2, $3, $4, $5, $6, true)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(tickets)
    .bind(stripe_subscription_id)
    .bind(stripe_customer_id)
    .bind(period_start)
    .bind(period_end)
    .fetch_one(pool)
    .await?;

    Ok(subscription)
}

/// Find subscription by Stripe subscription ID
pub async fn find_by_stripe_subscription_id(
    pool: &PgPool,
    stripe_subscription_id: &str,
) -> Result<Option<Subscription>> {
    let subscription = sqlx::query_as::<_, Subscription>(
        "SELECT * FROM subscriptions WHERE stripe_subscription_id = $1",
    )
    .bind(stripe_subscription_id)
    .fetch_optional(pool)
    .await?;

    Ok(subscription)
}

/// Find subscription by Stripe customer ID
pub async fn find_by_stripe_customer_id(
    pool: &PgPool,
    stripe_customer_id: &str,
) -> Result<Option<Subscription>> {
    let subscription = sqlx::query_as::<_, Subscription>(
        "SELECT * FROM subscriptions WHERE stripe_customer_id = $1",
    )
    .bind(stripe_customer_id)
    .fetch_optional(pool)
    .await?;

    Ok(subscription)
}

/// Update subscription from Stripe webhook data
pub async fn update_from_stripe(
    pool: &PgPool,
    stripe_subscription_id: &str,
    status: &str,
    period_start: Option<DateTime<Utc>>,
    period_end: Option<DateTime<Utc>>,
    cancel_at_period_end: bool,
) -> Result<Subscription> {
    let subscription = sqlx::query_as::<_, Subscription>(
        r#"
        UPDATE subscriptions
        SET status = $2,
            current_period_start = COALESCE($3, current_period_start),
            current_period_end = COALESCE($4, current_period_end),
            auto_renew = NOT $5,
            updated_at = NOW()
        WHERE stripe_subscription_id = $1
        RETURNING *
        "#,
    )
    .bind(stripe_subscription_id)
    .bind(status)
    .bind(period_start)
    .bind(period_end)
    .bind(cancel_at_period_end)
    .fetch_one(pool)
    .await?;

    Ok(subscription)
}

/// Renew subscription by adding tickets and extending period
pub async fn renew_subscription(
    pool: &PgPool,
    subscription_id: Uuid,
    tickets_to_add: i32,
    new_period_end: DateTime<Utc>,
) -> Result<Subscription> {
    let subscription = sqlx::query_as::<_, Subscription>(
        r#"
        UPDATE subscriptions
        SET tickets_remaining = tickets_remaining + $2,
            current_period_end = $3,
            status = 'active',
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(subscription_id)
    .bind(tickets_to_add)
    .bind(new_period_end)
    .fetch_one(pool)
    .await?;

    Ok(subscription)
}

/// Update auto-renew flag
pub async fn update_auto_renew(
    pool: &PgPool,
    subscription_id: Uuid,
    auto_renew: bool,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE subscriptions
        SET auto_renew = $2,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(subscription_id)
    .bind(auto_renew)
    .execute(pool)
    .await?;

    Ok(())
}

/// Mark subscription as cancelled
pub async fn mark_cancelled(pool: &PgPool, stripe_subscription_id: &str) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE subscriptions
        SET status = 'cancelled',
            auto_renew = false,
            updated_at = NOW()
        WHERE stripe_subscription_id = $1
        "#,
    )
    .bind(stripe_subscription_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Mark subscription as expired
pub async fn mark_expired(pool: &PgPool, stripe_subscription_id: &str) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE subscriptions
        SET status = 'expired',
            auto_renew = false,
            updated_at = NOW()
        WHERE stripe_subscription_id = $1
        "#,
    )
    .bind(stripe_subscription_id)
    .execute(pool)
    .await?;

    Ok(())
}
