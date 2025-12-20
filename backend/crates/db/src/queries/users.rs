use crate::models::{Role, User, UserWithRole};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

/// Find user by email
pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Find user by ID
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Find user with role by ID
pub async fn find_with_role_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserWithRole>> {
    let user = sqlx::query_as::<_, UserWithRole>(
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
            u.created_at as user_created_at,
            u.updated_at as user_updated_at,
            r.name as role_name
        FROM users u
        JOIN roles r ON u.role_id = r.id
        WHERE u.id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Find user with role by email
pub async fn find_with_role_by_email(pool: &PgPool, email: &str) -> Result<Option<UserWithRole>> {
    let user = sqlx::query_as::<_, UserWithRole>(
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
            u.created_at as user_created_at,
            u.updated_at as user_updated_at,
            r.name as role_name
        FROM users u
        JOIN roles r ON u.role_id = r.id
        WHERE u.email = $1
        "#
    )
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
