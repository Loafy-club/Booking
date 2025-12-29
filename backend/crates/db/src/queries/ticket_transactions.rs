use crate::models::{TicketTransaction, BonusTicket};
use anyhow::Result;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

/// Create a ticket transaction record
pub async fn create(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    subscription_id: Option<Uuid>,
    booking_id: Option<Uuid>,
    transaction_type: &str,
    amount: i32,
    balance_after: i32,
    notes: Option<&str>,
    admin_id: Option<Uuid>,
) -> Result<TicketTransaction> {
    let transaction = sqlx::query_as::<_, TicketTransaction>(
        r#"
        INSERT INTO ticket_transactions (
            user_id, subscription_id, booking_id, transaction_type,
            amount, balance_after, notes, admin_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(subscription_id)
    .bind(booking_id)
    .bind(transaction_type)
    .bind(amount)
    .bind(balance_after)
    .bind(notes)
    .bind(admin_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(transaction)
}

/// Create a ticket transaction record (non-transaction version for pool)
pub async fn create_with_pool(
    pool: &PgPool,
    user_id: Uuid,
    subscription_id: Option<Uuid>,
    booking_id: Option<Uuid>,
    transaction_type: &str,
    amount: i32,
    balance_after: i32,
    notes: Option<&str>,
    admin_id: Option<Uuid>,
) -> Result<TicketTransaction> {
    let transaction = sqlx::query_as::<_, TicketTransaction>(
        r#"
        INSERT INTO ticket_transactions (
            user_id, subscription_id, booking_id, transaction_type,
            amount, balance_after, notes, admin_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(subscription_id)
    .bind(booking_id)
    .bind(transaction_type)
    .bind(amount)
    .bind(balance_after)
    .bind(notes)
    .bind(admin_id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

/// List user's ticket transactions with pagination
pub async fn list_user_transactions(
    pool: &PgPool,
    user_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<TicketTransaction>, i64)> {
    let offset = (page - 1) * per_page;

    let transactions = sqlx::query_as::<_, TicketTransaction>(
        r#"
        SELECT * FROM ticket_transactions
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(user_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM ticket_transactions WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok((transactions, total.0))
}

/// Create a bonus ticket record
pub async fn create_bonus_ticket(
    pool: &PgPool,
    user_id: Uuid,
    bonus_type: &str,
    tickets: i32,
    note: Option<&str>,
    referrer_id: Option<Uuid>,
    granted_by: Option<Uuid>,
    year: Option<i32>,
) -> Result<BonusTicket> {
    let bonus = sqlx::query_as::<_, BonusTicket>(
        r#"
        INSERT INTO bonus_tickets (
            user_id, bonus_type, tickets, note, referrer_id, granted_by, year
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(bonus_type)
    .bind(tickets)
    .bind(note)
    .bind(referrer_id)
    .bind(granted_by)
    .bind(year)
    .fetch_one(pool)
    .await?;

    Ok(bonus)
}

/// Check if user has received birthday bonus for a specific year
pub async fn has_birthday_bonus_for_year(
    pool: &PgPool,
    user_id: Uuid,
    year: i32,
) -> Result<bool> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM bonus_tickets
        WHERE user_id = $1
          AND bonus_type = 'birthday'
          AND year = $2
        "#
    )
    .bind(user_id)
    .bind(year)
    .fetch_one(pool)
    .await?;

    Ok(count.0 > 0)
}

/// List user's bonus tickets
pub async fn list_user_bonus_tickets(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<BonusTicket>> {
    let bonuses = sqlx::query_as::<_, BonusTicket>(
        r#"
        SELECT * FROM bonus_tickets
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(bonuses)
}
