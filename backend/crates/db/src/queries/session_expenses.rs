use crate::models::SessionExpense;
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

/// Create a new session expense
pub async fn create_expense(
    pool: &PgPool,
    session_id: Uuid,
    category: &str,
    description: Option<&str>,
    cost_type: &str,
    amount_vnd: i32,
) -> Result<SessionExpense> {
    let expense = sqlx::query_as::<_, SessionExpense>(
        r#"
        INSERT INTO session_expenses (session_id, category, description, cost_type, amount_vnd)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(session_id)
    .bind(category)
    .bind(description)
    .bind(cost_type)
    .bind(amount_vnd)
    .fetch_one(pool)
    .await?;

    Ok(expense)
}

/// Create multiple expenses in a transaction
pub async fn create_expenses_batch(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    session_id: Uuid,
    expenses: &[(String, Option<String>, String, i32)], // (category, description, cost_type, amount_vnd)
) -> Result<Vec<SessionExpense>> {
    let mut created = Vec::new();

    for (category, description, cost_type, amount_vnd) in expenses {
        let expense = sqlx::query_as::<_, SessionExpense>(
            r#"
            INSERT INTO session_expenses (session_id, category, description, cost_type, amount_vnd)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(session_id)
        .bind(category)
        .bind(description.as_deref())
        .bind(cost_type)
        .bind(*amount_vnd)
        .fetch_one(&mut **tx)
        .await?;

        created.push(expense);
    }

    Ok(created)
}

/// List all expenses for a session
pub async fn list_expenses_for_session(pool: &PgPool, session_id: Uuid) -> Result<Vec<SessionExpense>> {
    let expenses = sqlx::query_as::<_, SessionExpense>(
        "SELECT * FROM session_expenses WHERE session_id = $1 ORDER BY created_at ASC"
    )
    .bind(session_id)
    .fetch_all(pool)
    .await?;

    Ok(expenses)
}

/// Get total expenses for a session (accounting for per-court multiplier)
pub async fn get_session_total_expenses(pool: &PgPool, session_id: Uuid) -> Result<i64> {
    let result: Option<(Option<i64>,)> = sqlx::query_as(
        r#"
        SELECT SUM(
            CASE
                WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
                ELSE e.amount_vnd
            END
        )
        FROM session_expenses e
        JOIN sessions s ON e.session_id = s.id
        WHERE e.session_id = $1
        "#
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.and_then(|(sum,)| sum).unwrap_or(0))
}

/// Delete all expenses for a session (for replacement)
pub async fn delete_expenses_for_session(pool: &PgPool, session_id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM session_expenses WHERE session_id = $1")
        .bind(session_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Delete a single expense
pub async fn delete_expense(pool: &PgPool, expense_id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM session_expenses WHERE id = $1")
        .bind(expense_id)
        .execute(pool)
        .await?;

    Ok(())
}
