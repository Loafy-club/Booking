use crate::models::Session;
use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use sqlx::{FromRow, PgPool, QueryBuilder, Postgres};
use uuid::Uuid;

/// Participant info from joined booking + user query
#[derive(Debug, Clone, FromRow)]
pub struct SessionParticipant {
    pub user_id: Uuid,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub guest_count: i32,
}

/// List upcoming sessions with optional filters
pub async fn list_sessions(
    pool: &PgPool,
    from_date: Option<NaiveDate>,
    organizer_id: Option<Uuid>,
    only_available: bool,
) -> Result<Vec<Session>> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT * FROM sessions WHERE cancelled = false"
    );

    // Add date filter with parameterized query
    if let Some(date) = from_date {
        query_builder.push(" AND date >= ");
        query_builder.push_bind(date);
    } else {
        // Default: only future sessions
        query_builder.push(" AND date >= CURRENT_DATE");
    }

    // Add organizer filter with parameterized query
    if let Some(org_id) = organizer_id {
        query_builder.push(" AND organizer_id = ");
        query_builder.push_bind(org_id);
    }

    // Add availability filter
    if only_available {
        query_builder.push(" AND available_slots > 0");
    }

    query_builder.push(" ORDER BY date ASC, time ASC");

    let sessions = query_builder
        .build_query_as::<Session>()
        .fetch_all(pool)
        .await?;

    Ok(sessions)
}

/// Get session by ID
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Session>> {
    let session = sqlx::query_as::<_, Session>(
        "SELECT * FROM sessions WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(session)
}

/// Get session by ID with FOR UPDATE lock (for booking creation)
pub async fn find_by_id_for_update(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: Uuid,
) -> Result<Option<Session>> {
    let session = sqlx::query_as::<_, Session>(
        "SELECT * FROM sessions WHERE id = $1 FOR UPDATE"
    )
    .bind(id)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(session)
}

/// Create new session
pub async fn create_session(
    pool: &PgPool,
    organizer_id: Uuid,
    title: &str,
    date: NaiveDate,
    time: NaiveTime,
    location: &str,
    courts: i32,
    max_players_per_court: Option<i32>,
    price_vnd: Option<i32>,
) -> Result<Session> {
    // Calculate total slots
    let max_players = max_players_per_court.unwrap_or(6);
    let total_slots = courts * max_players;

    let session = sqlx::query_as::<_, Session>(
        r#"
        INSERT INTO sessions (
            organizer_id, title, date, time, location, courts,
            max_players_per_court, total_slots, available_slots, price_vnd
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8, $9)
        RETURNING *
        "#
    )
    .bind(organizer_id)
    .bind(title)
    .bind(date)
    .bind(time)
    .bind(location)
    .bind(courts)
    .bind(max_players_per_court)
    .bind(total_slots)
    .bind(price_vnd)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

/// Update session (admin only)
pub async fn update_session(
    pool: &PgPool,
    id: Uuid,
    title: Option<&str>,
    date: Option<NaiveDate>,
    time: Option<NaiveTime>,
    location: Option<&str>,
    courts: Option<i32>,
    max_players_per_court: Option<i32>,
    price_vnd: Option<i32>,
) -> Result<Session> {
    // Get current session to recalculate slots if needed
    let current = find_by_id(pool, id).await?
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    // Calculate new total slots if courts or max_players changed
    let new_courts = courts.unwrap_or(current.courts);
    let new_max_players = max_players_per_court
        .or(current.max_players_per_court)
        .unwrap_or(6);
    let new_total_slots = new_courts * new_max_players;

    // Calculate new available slots (preserve the diff)
    let booked_slots = current.total_slots - current.available_slots;
    let new_available_slots = (new_total_slots - booked_slots).max(0);

    let session = sqlx::query_as::<_, Session>(
        r#"
        UPDATE sessions
        SET title = COALESCE($2, title),
            date = COALESCE($3, date),
            time = COALESCE($4, time),
            location = COALESCE($5, location),
            courts = COALESCE($6, courts),
            max_players_per_court = COALESCE($7, max_players_per_court),
            total_slots = $8,
            available_slots = $9,
            price_vnd = COALESCE($10, price_vnd),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(title)
    .bind(date)
    .bind(time)
    .bind(location)
    .bind(courts)
    .bind(max_players_per_court)
    .bind(new_total_slots)
    .bind(new_available_slots)
    .bind(price_vnd)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

/// Cancel session
pub async fn cancel_session(pool: &PgPool, id: Uuid) -> Result<Session> {
    let session = sqlx::query_as::<_, Session>(
        r#"
        UPDATE sessions
        SET cancelled = true,
            cancelled_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

/// Delete session (admin only)
pub async fn delete_session(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Decrement available slots (atomic)
pub async fn decrement_available_slots(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    session_id: Uuid,
    count: i32,
) -> Result<()> {
    sqlx::query(
        "UPDATE sessions SET available_slots = available_slots - $2 WHERE id = $1"
    )
    .bind(session_id)
    .bind(count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Increment available slots (atomic)
pub async fn increment_available_slots(
    pool: &PgPool,
    session_id: Uuid,
    count: i32,
) -> Result<()> {
    sqlx::query(
        "UPDATE sessions SET available_slots = available_slots + $2 WHERE id = $1"
    )
    .bind(session_id)
    .bind(count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get confirmed participants for a session (paid bookings only)
/// Deduplicates users - if a user has multiple bookings, aggregates their guest counts
pub async fn get_session_participants(
    pool: &PgPool,
    session_id: Uuid,
    limit: Option<i32>,
) -> Result<Vec<SessionParticipant>> {
    let limit_val = limit.unwrap_or(100);

    let participants = sqlx::query_as::<_, SessionParticipant>(
        r#"
        SELECT
            u.id as user_id,
            u.name,
            u.avatar_url,
            COALESCE(SUM(b.guest_count), 0)::int4 as guest_count
        FROM bookings b
        JOIN users u ON u.id = b.user_id
        WHERE b.session_id = $1
          AND b.payment_status = 'confirmed'
          AND b.cancelled_at IS NULL
        GROUP BY u.id, u.name, u.avatar_url
        ORDER BY MIN(b.created_at) ASC
        LIMIT $2
        "#
    )
    .bind(session_id)
    .bind(limit_val)
    .fetch_all(pool)
    .await?;

    Ok(participants)
}

/// Count unique confirmed participants for a session
pub async fn count_session_participants(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT user_id) as count
        FROM bookings
        WHERE session_id = $1
          AND payment_status = 'confirmed'
          AND cancelled_at IS NULL
        "#
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}
