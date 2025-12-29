use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use loafy_db::queries::users;
use loafy_types::api::{AuthUser, UpdateProfileRequest};
use uuid::Uuid;

use crate::middleware::AppState;
use crate::response::{self, ApiError};

/// Update current user's profile
pub async fn update_profile(
    State(state): State<AppState>,
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<AuthUser>, ApiError> {
    // Handle birthday if provided
    if let Some(birthday) = payload.birthday {
        // Validate birthday is in the past
        let today = Utc::now().date_naive();
        if birthday >= today {
            return Err(response::bad_request("Birthday must be a date in the past"));
        }

        // Try to set birthday (will fail if already set)
        users::set_birthday(&state.db, user.id, birthday)
            .await
            .map_err(|e| {
                let msg = e.to_string();
                if msg.contains("already been set") {
                    response::bad_request("Birthday has already been set and cannot be changed")
                } else {
                    response::internal_error_msg("Failed to set birthday", e)
                }
            })?;
    }

    // Update other user fields in database
    let updated_user = users::update_user(
        &state.db,
        user.id,
        payload.name.as_deref(),
        payload.avatar_url.as_deref(),
        payload.phone.as_deref(),
    )
    .await
    .map_err(|e| response::internal_error_msg("Failed to update profile", e))?;

    // Fetch updated user with role
    let user_with_role = users::find_with_role_by_id(&state.db, updated_user.id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::internal_error("Failed to fetch updated user"))?;

    Ok(Json(user_with_role.into()))
}

/// Delete current user's account and all associated data
pub async fn delete_account(
    State(state): State<AppState>,
    crate::middleware::AuthUser(user): crate::middleware::AuthUser,
) -> Result<StatusCode, ApiError> {
    // Get the Supabase user ID before deleting from our database
    let supabase_user_id: Uuid = user.auth_provider_id.parse()
        .map_err(|_| response::internal_error("Invalid auth provider ID"))?;

    // Delete user from our database first (handles cascading deletes)
    users::delete_user(&state.db, user.id)
        .await
        .map_err(|e| {
            // Check if it's the "active sessions" error
            let msg = e.to_string();
            if msg.contains("active sessions") {
                response::bad_request(&msg)
            } else {
                response::internal_error_msg("Failed to delete account", e)
            }
        })?;

    // Delete user from Supabase Auth
    state.supabase.delete_user(supabase_user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete user from Supabase: {}", e);
            // Don't fail the request - database deletion succeeded
            // Log the error for manual cleanup if needed
        })
        .ok();

    Ok(StatusCode::NO_CONTENT)
}
