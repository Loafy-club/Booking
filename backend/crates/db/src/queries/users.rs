use crate::models::{Role, User, UserWithRole};
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Base SQL query for selecting user with role.
/// Reused across multiple query functions to avoid duplication.
const USER_WITH_ROLE_SELECT: &str = r#"
    SELECT
        u.id,
        u.email,
        u.name,
        u.avatar_url,
        u.phone,
        u.role_id,
        u.auth_provider,
        u.auth_provider_id,
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
"#;

/// Find user by email
pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Find user by ID
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Find user with role by ID
pub async fn find_with_role_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserWithRole>> {
    let query = format!("{} WHERE u.id = $1", USER_WITH_ROLE_SELECT);
    let user = sqlx::query_as::<_, UserWithRole>(&query)
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Find user with role by auth provider ID (Supabase user ID)
/// Excludes soft-deleted users
pub async fn find_with_role_by_auth_provider_id(
    pool: &PgPool,
    auth_provider_id: &str,
) -> Result<Option<UserWithRole>> {
    let query = format!("{} WHERE u.auth_provider_id = $1 AND u.deleted_at IS NULL", USER_WITH_ROLE_SELECT);
    let user = sqlx::query_as::<_, UserWithRole>(&query)
        .bind(auth_provider_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Find user with role by email
pub async fn find_with_role_by_email(pool: &PgPool, email: &str) -> Result<Option<UserWithRole>> {
    let query = format!("{} WHERE u.email = $1", USER_WITH_ROLE_SELECT);
    let user = sqlx::query_as::<_, UserWithRole>(&query)
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Get role by name
pub async fn get_role_by_name(pool: &PgPool, name: &str) -> Result<Option<Role>> {
    let role = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles WHERE name = $1"
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    Ok(role)
}

/// Create new user
pub async fn create_user(
    pool: &PgPool,
    email: &str,
    name: Option<&str>,
    avatar_url: Option<&str>,
    auth_provider: &str,
    auth_provider_id: &str,
    role_name: &str,
) -> Result<User> {
    // Get role ID
    let role = get_role_by_name(pool, role_name)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Role '{}' not found", role_name))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, name, avatar_url, role_id, auth_provider, auth_provider_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(email)
    .bind(name)
    .bind(avatar_url)
    .bind(role.id)
    .bind(auth_provider)
    .bind(auth_provider_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Update user
pub async fn update_user(
    pool: &PgPool,
    id: Uuid,
    name: Option<&str>,
    avatar_url: Option<&str>,
    phone: Option<&str>,
) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET name = COALESCE($2, name),
            avatar_url = COALESCE($3, avatar_url),
            phone = COALESCE($4, phone),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(name)
    .bind(avatar_url)
    .bind(phone)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Update user role (admin only)
pub async fn update_user_role(
    pool: &PgPool,
    user_id: Uuid,
    role_name: &str,
) -> Result<User> {
    // Get role ID
    let role = get_role_by_name(pool, role_name)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Role '{}' not found", role_name))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET role_id = $2,
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(role.id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Update auth provider info for a user (syncs provider and provider_id on login)
pub async fn update_auth_provider(
    pool: &PgPool,
    user_id: Uuid,
    auth_provider: &str,
    auth_provider_id: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE users
        SET auth_provider = $2,
            auth_provider_id = $3,
            updated_at = NOW()
        WHERE id = $1
        "#
    )
    .bind(user_id)
    .bind(auth_provider)
    .bind(auth_provider_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Restore a soft-deleted user account.
/// Called when a user logs back in after deleting their account.
/// Also updates the auth_provider_id in case it changed (e.g., user deleted Supabase account and re-signed up).
pub async fn restore_user(
    pool: &PgPool,
    user_id: Uuid,
    name: Option<&str>,
    avatar_url: Option<&str>,
    auth_provider_id: &str,
) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET deleted_at = NULL,
            name = $2,
            avatar_url = $3,
            auth_provider_id = $4,
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(name)
    .bind(avatar_url)
    .bind(auth_provider_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Soft delete user - marks as deleted and clears PII while preserving records.
/// Returns an error if user is an organizer with active sessions.
pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<()> {
    let mut tx = pool.begin().await?;

    // Check if user has any sessions as organizer
    let session_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sessions WHERE organizer_id = $1 AND cancelled = false"
    )
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    if session_count.0 > 0 {
        return Err(anyhow::anyhow!(
            "Cannot delete account with active sessions. Please cancel or transfer your sessions first."
        ));
    }

    // Cancel pending bookings and release slots back to sessions
    let pending_bookings: Vec<(Uuid, i32, Uuid)> = sqlx::query_as(
        r#"
        SELECT id, guest_count, session_id
        FROM bookings
        WHERE user_id = $1 AND payment_status = 'pending'
        "#
    )
    .bind(user_id)
    .fetch_all(&mut *tx)
    .await?;

    for (_, guest_count, session_id) in &pending_bookings {
        let slots_to_release = 1 + guest_count; // User + guests
        sqlx::query(
            "UPDATE sessions SET available_slots = available_slots + $1 WHERE id = $2"
        )
        .bind(slots_to_release)
        .bind(session_id)
        .execute(&mut *tx)
        .await?;
    }

    // Cancel pending bookings (keep records but mark as cancelled)
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

    // Soft delete the user: set deleted_at and clear PII for privacy
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

/// Suspend a user (admin only)
pub async fn suspend_user(
    pool: &PgPool,
    user_id: Uuid,
    reason: &str,
    until: Option<DateTime<Utc>>,
    suspended_by: Uuid,
) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET suspended_at = NOW(),
            suspended_until = $2,
            suspension_reason = $3,
            suspended_by = $4,
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(until)
    .bind(reason)
    .bind(suspended_by)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Unsuspend a user (admin only)
pub async fn unsuspend_user(pool: &PgPool, user_id: Uuid) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET suspended_at = NULL,
            suspended_until = NULL,
            suspension_reason = NULL,
            suspended_by = NULL,
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}
