use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use loafy_db::{queries::users, PgPool};
use loafy_integrations::supabase::{SupabaseAuth, SupabaseUser};
use loafy_types::api::AuthUser;
use serde::{Deserialize, Serialize};
use crate::middleware::AppState;

#[derive(Debug, Deserialize)]
pub struct AuthCallbackRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: AuthUser,
    pub token: String,
}

/// Handle OAuth callback from Supabase
/// Creates user in database if doesn't exist
pub async fn handle_callback(
    State(state): State<AppState>,
    Json(payload): Json<AuthCallbackRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // Verify token and get user from Supabase
    let supabase_user = state
        .supabase
        .get_user_from_token(&payload.token)
        .await
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                format!("Failed to verify token: {}", e),
            )
        })?;

    // Check if user exists in database
    let db_user = users::find_with_role_by_email(&state.db, &supabase_user.email)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    // Create user if doesn't exist
    let user = match db_user {
        Some(existing_user) => existing_user,
        None => {
            // Create new user with 'user' role by default
            let new_user = users::create_user(
                &state.db,
                &supabase_user.email,
                supabase_user.user_metadata.full_name.as_deref(),
                supabase_user.user_metadata.avatar_url.as_deref(),
                &supabase_user.app_metadata.provider,
                &supabase_user.id.to_string(),
                "user", // Default role
            )
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create user: {}", e),
                )
            })?;

            // Fetch user with role
            users::find_with_role_by_id(&state.db, new_user.id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Database error: {}", e),
                    )
                })?
                .ok_or_else(|| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to fetch created user".to_string(),
                    )
                })?
        }
    };

    // Convert to API response type
    let auth_user = AuthUser {
        id: user.id,
        email: user.email.clone(),
        name: user.name.clone(),
        avatar_url: user.avatar_url.clone(),
        role: match user.role_name.as_str() {
            "admin" => loafy_types::enums::UserRole::Admin,
            "organizer" => loafy_types::enums::UserRole::Organizer,
            "moderator" => loafy_types::enums::UserRole::Moderator,
            _ => loafy_types::enums::UserRole::User,
        },
    };

    Ok(Json(AuthResponse {
        user: auth_user,
        token: payload.token,
    }))
}

/// Get current authenticated user
pub async fn get_current_user(
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
) -> Result<Json<AuthUser>, (StatusCode, String)> {
    let auth_user = AuthUser {
        id: user.id,
        email: user.email.clone(),
        name: user.name.clone(),
        avatar_url: user.avatar_url.clone(),
        role: match user.role_name.as_str() {
            "admin" => loafy_types::enums::UserRole::Admin,
            "organizer" => loafy_types::enums::UserRole::Organizer,
            "moderator" => loafy_types::enums::UserRole::Moderator,
            _ => loafy_types::enums::UserRole::User,
        },
    };

    Ok(Json(auth_user))
}

/// Logout user (signs out from Supabase)
pub async fn logout(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let token = bearer.token();

    state
        .supabase
        .sign_out(token)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to sign out: {}", e),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
