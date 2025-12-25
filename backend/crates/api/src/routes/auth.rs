use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use loafy_db::queries::users;
use loafy_types::api::AuthUser;
use serde::{Deserialize, Serialize};

use crate::middleware::AppState;
use crate::response::{self, ApiError};

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
) -> Result<Json<AuthResponse>, ApiError> {
    // Verify token and get user from Supabase
    let supabase_user = state
        .supabase
        .get_user_from_token(&payload.token)
        .await
        .map_err(|e| response::unauthorized(format!("Failed to verify token: {}", e)))?;

    // Check if user exists in database (including soft-deleted users)
    let db_user = users::find_with_role_by_email(&state.db, &supabase_user.email)
        .await
        .map_err(response::db_error)?;

    // Create user if doesn't exist, or restore if soft-deleted
    let user = match db_user {
        Some(existing_user) => {
            // Check if user was soft-deleted and restore if so
            if existing_user.user_deleted_at.is_some() {
                // Restore the soft-deleted user (also update auth_provider_id in case it changed)
                let restored_user = users::restore_user(
                    &state.db,
                    existing_user.id,
                    supabase_user.user_metadata.full_name.as_deref(),
                    supabase_user.user_metadata.avatar_url.as_deref(),
                    &supabase_user.id.to_string(),
                )
                .await
                .map_err(|e| response::internal_error_msg("Failed to restore user", e))?;

                // Fetch user with role
                users::find_with_role_by_id(&state.db, restored_user.id)
                    .await
                    .map_err(response::db_error)?
                    .ok_or_else(|| response::internal_error("Failed to fetch restored user"))?
            } else {
                // Sync auth provider info on every login to keep it current
                // Use providers array (all linked providers) joined by comma
                let new_auth_provider = supabase_user.app_metadata.providers.join(", ");
                let new_auth_provider_id = supabase_user.id.to_string();
                if existing_user.auth_provider != new_auth_provider || existing_user.auth_provider_id != new_auth_provider_id {
                    users::update_auth_provider(&state.db, existing_user.id, &new_auth_provider, &new_auth_provider_id)
                        .await
                        .map_err(|e| response::internal_error_msg("Failed to update auth provider", e))?;
                }
                existing_user
            }
        }
        None => {
            // Create new user with 'user' role by default
            // Use providers array (all linked providers) joined by comma
            let auth_provider = supabase_user.app_metadata.providers.join(", ");
            let new_user = users::create_user(
                &state.db,
                &supabase_user.email,
                supabase_user.user_metadata.full_name.as_deref(),
                supabase_user.user_metadata.avatar_url.as_deref(),
                &auth_provider,
                &supabase_user.id.to_string(),
                "user", // Default role
            )
            .await
            .map_err(|e| response::internal_error_msg("Failed to create user", e))?;

            // Fetch user with role
            users::find_with_role_by_id(&state.db, new_user.id)
                .await
                .map_err(response::db_error)?
                .ok_or_else(|| response::internal_error("Failed to fetch created user"))?
        }
    };

    Ok(Json(AuthResponse {
        user: user.into(),
        token: payload.token,
    }))
}

/// Get current authenticated user
pub async fn get_current_user(
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
) -> Result<Json<AuthUser>, ApiError> {
    Ok(Json(user.into()))
}

/// Logout user (signs out from Supabase)
pub async fn logout(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, ApiError> {
    let token = bearer.token();

    state
        .supabase
        .sign_out(token)
        .await
        .map_err(|e| response::internal_error_msg("Failed to sign out", e))?;

    Ok(StatusCode::NO_CONTENT)
}
