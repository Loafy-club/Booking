use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use loafy_db::{conversions::SessionResponseExt, queries::{sessions, session_expenses}};
use loafy_types::api::sessions::{CreateSessionRequest, ParticipantInfo, SessionParticipantsResponse, SessionResponse};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::middleware::{AppState, AuthUser, require_role};
use crate::response::{self, ApiError};

#[derive(Debug, Deserialize)]
pub struct SessionFilters {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub time_of_day: Option<String>, // "morning,afternoon,evening" (comma-separated)
    pub location: Option<String>,
    pub organizer_id: Option<Uuid>,
    pub available_only: Option<bool>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// List upcoming sessions
pub async fn list_sessions(
    State(state): State<AppState>,
    Query(filters): Query<SessionFilters>,
) -> Result<Json<Vec<SessionResponse>>, ApiError> {
    let db_sessions = sessions::list_sessions(
        &state.db,
        sessions::SessionQueryFilters {
            from_date: filters.from_date,
            to_date: filters.to_date,
            time_of_day: filters.time_of_day.clone(),
            location: filters.location.clone(),
            organizer_id: filters.organizer_id,
            available_only: filters.available_only.unwrap_or(false),
        },
    )
    .await
    .map_err(|e| response::internal_error_msg("Failed to fetch sessions", e))?;

    // Build response with participants preview for each session
    let mut response = Vec::with_capacity(db_sessions.len());
    for session in db_sessions {
        let session_id = session.id;
        let mut session_response: SessionResponse = session.into();

        // Fetch participants preview (max 5) and count
        let participants = sessions::get_session_participants(&state.db, session_id, Some(5))
            .await
            .unwrap_or_default();
        let count = sessions::count_session_participants(&state.db, session_id)
            .await
            .unwrap_or(0) as i32;

        let participant_infos: Vec<ParticipantInfo> = participants
            .into_iter()
            .map(|p| ParticipantInfo {
                id: p.user_id,
                name: p.name,
                avatar_url: p.avatar_url,
                guest_count: p.guest_count,
            })
            .collect();

        session_response = session_response.with_participants(participant_infos, count);
        response.push(session_response);
    }

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

    // Fetch expenses for this session
    let expenses = session_expenses::list_expenses_for_session(&state.db, id)
        .await
        .map_err(|e| response::internal_error_msg("Failed to fetch expenses", e))?;

    // Calculate total expenses
    let total_expenses: i64 = expenses.iter().map(|e| {
        if e.cost_type == "per_court" {
            e.amount_vnd as i64 * session.courts as i64
        } else {
            e.amount_vnd as i64
        }
    }).sum();

    let expense_responses: Vec<_> = expenses.into_iter().map(Into::into).collect();

    // Fetch participants preview and count
    let participants = sessions::get_session_participants(&state.db, id, Some(5))
        .await
        .unwrap_or_default();
    let count = sessions::count_session_participants(&state.db, id)
        .await
        .unwrap_or(0) as i32;

    let participant_infos: Vec<ParticipantInfo> = participants
        .into_iter()
        .map(|p| ParticipantInfo {
            id: p.user_id,
            name: p.name,
            avatar_url: p.avatar_url,
            guest_count: p.guest_count,
        })
        .collect();

    let response: SessionResponse = session.into();
    let response = response
        .with_expenses(expense_responses, total_expenses)
        .with_participants(participant_infos, count);

    Ok(Json(response))
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

    // Parse start_time to extract date and time
    let start_datetime = NaiveDateTime::parse_from_str(&payload.start_time, "%Y-%m-%dT%H:%M")
        .map_err(|_| response::bad_request("Invalid start_time format. Use YYYY-MM-DDTHH:MM"))?;

    let date = start_datetime.date();
    let time = start_datetime.time();

    // Parse end_time if provided
    let end_time = NaiveDateTime::parse_from_str(&payload.end_time, "%Y-%m-%dT%H:%M")
        .map(|dt| dt.time())
        .ok();

    // Validate expenses if provided
    if let Some(ref expenses) = payload.expenses {
        for expense in expenses {
            // Validate category
            if !["court_rental", "equipment", "instructor", "custom"].contains(&expense.category.as_str()) {
                return Err(response::bad_request(format!("Invalid expense category: {}", expense.category)));
            }
            // Validate cost_type
            if !["per_court", "total"].contains(&expense.cost_type.as_str()) {
                return Err(response::bad_request(format!("Invalid cost type: {}", expense.cost_type)));
            }
            // Custom category requires description
            if expense.category == "custom" && expense.description.is_none() {
                return Err(response::bad_request("Custom expenses require a description"));
            }
            // Amount must be positive
            if expense.amount_vnd <= 0 {
                return Err(response::bad_request("Expense amount must be positive"));
            }
        }
    }

    // For simplicity, treat max_slots as total available (1 court with max_slots players)
    let courts = 1;
    let max_players_per_court = Some(payload.max_slots);

    // Create session
    let session = sessions::create_session(
        &state.db,
        user.id,
        &payload.title,
        date,
        time,
        end_time,
        &payload.location,
        courts,
        max_players_per_court,
        payload.price_vnd,
    )
    .await
    .map_err(|e| response::internal_error_msg("Failed to create session", e))?;

    // Create expenses if provided
    let mut expense_responses = Vec::new();
    let mut total_expenses: i64 = 0;

    if let Some(expenses) = payload.expenses {
        for expense in expenses {
            let created = session_expenses::create_expense(
                &state.db,
                session.id,
                &expense.category,
                expense.description.as_deref(),
                &expense.cost_type,
                expense.amount_vnd,
            )
            .await
            .map_err(|e| response::internal_error_msg("Failed to create expense", e))?;

            // Calculate actual expense (per_court * courts or total)
            let actual_amount = if expense.cost_type == "per_court" {
                expense.amount_vnd as i64 * session.courts as i64
            } else {
                expense.amount_vnd as i64
            };
            total_expenses += actual_amount;

            expense_responses.push(created.into());
        }
    }

    let response: SessionResponse = session.into();
    let response = response
        .with_organizer_name(user.name.clone())
        .with_expenses(expense_responses, total_expenses);

    Ok(Json(response))
}

/// Update session (admin can update any, organizer can update own)
pub async fn update_session(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>, ApiError> {
    // Fetch the session first to check ownership
    let existing_session = sessions::find_by_id(&state.db, id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Session"))?;

    // Check permissions: admin can edit any, organizer can only edit their own
    let is_admin = user.is_admin();
    let is_owner = existing_session.organizer_id == user.id;

    if !is_admin && !is_owner {
        return Err(response::forbidden("You can only edit your own sessions"));
    }

    // Check if user is at least an organizer (handles edge case where regular user somehow has a session)
    if !user.is_organizer() && !is_admin {
        return Err(response::forbidden("Only organizers and admins can update sessions"));
    }

    // Validate input
    payload.validate().map_err(|e| response::bad_request(format!("Validation error: {}", e)))?;

    // Parse start_time to extract date and time
    let start_datetime = NaiveDateTime::parse_from_str(&payload.start_time, "%Y-%m-%dT%H:%M")
        .map_err(|_| response::bad_request("Invalid start_time format. Use YYYY-MM-DDTHH:MM"))?;

    let date = start_datetime.date();
    let time = start_datetime.time();

    // Parse end_time if provided
    let end_time = NaiveDateTime::parse_from_str(&payload.end_time, "%Y-%m-%dT%H:%M")
        .map(|dt| dt.time())
        .ok();

    // For simplicity, treat max_slots as total available (1 court with max_slots players)
    let courts = 1;
    let max_players_per_court = Some(payload.max_slots);

    // Update session
    let session = sessions::update_session(
        &state.db,
        id,
        Some(&payload.title),
        Some(date),
        Some(time),
        end_time,
        Some(&payload.location),
        Some(courts),
        max_players_per_court,
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

/// Get all distinct session locations
pub async fn list_locations(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, ApiError> {
    let locations = sessions::list_locations(&state.db)
        .await
        .map_err(|e| response::internal_error_msg("Failed to fetch locations", e))?;

    Ok(Json(locations))
}

/// Get all participants for a session
pub async fn get_session_participants(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<SessionParticipantsResponse>, ApiError> {
    // Verify session exists
    sessions::find_by_id(&state.db, session_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Session"))?;

    // Fetch all participants (no limit)
    let participants = sessions::get_session_participants(&state.db, session_id, None)
        .await
        .map_err(|e| response::internal_error_msg("Failed to fetch participants", e))?;

    let participant_infos: Vec<ParticipantInfo> = participants
        .into_iter()
        .map(|p| ParticipantInfo {
            id: p.user_id,
            name: p.name,
            avatar_url: p.avatar_url,
            guest_count: p.guest_count,
        })
        .collect();

    let total_count = participant_infos.len() as i32;

    Ok(Json(SessionParticipantsResponse {
        session_id,
        participants: participant_infos,
        total_count,
    }))
}
