use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use loafy_db::{conversions::SessionResponseExt, queries::sessions};
use loafy_types::api::sessions::{CreateSessionRequest, SessionResponse};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::middleware::{AppState, AuthUser, require_role};
use crate::response::{self, ApiError};

#[derive(Debug, Deserialize)]
pub struct SessionFilters {
    pub from_date: Option<NaiveDate>,
    pub organizer_id: Option<Uuid>,
    pub available_only: Option<bool>,
}

/// List upcoming sessions
pub async fn list_sessions(
    State(state): State<AppState>,
    Query(filters): Query<SessionFilters>,
) -> Result<Json<Vec<SessionResponse>>, ApiError> {
    let db_sessions = sessions::list_sessions(
        &state.db,
        filters.from_date,
        filters.organizer_id,
        filters.available_only.unwrap_or(false),
    )
    .await
    .map_err(|e| response::internal_error_msg("Failed to fetch sessions", e))?;

    let response: Vec<SessionResponse> = db_sessions.into_iter().map(Into::into).collect();
    Ok(Json(response))
}

/// Get session by ID
pub async fn get_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SessionResponse>, ApiError> {
    let session = sessions::find_by_id(&state.db, id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Session"))?;

    Ok(Json(session.into()))
}

/// Create session (organizer or admin)
pub async fn create_session(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>, ApiError> {
    // Check if user is organizer or admin
    if !user.is_organizer() {
        return Err(response::forbidden("Only organizers and admins can create sessions"));
    }

    // Validate input
    payload.validate().map_err(|e| response::bad_request(format!("Validation error: {}", e)))?;

    // Create session
    let session = sessions::create_session(
        &state.db,
        user.id,
        &payload.title,
        payload.date,
        payload.time,
        &payload.location,
        payload.courts,
        payload.max_players_per_court,
        payload.price_vnd,
    )
    .await
    .map_err(|e| response::internal_error_msg("Failed to create session", e))?;

    let response: SessionResponse = session.into();
    Ok(Json(response.with_organizer_name(user.name.clone())))
}

/// Update session (admin only)
pub async fn update_session(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>, ApiError> {
    // Only admins can update sessions
    require_role(&user, "admin").map_err(|_| response::forbidden("Only admins can update sessions"))?;

    // Validate input
    payload.validate().map_err(|e| response::bad_request(format!("Validation error: {}", e)))?;

    // Update session
    let session = sessions::update_session(
        &state.db,
        id,
        Some(&payload.title),
        Some(payload.date),
        Some(payload.time),
        Some(&payload.location),
        Some(payload.courts),
        payload.max_players_per_court,
        payload.price_vnd,
    )
    .await
    .map_err(|e| response::internal_error_msg("Failed to update session", e))?;

    Ok(Json(session.into()))
}

/// Delete session (admin only)
pub async fn delete_session(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Only admins can delete sessions
    require_role(&user, "admin").map_err(|_| response::forbidden("Only admins can delete sessions"))?;

    sessions::delete_session(&state.db, id)
        .await
        .map_err(|e| response::internal_error_msg("Failed to delete session", e))?;

    Ok(StatusCode::NO_CONTENT)
}
