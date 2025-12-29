use crate::models::UserWithRole;
use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;

/// SQL fragment for calculating expense amounts, accounting for per-court multiplier.
///
/// Usage in queries:
/// ```sql
/// SUM({EXPENSE_AMOUNT_CALC})
/// ```
/// where `e` is the session_expenses alias and `s` is the sessions alias.
const EXPENSE_AMOUNT_CALC: &str = r#"
    CASE
        WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
        ELSE e.amount_vnd
    END
"#;

/// Admin statistics for dashboard
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdminStats {
    pub total_users: i64,
    pub total_sessions: i64,
    pub total_bookings: i64,
    pub pending_bookings: i64,
    pub confirmed_bookings: i64,
    pub cancelled_bookings: i64,
    pub total_revenue_vnd: i64,
    pub upcoming_sessions: i64,
    pub new_users: i64,
}

/// Comparison statistics for previous period
#[derive(Debug, Clone, serde::Serialize)]
pub struct PreviousPeriodStats {
    pub new_users: i64,
    pub total_bookings: i64,
    pub total_revenue_vnd: i64,
    pub upcoming_sessions: i64,
}

/// Daily data point for sparkline charts
#[derive(Debug, Clone, serde::Serialize)]
pub struct DailyDataPoint {
    pub date: String,
    pub value: i64,
}

/// Get admin dashboard statistics with optional time period filter
/// `since` - If provided, filters time-based stats to this date onwards
pub async fn get_admin_stats(pool: &PgPool, since: Option<DateTime<Utc>>) -> Result<AdminStats> {
    // Total users (excluding deleted) - always all-time for context
    let (total_users,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await?;

    // New users in period
    let new_users: i64 = if let Some(since_date) = since {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE deleted_at IS NULL AND created_at >= $1"
        )
        .bind(since_date)
        .fetch_one(pool)
        .await?;
        count
    } else {
        total_users
    };

    // Total sessions - always all-time
    let (total_sessions,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sessions"
    )
    .fetch_one(pool)
    .await?;

    // Upcoming sessions - always future sessions
    let (upcoming_sessions,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sessions WHERE date >= CURRENT_DATE AND cancelled = false"
    )
    .fetch_one(pool)
    .await?;

    // Booking counts - filtered by period if provided
    let (total_bookings, pending_bookings, confirmed_bookings, cancelled_bookings, total_revenue_vnd) =
        if let Some(since_date) = since {
            let (total,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM bookings WHERE created_at >= $1"
            )
            .bind(since_date)
            .fetch_one(pool)
            .await?;

            let (pending,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM bookings WHERE payment_status = 'pending' AND cancelled_at IS NULL AND created_at >= $1"
            )
            .bind(since_date)
            .fetch_one(pool)
            .await?;

            let (confirmed,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM bookings WHERE payment_status = 'confirmed' AND created_at >= $1"
            )
            .bind(since_date)
            .fetch_one(pool)
            .await?;

            let (cancelled,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM bookings WHERE cancelled_at IS NOT NULL AND created_at >= $1"
            )
            .bind(since_date)
            .fetch_one(pool)
            .await?;

            let revenue_result: Option<(Option<i64>,)> = sqlx::query_as(
                "SELECT SUM(price_paid_vnd + guest_price_paid_vnd) FROM bookings WHERE payment_status = 'confirmed' AND created_at >= $1"
            )
            .bind(since_date)
            .fetch_optional(pool)
            .await?;

            let revenue = revenue_result.and_then(|(sum,)| sum).unwrap_or(0);

            (total, pending, confirmed, cancelled, revenue)
        } else {
            let (total,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM bookings"
            )
            .fetch_one(pool)
            .await?;

            let (pending,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM bookings WHERE payment_status = 'pending' AND cancelled_at IS NULL"
            )
            .fetch_one(pool)
            .await?;

            let (confirmed,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM bookings WHERE payment_status = 'confirmed'"
            )
            .fetch_one(pool)
            .await?;

            let (cancelled,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM bookings WHERE cancelled_at IS NOT NULL"
            )
            .fetch_one(pool)
            .await?;

            let revenue_result: Option<(Option<i64>,)> = sqlx::query_as(
                "SELECT SUM(price_paid_vnd + guest_price_paid_vnd) FROM bookings WHERE payment_status = 'confirmed'"
            )
            .fetch_optional(pool)
            .await?;

            let revenue = revenue_result.and_then(|(sum,)| sum).unwrap_or(0);

            (total, pending, confirmed, cancelled, revenue)
        };

    Ok(AdminStats {
        total_users,
        total_sessions,
        total_bookings,
        pending_bookings,
        confirmed_bookings,
        cancelled_bookings,
        total_revenue_vnd,
        upcoming_sessions,
        new_users,
    })
}

/// List all users with their roles (admin only)
pub async fn list_all_users(pool: &PgPool) -> Result<Vec<UserWithRole>> {
    let users = sqlx::query_as::<_, UserWithRole>(
        r#"
        SELECT
            u.id,
            u.email,
            u.name,
            u.avatar_url,
            u.phone,
            u.role_id,
            u.auth_provider,
            u.auth_provider_id,
            u.birthday,
            u.created_at as user_created_at,
            u.updated_at as user_updated_at,
            u.deleted_at as user_deleted_at,
            u.suspended_at as user_suspended_at,
            u.suspended_until as user_suspended_until,
            u.suspension_reason as user_suspension_reason,
            u.suspended_by as user_suspended_by,
            r.name as role_name
        FROM users u
        JOIN roles r ON u.role_id = r.id
        WHERE u.deleted_at IS NULL
        ORDER BY u.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

/// Booking with user and session info for admin view
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct BookingWithDetails {
    // Booking fields
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub booking_code: String,
    pub guest_count: i32,
    pub price_paid_vnd: i32,
    pub guest_price_paid_vnd: i32,
    pub payment_method: String,
    pub payment_status: String,
    pub payment_deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    // User info
    pub user_email: String,
    pub user_name: Option<String>,
    // Session info
    pub session_title: String,
    pub session_date: chrono::NaiveDate,
    pub session_time: chrono::NaiveTime,
}

/// List all bookings with user and session details (admin only)
pub async fn list_all_bookings(pool: &PgPool) -> Result<Vec<BookingWithDetails>> {
    let bookings = sqlx::query_as::<_, BookingWithDetails>(
        r#"
        SELECT
            b.id,
            b.user_id,
            b.session_id,
            b.booking_code,
            b.guest_count,
            b.price_paid_vnd,
            b.guest_price_paid_vnd,
            b.payment_method,
            b.payment_status,
            b.payment_deadline,
            b.cancelled_at,
            b.created_at,
            u.email as user_email,
            u.name as user_name,
            s.title as session_title,
            s.date as session_date,
            s.time as session_time
        FROM bookings b
        JOIN users u ON b.user_id = u.id
        JOIN sessions s ON b.session_id = s.id
        ORDER BY b.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(bookings)
}

/// List all available roles
pub async fn list_roles(pool: &PgPool) -> Result<Vec<(uuid::Uuid, String)>> {
    let roles: Vec<(uuid::Uuid, String)> = sqlx::query_as(
        "SELECT id, name FROM roles ORDER BY name"
    )
    .fetch_all(pool)
    .await?;

    Ok(roles)
}

/// Get statistics for the previous period (for comparison)
/// `period_start` - Start of the current period
/// `period_end` - End of the previous period (same as current period start)
/// The previous period has the same duration as the current period
pub async fn get_previous_period_stats(
    pool: &PgPool,
    current_period_start: DateTime<Utc>,
    previous_period_start: DateTime<Utc>,
) -> Result<PreviousPeriodStats> {
    // New users in previous period
    let (new_users,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE deleted_at IS NULL AND created_at >= $1 AND created_at < $2"
    )
    .bind(previous_period_start)
    .bind(current_period_start)
    .fetch_one(pool)
    .await?;

    // Bookings in previous period
    let (total_bookings,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM bookings WHERE created_at >= $1 AND created_at < $2"
    )
    .bind(previous_period_start)
    .bind(current_period_start)
    .fetch_one(pool)
    .await?;

    // Revenue in previous period
    let revenue_result: Option<(Option<i64>,)> = sqlx::query_as(
        "SELECT SUM(price_paid_vnd + guest_price_paid_vnd) FROM bookings WHERE payment_status = 'confirmed' AND created_at >= $1 AND created_at < $2"
    )
    .bind(previous_period_start)
    .bind(current_period_start)
    .fetch_optional(pool)
    .await?;
    let total_revenue_vnd = revenue_result.and_then(|(sum,)| sum).unwrap_or(0);

    // Sessions created in previous period (approximate for "upcoming" comparison)
    let (upcoming_sessions,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sessions WHERE created_at >= $1 AND created_at < $2 AND cancelled = false"
    )
    .bind(previous_period_start)
    .bind(current_period_start)
    .fetch_one(pool)
    .await?;

    Ok(PreviousPeriodStats {
        new_users,
        total_bookings,
        total_revenue_vnd,
        upcoming_sessions,
    })
}

/// Get daily data points for sparkline charts
/// Returns data aggregated by day for the given period
pub async fn get_daily_stats(
    pool: &PgPool,
    since: DateTime<Utc>,
    metric: &str,
) -> Result<Vec<DailyDataPoint>> {
    let query = match metric {
        "users" => {
            r#"
            SELECT DATE(created_at)::text as date, COUNT(*) as value
            FROM users
            WHERE deleted_at IS NULL AND created_at >= $1
            GROUP BY DATE(created_at)
            ORDER BY DATE(created_at)
            "#
        }
        "bookings" => {
            r#"
            SELECT DATE(created_at)::text as date, COUNT(*) as value
            FROM bookings
            WHERE created_at >= $1
            GROUP BY DATE(created_at)
            ORDER BY DATE(created_at)
            "#
        }
        "revenue" => {
            r#"
            SELECT DATE(created_at)::text as date, COALESCE(SUM(price_paid_vnd + guest_price_paid_vnd), 0) as value
            FROM bookings
            WHERE payment_status = 'confirmed' AND created_at >= $1
            GROUP BY DATE(created_at)
            ORDER BY DATE(created_at)
            "#
        }
        "sessions" => {
            r#"
            SELECT DATE(date)::text as date, COUNT(*) as value
            FROM sessions
            WHERE date >= DATE($1) AND cancelled = false
            GROUP BY DATE(date)
            ORDER BY DATE(date)
            "#
        }
        _ => return Ok(vec![]),
    };

    let rows: Vec<(String, i64)> = sqlx::query_as(query)
        .bind(since)
        .fetch_all(pool)
        .await?;

    Ok(rows
        .into_iter()
        .map(|(date, value)| DailyDataPoint { date, value })
        .collect())
}

// =============================================================================
// Profit & Expense Related Queries
// =============================================================================

/// Profit statistics for dashboard
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProfitStats {
    pub total_revenue_vnd: i64,
    pub total_expenses_vnd: i64,
    pub net_profit_vnd: i64,
    pub profit_margin_percent: f64,
}

/// Per-session profit summary
#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionProfitSummary {
    pub session_id: uuid::Uuid,
    pub title: String,
    pub date: NaiveDate,
    pub revenue_vnd: i64,
    pub expenses_vnd: i64,
    pub profit_vnd: i64,
    pub profit_margin_percent: f64,
}

/// Expense breakdown by category
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExpenseByCategory {
    pub category: String,
    pub total_vnd: i64,
    pub percentage: f64,
}

/// Daily profit data point for charts
#[derive(Debug, Clone, serde::Serialize)]
pub struct DailyProfitDataPoint {
    pub date: String,
    pub revenue: i64,
    pub expenses: i64,
    pub profit: i64,
}

/// Get profit statistics for a period
pub async fn get_profit_stats(pool: &PgPool, since: Option<DateTime<Utc>>) -> Result<ProfitStats> {
    // Get total revenue from confirmed bookings
    let revenue_result: Option<(Option<i64>,)> = if let Some(since_date) = since {
        sqlx::query_as(
            r#"
            SELECT SUM(price_paid_vnd + guest_price_paid_vnd)
            FROM bookings
            WHERE payment_status = 'confirmed' AND created_at >= $1
            "#
        )
        .bind(since_date)
        .fetch_optional(pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT SUM(price_paid_vnd + guest_price_paid_vnd) FROM bookings WHERE payment_status = 'confirmed'"
        )
        .fetch_optional(pool)
        .await?
    };
    let total_revenue_vnd = revenue_result.and_then(|(sum,)| sum).unwrap_or(0);

    // Get total expenses (accounting for per-court multiplier)
    let expenses_result: Option<(Option<i64>,)> = if let Some(since_date) = since {
        sqlx::query_as(
            r#"
            SELECT SUM(
                CASE
                    WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
                    ELSE e.amount_vnd
                END
            )
            FROM session_expenses e
            JOIN sessions s ON e.session_id = s.id
            WHERE s.date >= DATE($1)
            "#
        )
        .bind(since_date)
        .fetch_optional(pool)
        .await?
    } else {
        sqlx::query_as(
            r#"
            SELECT SUM(
                CASE
                    WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
                    ELSE e.amount_vnd
                END
            )
            FROM session_expenses e
            JOIN sessions s ON e.session_id = s.id
            "#
        )
        .fetch_optional(pool)
        .await?
    };
    let total_expenses_vnd = expenses_result.and_then(|(sum,)| sum).unwrap_or(0);

    let net_profit_vnd = total_revenue_vnd - total_expenses_vnd;
    let profit_margin_percent = if total_revenue_vnd > 0 {
        (net_profit_vnd as f64 / total_revenue_vnd as f64) * 100.0
    } else {
        0.0
    };

    Ok(ProfitStats {
        total_revenue_vnd,
        total_expenses_vnd,
        net_profit_vnd,
        profit_margin_percent,
    })
}

/// Get per-session profit breakdown
pub async fn get_sessions_profit(
    pool: &PgPool,
    since: Option<DateTime<Utc>>,
    limit: i32,
) -> Result<Vec<SessionProfitSummary>> {
    let rows: Vec<(uuid::Uuid, String, NaiveDate, Option<i64>, Option<i64>)> = if let Some(since_date) = since {
        sqlx::query_as(
            r#"
            SELECT
                s.id,
                s.title,
                s.date,
                (
                    SELECT COALESCE(SUM(b.price_paid_vnd + b.guest_price_paid_vnd), 0)
                    FROM bookings b
                    WHERE b.session_id = s.id AND b.payment_status = 'confirmed'
                ) as revenue,
                (
                    SELECT COALESCE(SUM(
                        CASE
                            WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
                            ELSE e.amount_vnd
                        END
                    ), 0)
                    FROM session_expenses e
                    WHERE e.session_id = s.id
                ) as expenses
            FROM sessions s
            WHERE s.date >= DATE($1) AND s.cancelled = false
            ORDER BY s.date DESC
            LIMIT $2
            "#
        )
        .bind(since_date)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as(
            r#"
            SELECT
                s.id,
                s.title,
                s.date,
                (
                    SELECT COALESCE(SUM(b.price_paid_vnd + b.guest_price_paid_vnd), 0)
                    FROM bookings b
                    WHERE b.session_id = s.id AND b.payment_status = 'confirmed'
                ) as revenue,
                (
                    SELECT COALESCE(SUM(
                        CASE
                            WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
                            ELSE e.amount_vnd
                        END
                    ), 0)
                    FROM session_expenses e
                    WHERE e.session_id = s.id
                ) as expenses
            FROM sessions s
            WHERE s.cancelled = false
            ORDER BY s.date DESC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    };

    let summaries = rows
        .into_iter()
        .map(|(session_id, title, date, revenue, expenses)| {
            let revenue_vnd = revenue.unwrap_or(0);
            let expenses_vnd = expenses.unwrap_or(0);
            let profit_vnd = revenue_vnd - expenses_vnd;
            let profit_margin_percent = if revenue_vnd > 0 {
                (profit_vnd as f64 / revenue_vnd as f64) * 100.0
            } else if expenses_vnd > 0 {
                -100.0 // All expenses, no revenue
            } else {
                0.0
            };

            SessionProfitSummary {
                session_id,
                title,
                date,
                revenue_vnd,
                expenses_vnd,
                profit_vnd,
                profit_margin_percent,
            }
        })
        .collect();

    Ok(summaries)
}

/// Get expense breakdown by category
pub async fn get_expenses_by_category(
    pool: &PgPool,
    since: Option<DateTime<Utc>>,
) -> Result<Vec<ExpenseByCategory>> {
    let rows: Vec<(String, Option<i64>)> = if let Some(since_date) = since {
        sqlx::query_as(
            r#"
            SELECT
                e.category,
                SUM(
                    CASE
                        WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
                        ELSE e.amount_vnd
                    END
                ) as total
            FROM session_expenses e
            JOIN sessions s ON e.session_id = s.id
            WHERE s.date >= DATE($1)
            GROUP BY e.category
            ORDER BY total DESC
            "#
        )
        .bind(since_date)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as(
            r#"
            SELECT
                e.category,
                SUM(
                    CASE
                        WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
                        ELSE e.amount_vnd
                    END
                ) as total
            FROM session_expenses e
            JOIN sessions s ON e.session_id = s.id
            GROUP BY e.category
            ORDER BY total DESC
            "#
        )
        .fetch_all(pool)
        .await?
    };

    // Calculate total for percentages
    let grand_total: i64 = rows.iter().map(|(_, t)| t.unwrap_or(0)).sum();

    let categories = rows
        .into_iter()
        .map(|(category, total)| {
            let total_vnd = total.unwrap_or(0);
            let percentage = if grand_total > 0 {
                (total_vnd as f64 / grand_total as f64) * 100.0
            } else {
                0.0
            };
            ExpenseByCategory {
                category,
                total_vnd,
                percentage,
            }
        })
        .collect();

    Ok(categories)
}

/// Get daily profit data for trend charts
pub async fn get_daily_profit_data(
    pool: &PgPool,
    since: DateTime<Utc>,
) -> Result<Vec<DailyProfitDataPoint>> {
    // Get daily revenue
    let revenue_rows: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT DATE(created_at)::text as date, COALESCE(SUM(price_paid_vnd + guest_price_paid_vnd), 0) as value
        FROM bookings
        WHERE payment_status = 'confirmed' AND created_at >= $1
        GROUP BY DATE(created_at)
        ORDER BY DATE(created_at)
        "#
    )
    .bind(since)
    .fetch_all(pool)
    .await?;

    // Get daily expenses (based on session date)
    let expense_rows: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT s.date::text as date, COALESCE(SUM(
            CASE
                WHEN e.cost_type = 'per_court' THEN e.amount_vnd * s.courts
                ELSE e.amount_vnd
            END
        ), 0) as value
        FROM session_expenses e
        JOIN sessions s ON e.session_id = s.id
        WHERE s.date >= DATE($1)
        GROUP BY s.date
        ORDER BY s.date
        "#
    )
    .bind(since)
    .fetch_all(pool)
    .await?;

    // Merge revenue and expense data by date
    use std::collections::HashMap;
    let mut data_by_date: HashMap<String, (i64, i64)> = HashMap::new();

    for (date, revenue) in revenue_rows {
        data_by_date.entry(date).or_insert((0, 0)).0 = revenue;
    }

    for (date, expenses) in expense_rows {
        data_by_date.entry(date).or_insert((0, 0)).1 = expenses;
    }

    // Sort by date and convert to result
    let mut dates: Vec<_> = data_by_date.into_iter().collect();
    dates.sort_by(|a, b| a.0.cmp(&b.0));

    let result = dates
        .into_iter()
        .map(|(date, (revenue, expenses))| DailyProfitDataPoint {
            date,
            revenue,
            expenses,
            profit: revenue - expenses,
        })
        .collect();

    Ok(result)
}

// =============================================================================
// Paginated Query Functions
// =============================================================================

/// Parameters for paginated users query
pub struct UsersQueryParams {
    pub page: i32,
    pub per_page: i32,
    pub search: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>, // "active", "suspended"
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// List users with pagination, filtering, and sorting
pub async fn list_users_paginated(
    pool: &PgPool,
    params: UsersQueryParams,
) -> Result<(Vec<UserWithRole>, i64)> {
    let offset = (params.page - 1) * params.per_page;

    // Build WHERE clauses
    let mut conditions = vec!["u.deleted_at IS NULL".to_string()];
    let mut bind_idx = 1;

    if params.search.is_some() {
        conditions.push(format!(
            "(u.name ILIKE '%' || ${} || '%' OR u.email ILIKE '%' || ${} || '%')",
            bind_idx,
            bind_idx
        ));
        bind_idx += 1;
    }

    if params.role.is_some() {
        conditions.push(format!("r.name = ${}", bind_idx));
        bind_idx += 1;
    }

    if let Some(ref status) = params.status {
        match status.as_str() {
            "active" => conditions.push("u.suspended_at IS NULL".to_string()),
            "suspended" => conditions.push("u.suspended_at IS NOT NULL AND (u.suspended_until IS NULL OR u.suspended_until > NOW())".to_string()),
            _ => {}
        }
    }

    let where_clause = conditions.join(" AND ");

    // Build ORDER BY clause
    let order_column = match params.sort_by.as_deref() {
        Some("name") => "u.name",
        Some("email") => "u.email",
        Some("role") => "r.name",
        Some("created_at") | _ => "u.created_at",
    };
    let order_dir = match params.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Count query
    let count_query = format!(
        r#"
        SELECT COUNT(*)
        FROM users u
        JOIN roles r ON u.role_id = r.id
        WHERE {}
        "#,
        where_clause
    );

    // Data query
    let data_query = format!(
        r#"
        SELECT
            u.id,
            u.email,
            u.name,
            u.avatar_url,
            u.phone,
            u.role_id,
            u.auth_provider,
            u.auth_provider_id,
            u.birthday,
            u.created_at as user_created_at,
            u.updated_at as user_updated_at,
            u.deleted_at as user_deleted_at,
            u.suspended_at as user_suspended_at,
            u.suspended_until as user_suspended_until,
            u.suspension_reason as user_suspension_reason,
            u.suspended_by as user_suspended_by,
            r.name as role_name
        FROM users u
        JOIN roles r ON u.role_id = r.id
        WHERE {}
        ORDER BY {} {} NULLS LAST
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        order_column,
        order_dir,
        bind_idx,
        bind_idx + 1
    );

    // Execute count query
    let mut count_builder = sqlx::query_scalar::<_, i64>(&count_query);
    if let Some(ref search) = params.search {
        count_builder = count_builder.bind(search);
    }
    if let Some(ref role) = params.role {
        count_builder = count_builder.bind(role);
    }
    let total: i64 = count_builder.fetch_one(pool).await?;

    // Execute data query
    let mut data_builder = sqlx::query_as::<_, UserWithRole>(&data_query);
    if let Some(ref search) = params.search {
        data_builder = data_builder.bind(search);
    }
    if let Some(ref role) = params.role {
        data_builder = data_builder.bind(role);
    }
    data_builder = data_builder.bind(params.per_page).bind(offset);
    let users = data_builder.fetch_all(pool).await?;

    Ok((users, total))
}

/// Parameters for paginated bookings query
pub struct BookingsQueryParams {
    pub page: i32,
    pub per_page: i32,
    pub search: Option<String>,
    pub payment_status: Option<String>,
    pub session_id: Option<uuid::Uuid>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// List bookings with pagination, filtering, and sorting
pub async fn list_bookings_paginated(
    pool: &PgPool,
    params: BookingsQueryParams,
) -> Result<(Vec<BookingWithDetails>, i64)> {
    let offset = (params.page - 1) * params.per_page;

    // Build WHERE clauses
    let mut conditions = vec!["1=1".to_string()];
    let mut bind_idx = 1;

    if params.search.is_some() {
        conditions.push(format!(
            "(b.booking_code ILIKE '%' || ${} || '%' OR u.name ILIKE '%' || ${} || '%' OR u.email ILIKE '%' || ${} || '%')",
            bind_idx, bind_idx, bind_idx
        ));
        bind_idx += 1;
    }

    if params.payment_status.is_some() {
        conditions.push(format!("b.payment_status = ${}", bind_idx));
        bind_idx += 1;
    }

    if params.session_id.is_some() {
        conditions.push(format!("b.session_id = ${}", bind_idx));
        bind_idx += 1;
    }

    let where_clause = conditions.join(" AND ");

    // Build ORDER BY clause
    let order_column = match params.sort_by.as_deref() {
        Some("booking_code") => "b.booking_code",
        Some("user") => "u.name",
        Some("session") => "s.date",
        Some("amount") => "(b.price_paid_vnd + b.guest_price_paid_vnd)",
        Some("status") => "b.payment_status",
        Some("created_at") | _ => "b.created_at",
    };
    let order_dir = match params.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Count query
    let count_query = format!(
        r#"
        SELECT COUNT(*)
        FROM bookings b
        JOIN users u ON b.user_id = u.id
        JOIN sessions s ON b.session_id = s.id
        WHERE {}
        "#,
        where_clause
    );

    // Data query
    let data_query = format!(
        r#"
        SELECT
            b.id,
            b.user_id,
            b.session_id,
            b.booking_code,
            b.guest_count,
            b.price_paid_vnd,
            b.guest_price_paid_vnd,
            b.payment_method,
            b.payment_status,
            b.payment_deadline,
            b.cancelled_at,
            b.created_at,
            u.email as user_email,
            u.name as user_name,
            s.title as session_title,
            s.date as session_date,
            s.time as session_time
        FROM bookings b
        JOIN users u ON b.user_id = u.id
        JOIN sessions s ON b.session_id = s.id
        WHERE {}
        ORDER BY {} {} NULLS LAST
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        order_column,
        order_dir,
        bind_idx,
        bind_idx + 1
    );

    // Execute count query
    let mut count_builder = sqlx::query_scalar::<_, i64>(&count_query);
    if let Some(ref search) = params.search {
        count_builder = count_builder.bind(search);
    }
    if let Some(ref status) = params.payment_status {
        count_builder = count_builder.bind(status);
    }
    if let Some(session_id) = params.session_id {
        count_builder = count_builder.bind(session_id);
    }
    let total: i64 = count_builder.fetch_one(pool).await?;

    // Execute data query
    let mut data_builder = sqlx::query_as::<_, BookingWithDetails>(&data_query);
    if let Some(ref search) = params.search {
        data_builder = data_builder.bind(search);
    }
    if let Some(ref status) = params.payment_status {
        data_builder = data_builder.bind(status);
    }
    if let Some(session_id) = params.session_id {
        data_builder = data_builder.bind(session_id);
    }
    data_builder = data_builder.bind(params.per_page).bind(offset);
    let bookings = data_builder.fetch_all(pool).await?;

    Ok((bookings, total))
}

/// Session with organizer info for admin view
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct SessionWithOrganizer {
    pub id: uuid::Uuid,
    pub organizer_id: uuid::Uuid,
    pub organizer_name: Option<String>,
    pub title: String,
    pub date: NaiveDate,
    pub time: chrono::NaiveTime,
    pub end_time: Option<chrono::NaiveTime>,
    pub location: String,
    pub courts: i32,
    pub total_slots: i32,
    pub available_slots: i32,
    pub price_vnd: Option<i32>,
    pub cancelled: bool,
    pub created_at: DateTime<Utc>,
}

/// Parameters for paginated sessions query
pub struct SessionsQueryParams {
    pub page: i32,
    pub per_page: i32,
    pub search: Option<String>,
    pub status: Option<String>, // "upcoming", "past", "cancelled"
    pub organizer_id: Option<uuid::Uuid>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// List sessions with pagination, filtering, and sorting (admin only)
pub async fn list_sessions_paginated(
    pool: &PgPool,
    params: SessionsQueryParams,
) -> Result<(Vec<SessionWithOrganizer>, i64)> {
    let offset = (params.page - 1) * params.per_page;

    // Build WHERE clauses
    let mut conditions = vec!["1=1".to_string()];
    let mut bind_idx = 1;

    if params.search.is_some() {
        conditions.push(format!(
            "(s.title ILIKE '%' || ${} || '%' OR s.location ILIKE '%' || ${} || '%')",
            bind_idx, bind_idx
        ));
        bind_idx += 1;
    }

    if let Some(ref status) = params.status {
        match status.as_str() {
            "upcoming" => conditions.push("s.date >= CURRENT_DATE AND s.cancelled = false".to_string()),
            "past" => conditions.push("s.date < CURRENT_DATE AND s.cancelled = false".to_string()),
            "cancelled" => conditions.push("s.cancelled = true".to_string()),
            _ => {}
        }
    }

    if params.organizer_id.is_some() {
        conditions.push(format!("s.organizer_id = ${}", bind_idx));
        bind_idx += 1;
    }

    let where_clause = conditions.join(" AND ");

    // Build ORDER BY clause
    let order_column = match params.sort_by.as_deref() {
        Some("title") => "s.title",
        Some("location") => "s.location",
        Some("slots") => "s.available_slots",
        Some("price") => "s.price_vnd",
        Some("date") | _ => "s.date",
    };
    let order_dir = match params.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Count query
    let count_query = format!(
        r#"
        SELECT COUNT(*)
        FROM sessions s
        JOIN users u ON s.organizer_id = u.id
        WHERE {}
        "#,
        where_clause
    );

    // Data query
    let data_query = format!(
        r#"
        SELECT
            s.id,
            s.organizer_id,
            u.name as organizer_name,
            s.title,
            s.date,
            s.time,
            s.end_time,
            s.location,
            s.courts,
            s.total_slots,
            s.available_slots,
            s.price_vnd,
            s.cancelled,
            s.created_at
        FROM sessions s
        JOIN users u ON s.organizer_id = u.id
        WHERE {}
        ORDER BY {} {} NULLS LAST
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        order_column,
        order_dir,
        bind_idx,
        bind_idx + 1
    );

    // Execute count query
    let mut count_builder = sqlx::query_scalar::<_, i64>(&count_query);
    if let Some(ref search) = params.search {
        count_builder = count_builder.bind(search);
    }
    if let Some(organizer_id) = params.organizer_id {
        count_builder = count_builder.bind(organizer_id);
    }
    let total: i64 = count_builder.fetch_one(pool).await?;

    // Execute data query
    let mut data_builder = sqlx::query_as::<_, SessionWithOrganizer>(&data_query);
    if let Some(ref search) = params.search {
        data_builder = data_builder.bind(search);
    }
    if let Some(organizer_id) = params.organizer_id {
        data_builder = data_builder.bind(organizer_id);
    }
    data_builder = data_builder.bind(params.per_page).bind(offset);
    let sessions = data_builder.fetch_all(pool).await?;

    Ok((sessions, total))
}

// =============================================================================
// Admin Booking Edit Operations
// =============================================================================

/// Get a single booking by ID with full details (for admin edit)
pub async fn get_booking_by_id(pool: &PgPool, booking_id: uuid::Uuid) -> Result<Option<BookingWithDetails>> {
    let booking = sqlx::query_as::<_, BookingWithDetails>(
        r#"
        SELECT
            b.id,
            b.user_id,
            b.session_id,
            b.booking_code,
            b.guest_count,
            b.price_paid_vnd,
            b.guest_price_paid_vnd,
            b.payment_method,
            b.payment_status,
            b.payment_deadline,
            b.cancelled_at,
            b.created_at,
            u.email as user_email,
            u.name as user_name,
            s.title as session_title,
            s.date as session_date,
            s.time as session_time
        FROM bookings b
        JOIN users u ON b.user_id = u.id
        JOIN sessions s ON b.session_id = s.id
        WHERE b.id = $1
        "#
    )
    .bind(booking_id)
    .fetch_optional(pool)
    .await?;

    Ok(booking)
}

/// Parameters for updating a booking (admin only)
pub struct UpdateBookingParams {
    pub guest_count: Option<i32>,
    pub price_paid_vnd: Option<i32>,
    pub guest_price_paid_vnd: Option<i32>,
    pub payment_method: Option<String>,
    pub payment_status: Option<String>,
}

// =============================================================================
// Admin User Edit Operations
// =============================================================================

/// Parameters for updating a user (admin only)
pub struct UpdateUserParams {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
}

/// Update a user (admin only)
/// Can update name, phone, and role
pub async fn update_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
    params: UpdateUserParams,
) -> Result<UserWithRole> {
    let mut tx = pool.begin().await?;

    // If role is being updated, get the role ID first
    if let Some(ref role_name) = params.role {
        let role_id: Option<(uuid::Uuid,)> = sqlx::query_as(
            "SELECT id FROM roles WHERE name = $1"
        )
        .bind(role_name)
        .fetch_optional(&mut *tx)
        .await?;

        if role_id.is_none() {
            return Err(anyhow::anyhow!("Role '{}' not found", role_name));
        }

        let (role_id,) = role_id.unwrap();
        sqlx::query(
            "UPDATE users SET role_id = $2, updated_at = NOW() WHERE id = $1"
        )
        .bind(user_id)
        .bind(role_id)
        .execute(&mut *tx)
        .await?;
    }

    // Update other fields if provided
    if params.name.is_some() || params.phone.is_some() {
        // Build dynamic update query
        let mut set_clauses = vec!["updated_at = NOW()".to_string()];
        let mut bind_idx = 2;

        if params.name.is_some() {
            set_clauses.push(format!("name = ${}", bind_idx));
            bind_idx += 1;
        }
        if params.phone.is_some() {
            set_clauses.push(format!("phone = ${}", bind_idx));
        }

        let update_query = format!(
            "UPDATE users SET {} WHERE id = $1",
            set_clauses.join(", ")
        );

        let mut query = sqlx::query(&update_query).bind(user_id);
        if let Some(ref name) = params.name {
            query = query.bind(name);
        }
        if let Some(ref phone) = params.phone {
            query = query.bind(phone);
        }

        query.execute(&mut *tx).await?;
    }

    tx.commit().await?;

    // Fetch and return updated user with role
    let query = format!(
        r#"
        SELECT
            u.id,
            u.email,
            u.name,
            u.avatar_url,
            u.phone,
            u.role_id,
            u.auth_provider,
            u.auth_provider_id,
            u.birthday,
            u.created_at as user_created_at,
            u.updated_at as user_updated_at,
            u.deleted_at as user_deleted_at,
            u.suspended_at as user_suspended_at,
            u.suspended_until as user_suspended_until,
            u.suspension_reason as user_suspension_reason,
            u.suspended_by as user_suspended_by,
            r.name as role_name
        FROM users u
        JOIN roles r ON u.role_id = r.id
        WHERE u.id = $1
        "#
    );

    let user = sqlx::query_as::<_, UserWithRole>(&query)
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    Ok(user)
}

/// Delete a user (admin only) - soft delete
/// Cancels pending bookings, handles sessions, and marks user as deleted
pub async fn delete_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    // Check if user exists and is not already deleted
    let user_exists: Option<(uuid::Uuid,)> = sqlx::query_as(
        "SELECT id FROM users WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;

    if user_exists.is_none() {
        return Err(anyhow::anyhow!("User not found or already deleted"));
    }

    // Cancel any active sessions owned by this user
    sqlx::query(
        "UPDATE sessions SET cancelled = true WHERE organizer_id = $1 AND cancelled = false"
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    // Cancel pending bookings and release slots back to sessions
    let pending_bookings: Vec<(uuid::Uuid, i32, uuid::Uuid)> = sqlx::query_as(
        r#"
        SELECT id, guest_count, session_id
        FROM bookings
        WHERE user_id = $1 AND payment_status = 'pending' AND cancelled_at IS NULL
        "#
    )
    .bind(user_id)
    .fetch_all(&mut *tx)
    .await?;

    for (_, guest_count, session_id) in &pending_bookings {
        let slots_to_release = 1 + guest_count;
        sqlx::query(
            "UPDATE sessions SET available_slots = available_slots + $1 WHERE id = $2"
        )
        .bind(slots_to_release)
        .bind(session_id)
        .execute(&mut *tx)
        .await?;
    }

    // Cancel pending bookings
    sqlx::query(
        r#"
        UPDATE bookings
        SET payment_status = 'cancelled', cancelled_at = NOW()
        WHERE user_id = $1 AND payment_status = 'pending'
        "#
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    // Soft delete the user: set deleted_at and clear PII
    sqlx::query(
        r#"
        UPDATE users
        SET deleted_at = NOW(),
            name = NULL,
            avatar_url = NULL,
            phone = NULL,
            updated_at = NOW()
        WHERE id = $1
        "#
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

// =============================================================================
// Admin Booking Edit Operations (existing)
// =============================================================================

/// Update a booking (admin only)
/// Handles slot availability changes when guest_count is modified
/// Returns the updated booking or error
pub async fn update_booking(
    pool: &PgPool,
    booking_id: uuid::Uuid,
    params: UpdateBookingParams,
) -> Result<BookingWithDetails> {
    // Start a transaction for atomic updates
    let mut tx = pool.begin().await?;

    // Get current booking with lock
    let current: (i32, uuid::Uuid) = sqlx::query_as(
        "SELECT guest_count, session_id FROM bookings WHERE id = $1 FOR UPDATE"
    )
    .bind(booking_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| anyhow::anyhow!("Booking not found"))?;

    let (current_guest_count, session_id) = current;

    // If guest_count is being changed, update session available_slots
    if let Some(new_guest_count) = params.guest_count {
        let slot_diff = current_guest_count - new_guest_count;
        // slot_diff positive = returning slots, negative = taking more slots

        if slot_diff != 0 {
            // Lock session row and check availability
            let (available_slots,): (i32,) = sqlx::query_as(
                "SELECT available_slots FROM sessions WHERE id = $1 FOR UPDATE"
            )
            .bind(session_id)
            .fetch_one(&mut *tx)
            .await?;

            // If taking more slots (negative diff), check availability
            if slot_diff < 0 && available_slots < -slot_diff {
                return Err(anyhow::anyhow!(
                    "Not enough available slots. Need {} more but only {} available",
                    -slot_diff,
                    available_slots
                ));
            }

            // Update session available_slots (add back or subtract)
            // +1 slot added back for each reduction in guests
            // -1 slot removed for each additional guest
            sqlx::query(
                "UPDATE sessions SET available_slots = available_slots + $1 WHERE id = $2"
            )
            .bind(slot_diff)
            .bind(session_id)
            .execute(&mut *tx)
            .await?;
        }
    }

    // Build dynamic UPDATE query
    let mut set_clauses = vec!["updated_at = NOW()".to_string()];
    let mut bind_idx = 2; // $1 is booking_id

    if params.guest_count.is_some() {
        set_clauses.push(format!("guest_count = ${}", bind_idx));
        bind_idx += 1;
    }
    if params.price_paid_vnd.is_some() {
        set_clauses.push(format!("price_paid_vnd = ${}", bind_idx));
        bind_idx += 1;
    }
    if params.guest_price_paid_vnd.is_some() {
        set_clauses.push(format!("guest_price_paid_vnd = ${}", bind_idx));
        bind_idx += 1;
    }
    if params.payment_method.is_some() {
        set_clauses.push(format!("payment_method = ${}", bind_idx));
        bind_idx += 1;
    }
    if params.payment_status.is_some() {
        set_clauses.push(format!("payment_status = ${}", bind_idx));
    }

    let update_query = format!(
        "UPDATE bookings SET {} WHERE id = $1",
        set_clauses.join(", ")
    );

    // Build and execute the update
    let mut query = sqlx::query(&update_query).bind(booking_id);

    if let Some(guest_count) = params.guest_count {
        query = query.bind(guest_count);
    }
    if let Some(price_paid_vnd) = params.price_paid_vnd {
        query = query.bind(price_paid_vnd);
    }
    if let Some(guest_price_paid_vnd) = params.guest_price_paid_vnd {
        query = query.bind(guest_price_paid_vnd);
    }
    if let Some(ref payment_method) = params.payment_method {
        query = query.bind(payment_method);
    }
    if let Some(ref payment_status) = params.payment_status {
        query = query.bind(payment_status);
    }

    query.execute(&mut *tx).await?;

    // Commit transaction
    tx.commit().await?;

    // Fetch and return updated booking
    get_booking_by_id(pool, booking_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch updated booking"))
}
