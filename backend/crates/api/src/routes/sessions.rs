use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use loafy_db::{queries::sessions, PgPool};
use loafy_types::{
    api::sessions::{CreateSessionRequest, SessionResponse},
    AppError,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::middleware::{AppState, AuthUser, require_role};

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
) -> Result<Json<Vec<SessionResponse>>, (StatusCode, String)> {
    let db_sessions = sessions::list_sessions(
        &state.db,
        filters.from_date,
        filters.organizer_id,
        filters.available_only.unwrap_or(false),
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch sessions: {}", e),
        )
    })?;

    let response: Vec<SessionResponse> = db_sessions
        .into_iter()
        .map(|s| SessionResponse {
            id: s.id,
            organizer_id: s.organizer_id,
            organizer_name: None, // TODO: Join with users table
            title: s.title,
            date: s.date,
            time: s.time,
            location: s.location,
            courts: s.courts,
            max_players_per_court: s.max_players_per_court.unwrap_or(6),
            total_slots: s.total_slots,
            available_slots: s.available_slots,
            price_vnd: s.price_vnd.unwrap_or(100000),
            price_usd: s.price_usd.map(|d| d.to_string()),
            cancelled: s.cancelled,
        })
        .collect();

    Ok(Json(response))
}

/// Get session by ID
pub async fn get_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SessionResponse>, (StatusCode, String)> {
    let session = sessions::find_by_id(&state.db, id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, "Session not found".to_string())
        })?;

    let response = SessionResponse {
        id: session.id,
        organizer_id: session.organizer_id,
        organizer_name: None, // TODO: Join with users table
        title: session.title,
        date: session.date,
        time: session.time,
        location: session.location,
        courts: session.courts,
        max_players_per_court: session.max_players_per_court.unwrap_or(6),
        total_slots: session.total_slots,
        available_slots: session.available_slots,
        price_vnd: session.price_vnd.unwrap_or(100000),
        price_usd: session.price_usd.map(|d| d.to_string()),
        cancelled: session.cancelled,
    };

    Ok(Json(response))
}

/// Create session (organizer or admin)
pub async fn create_session(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>, (StatusCode, String)> {
    // Check if user is organizer or admin
    if !user.is_organizer() {
        return Err((
            StatusCode::FORBIDDEN,
            "Only organizers and admins can create sessions".to_string(),
        ));
    }

    // Validate input
    payload.validate().map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Validation error: {}", e))
    })?;

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
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create session: {}", e),
        )
    })?;

    let response = SessionResponse {
        id: session.id,
        organizer_id: session.organizer_id,
        organizer_name: user.name.clone(),
        title: session.title,
        date: session.date,
        time: session.time,
        location: session.location,
        courts: session.courts,
        max_players_per_court: session.max_players_per_court.unwrap_or(6),
        total_slots: session.total_slots,
        available_slots: session.available_slots,
        price_vnd: session.price_vnd.unwrap_or(100000),
        price_usd: session.price_usd.map(|d| d.to_string()),
        cancelled: session.cancelled,
    };

    Ok(Json(response))
}

/// Update session (admin only)
pub async fn update_session(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>, (StatusCode, String)> {
    // Only admins can update sessions
    require_role(&user, "admin").map_err(|_| {
        (
            StatusCode::FORBIDDEN,
            "Only admins can update sessions".to_string(),
        )
    })?;

    // Validate input
    payload.validate().map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Validation error: {}", e))
    })?;

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
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update session: {}", e),
        )
    })?;

    let response = SessionResponse {
        id: session.id,
        organizer_id: session.organizer_id,
        organizer_name: None,
        title: session.title,
        date: session.date,
        time: session.time,
        location: session.location,
        courts: session.courts,
        max_players_per_court: session.max_players_per_court.unwrap_or(6),
        total_slots: session.total_slots,
        available_slots: session.available_slots,
        price_vnd: session.price_vnd.unwrap_or(100000),
        price_usd: session.price_usd.map(|d| d.to_string()),
        cancelled: session.cancelled,
    };

    Ok(Json(response))
}

/// Delete session (admin only)
pub async fn delete_session(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Only admins can delete sessions
    require_role(&user, "admin").map_err(|_| {
        (
            StatusCode::FORBIDDEN,
            "Only admins can delete sessions".to_string(),
        )
    })?;

    sessions::delete_session(&state.db, id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete session: {}", e),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
