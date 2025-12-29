use anyhow::Result;
use sqlx::{PgPool, Postgres, Transaction};

/// Get config value by key
pub async fn get_value(pool: &PgPool, key: &str) -> Result<Option<String>> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = $1"
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|(v,)| v))
}

/// Get config value by key within a transaction
pub async fn get_value_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    key: &str,
) -> Result<Option<String>> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = $1"
    )
    .bind(key)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(result.map(|(v,)| v))
}

/// Get subscriber out-of-ticket discount percentage
/// Returns the discount percentage (e.g., 10 for 10%)
pub async fn get_out_of_ticket_discount(tx: &mut Transaction<'_, Postgres>) -> Result<i32> {
    let value = get_value_in_tx(tx, "subscriber_out_of_ticket_discount_percent")
        .await?
        .unwrap_or_else(|| "10".to_string());
    Ok(value.parse().unwrap_or(10))
}

/// Get referral bonus tickets amount
pub async fn get_referral_bonus_tickets(pool: &PgPool) -> Result<i32> {
    let value = get_value(pool, "referral_bonus_tickets")
        .await?
        .unwrap_or_else(|| "1".to_string());
    Ok(value.parse().unwrap_or(1))
}

/// Get birthday bonus tickets amount
pub async fn get_birthday_bonus_tickets(pool: &PgPool) -> Result<i32> {
    let value = get_value(pool, "birthday_bonus_tickets")
        .await?
        .unwrap_or_else(|| "1".to_string());
    Ok(value.parse().unwrap_or(1))
}

/// Get birthday account age requirement in days
pub async fn get_birthday_account_age_days(pool: &PgPool) -> Result<i32> {
    let value = get_value(pool, "birthday_account_age_days")
        .await?
        .unwrap_or_else(|| "30".to_string());
    Ok(value.parse().unwrap_or(30))
}
