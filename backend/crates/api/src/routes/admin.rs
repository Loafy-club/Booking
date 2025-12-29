use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Duration;
use loafy_types::{parse_period, validate_payment_method, validate_payment_status, validate_role};
use loafy_db::{
    models::{bonus_types, transaction_types},
    queries::{admin, bookings, sessions as sessions_queries, subscriptions, ticket_transactions, users},
};
use loafy_types::api::admin::{
    AdminBookingResponse, AdminSessionResponse, AdminUserResponse,
    PageInfo, PaginatedBookingsResponse, PaginatedSessionsResponse, PaginatedUsersResponse,
    SuspendUserRequest, UpdateBookingRequest, UpdateUserRequest,
};
use loafy_types::api::subscriptions::{
    AdminGrantTicketsRequest, AdminUserTicketsResponse, TicketBalanceResponse,
    TicketTransactionResponse,
};
use loafy_types::api::sessions::ParticipantInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::middleware::{AppState, AuthUser, require_role};
use crate::response::{self, ApiError};

/// Query parameters for stats endpoint
#[derive(Deserialize)]
pub struct StatsQuery {
    /// Period filter: "7d", "30d", "90d", "365d", or "all"
    #[serde(default = "default_period")]
    pub period: String,
}

fn default_period() -> String {
    "30d".to_string()
}

/// Daily data point for sparklines
#[derive(Serialize)]
pub struct DailyDataPoint {
    pub date: String,
    pub value: i64,
}

/// Previous period stats for comparison
#[derive(Serialize)]
pub struct PreviousPeriodResponse {
    pub new_users: i64,
    pub total_bookings: i64,
    pub total_revenue_vnd: i64,
    pub upcoming_sessions: i64,
}

/// Daily chart data
#[derive(Serialize)]
pub struct DailyChartData {
    pub users: Vec<DailyDataPoint>,
    pub bookings: Vec<DailyDataPoint>,
    pub revenue: Vec<DailyDataPoint>,
    pub sessions: Vec<DailyDataPoint>,
}

/// Admin dashboard statistics response
#[derive(Serialize)]
pub struct StatsResponse {
    pub total_users: i64,
    pub new_users: i64,
    pub total_sessions: i64,
    pub total_bookings: i64,
    pub pending_bookings: i64,
    pub confirmed_bookings: i64,
    pub cancelled_bookings: i64,
    pub total_revenue_vnd: i64,
    pub upcoming_sessions: i64,
    pub period: String,
    pub previous_period: Option<PreviousPeriodResponse>,
    pub daily_data: Option<DailyChartData>,
}

/// Get admin dashboard statistics
pub async fn get_stats(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<StatsQuery>,
) -> Result<Json<StatsResponse>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Parse period to get since date and duration
    let period = parse_period(&query.period);
    let (since, days) = (period.since, period.days);

    let stats = admin::get_admin_stats(&state.db, since)
        .await
        .map_err(response::db_error)?;

    // Get previous period stats and daily data if a specific period is selected
    let (previous_period, daily_data) = if let (Some(current_start), Some(period_days)) = (since, days) {
        let previous_start = current_start - Duration::days(period_days);

        // Fetch previous period stats
        let prev_stats = admin::get_previous_period_stats(&state.db, current_start, previous_start)
            .await
            .map_err(response::db_error)?;

        // Fetch daily data for sparklines
        let (users_daily, bookings_daily, revenue_daily, sessions_daily) = tokio::try_join!(
            admin::get_daily_stats(&state.db, current_start, "users"),
            admin::get_daily_stats(&state.db, current_start, "bookings"),
            admin::get_daily_stats(&state.db, current_start, "revenue"),
            admin::get_daily_stats(&state.db, current_start, "sessions"),
        ).map_err(response::db_error)?;

        (
            Some(PreviousPeriodResponse {
                new_users: prev_stats.new_users,
                total_bookings: prev_stats.total_bookings,
                total_revenue_vnd: prev_stats.total_revenue_vnd,
                upcoming_sessions: prev_stats.upcoming_sessions,
            }),
            Some(DailyChartData {
                users: users_daily.into_iter().map(|d| DailyDataPoint { date: d.date, value: d.value }).collect(),
                bookings: bookings_daily.into_iter().map(|d| DailyDataPoint { date: d.date, value: d.value }).collect(),
                revenue: revenue_daily.into_iter().map(|d| DailyDataPoint { date: d.date, value: d.value }).collect(),
                sessions: sessions_daily.into_iter().map(|d| DailyDataPoint { date: d.date, value: d.value }).collect(),
            }),
        )
    } else {
        (None, None)
    };

    Ok(Json(StatsResponse {
        total_users: stats.total_users,
        new_users: stats.new_users,
        total_sessions: stats.total_sessions,
        total_bookings: stats.total_bookings,
        pending_bookings: stats.pending_bookings,
        confirmed_bookings: stats.confirmed_bookings,
        cancelled_bookings: stats.cancelled_bookings,
        total_revenue_vnd: stats.total_revenue_vnd,
        upcoming_sessions: stats.upcoming_sessions,
        period: query.period,
        previous_period,
        daily_data,
    }))
}

/// Query parameters for users list endpoint
#[derive(Deserialize)]
pub struct UsersQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    pub search: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    10
}

/// List users with pagination (admin only)
pub async fn list_users(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<UsersQuery>,
) -> Result<Json<PaginatedUsersResponse>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let page = query.page.max(1);
    let per_page = query.per_page.clamp(1, 100);

    let (db_users, total) = admin::list_users_paginated(
        &state.db,
        admin::UsersQueryParams {
            page,
            per_page,
            search: query.search,
            role: query.role,
            status: query.status,
            sort_by: query.sort_by,
            sort_order: query.sort_order,
        },
    )
    .await
    .map_err(response::db_error)?;

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    let data: Vec<AdminUserResponse> = db_users
        .into_iter()
        .map(AdminUserResponse::from)
        .collect();

    Ok(Json(PaginatedUsersResponse {
        data,
        page_info: PageInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}

/// Request to update user role
#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

/// Update user role (admin only)
pub async fn update_user_role(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<Json<AdminUserResponse>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Validate role
    validate_role(&request.role).map_err(response::bad_request)?;

    // Prevent admin from demoting themselves
    if user_id == user.id && request.role != "admin" {
        return Err(response::bad_request("Cannot change your own admin role"));
    }

    // Update the role
    let updated_user = users::update_user_role(&state.db, user_id, &request.role)
        .await
        .map_err(response::db_error)?;

    // Fetch user with role for response
    let user_with_role = users::find_with_role_by_id(&state.db, updated_user.id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    Ok(Json(AdminUserResponse::from(user_with_role)))
}

// =============================================================================
// User Suspension Endpoints
// =============================================================================

/// Suspend a user (admin only)
pub async fn suspend_user(
    AuthUser(admin): AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<SuspendUserRequest>,
) -> Result<Json<AdminUserResponse>, ApiError> {
    require_role(&admin, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Validate reason is not empty
    if request.reason.trim().is_empty() {
        return Err(response::bad_request("Suspension reason is required"));
    }

    // Prevent admin from suspending themselves
    if user_id == admin.id {
        return Err(response::bad_request("Cannot suspend your own account"));
    }

    // Check target user exists and is not an admin
    let target = users::find_with_role_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    if target.is_admin() {
        return Err(response::bad_request("Cannot suspend admin users"));
    }

    // Apply suspension
    users::suspend_user(
        &state.db,
        user_id,
        request.reason.trim(),
        request.until,
        admin.id,
    )
    .await
    .map_err(response::db_error)?;

    // Fetch updated user with role for response
    let user_with_role = users::find_with_role_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    Ok(Json(AdminUserResponse::from(user_with_role)))
}

/// Unsuspend a user (admin only)
pub async fn unsuspend_user(
    AuthUser(admin): AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<AdminUserResponse>, ApiError> {
    require_role(&admin, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Check target user exists
    let _target = users::find_with_role_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    // Remove suspension
    users::unsuspend_user(&state.db, user_id)
        .await
        .map_err(response::db_error)?;

    // Fetch updated user with role for response
    let user_with_role = users::find_with_role_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    Ok(Json(AdminUserResponse::from(user_with_role)))
}

/// Update a user (admin only)
pub async fn update_user(
    AuthUser(admin_user): AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<AdminUserResponse>, ApiError> {
    require_role(&admin_user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Check target user exists
    let _target = users::find_with_role_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    // Validate role if provided
    if let Some(ref role) = request.role {
        validate_role(role).map_err(response::bad_request)?;

        // Prevent admin from demoting themselves
        if user_id == admin_user.id && role != "admin" {
            return Err(response::bad_request("Cannot change your own admin role"));
        }
    }

    // Update the user
    let updated = admin::update_user(
        &state.db,
        user_id,
        admin::UpdateUserParams {
            name: request.name,
            phone: request.phone,
            role: request.role,
        },
    )
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("not found") {
            response::not_found("User")
        } else {
            response::db_error(e)
        }
    })?;

    Ok(Json(AdminUserResponse::from(updated)))
}

/// Delete a user (admin only) - soft delete with PII removal
pub async fn delete_user(
    AuthUser(admin_user): AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    require_role(&admin_user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Prevent admin from deleting themselves
    if user_id == admin_user.id {
        return Err(response::bad_request("Cannot delete your own account"));
    }

    // Check target user exists
    let target = users::find_with_role_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    // Prevent deleting other admins
    if target.is_admin() {
        return Err(response::bad_request("Cannot delete admin users"));
    }

    // Delete the user
    admin::delete_user(&state.db, user_id)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("already deleted") {
                response::not_found("User")
            } else {
                response::db_error(e)
            }
        })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "User deleted successfully"
    })))
}

/// Query parameters for bookings list endpoint
#[derive(Deserialize)]
pub struct BookingsQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    pub search: Option<String>,
    pub payment_status: Option<String>,
    pub session_id: Option<Uuid>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// List bookings with pagination (admin only)
pub async fn list_bookings(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<BookingsQuery>,
) -> Result<Json<PaginatedBookingsResponse>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let page = query.page.max(1);
    let per_page = query.per_page.clamp(1, 100);

    let (bookings, total) = admin::list_bookings_paginated(
        &state.db,
        admin::BookingsQueryParams {
            page,
            per_page,
            search: query.search,
            payment_status: query.payment_status,
            session_id: query.session_id,
            sort_by: query.sort_by,
            sort_order: query.sort_order,
        },
    )
    .await
    .map_err(response::db_error)?;

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    let data: Vec<AdminBookingResponse> = bookings
        .into_iter()
        .map(|b| AdminBookingResponse {
            id: b.id,
            user_id: b.user_id,
            session_id: b.session_id,
            booking_code: b.booking_code,
            guest_count: b.guest_count,
            total_price_vnd: b.price_paid_vnd + b.guest_price_paid_vnd,
            payment_method: b.payment_method,
            payment_status: b.payment_status,
            payment_deadline: b.payment_deadline,
            cancelled_at: b.cancelled_at,
            created_at: b.created_at,
            user_email: b.user_email,
            user_name: b.user_name,
            session_title: b.session_title,
            session_date: b.session_date,
            session_time: b.session_time,
        })
        .collect();

    Ok(Json(PaginatedBookingsResponse {
        data,
        page_info: PageInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}

/// Get a single booking by ID (admin only)
pub async fn get_booking(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(booking_id): Path<Uuid>,
) -> Result<Json<AdminBookingResponse>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let booking = admin::get_booking_by_id(&state.db, booking_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("Booking"))?;

    Ok(Json(AdminBookingResponse {
        id: booking.id,
        user_id: booking.user_id,
        session_id: booking.session_id,
        booking_code: booking.booking_code,
        guest_count: booking.guest_count,
        total_price_vnd: booking.price_paid_vnd + booking.guest_price_paid_vnd,
        payment_method: booking.payment_method,
        payment_status: booking.payment_status,
        payment_deadline: booking.payment_deadline,
        cancelled_at: booking.cancelled_at,
        created_at: booking.created_at,
        user_email: booking.user_email,
        user_name: booking.user_name,
        session_title: booking.session_title,
        session_date: booking.session_date,
        session_time: booking.session_time,
    }))
}

/// Update a booking (admin only)
pub async fn update_booking(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(booking_id): Path<Uuid>,
    Json(request): Json<UpdateBookingRequest>,
) -> Result<Json<AdminBookingResponse>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Validate payment_status if provided
    if let Some(ref status) = request.payment_status {
        validate_payment_status(status).map_err(response::bad_request)?;
    }

    // Validate payment_method if provided
    if let Some(ref method) = request.payment_method {
        validate_payment_method(method).map_err(response::bad_request)?;
    }

    // Validate guest_count if provided
    if let Some(count) = request.guest_count {
        if count < 0 {
            return Err(response::bad_request("Guest count cannot be negative"));
        }
    }

    // Update the booking
    let updated = admin::update_booking(
        &state.db,
        booking_id,
        admin::UpdateBookingParams {
            guest_count: request.guest_count,
            price_paid_vnd: request.price_paid_vnd,
            guest_price_paid_vnd: request.guest_price_paid_vnd,
            payment_method: request.payment_method,
            payment_status: request.payment_status,
        },
    )
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("not found") {
            response::not_found("Booking")
        } else if msg.contains("Not enough available slots") {
            response::bad_request(msg)
        } else {
            response::db_error(e)
        }
    })?;

    // TODO: If admin_notes was provided, store it in an audit log

    Ok(Json(AdminBookingResponse {
        id: updated.id,
        user_id: updated.user_id,
        session_id: updated.session_id,
        booking_code: updated.booking_code,
        guest_count: updated.guest_count,
        total_price_vnd: updated.price_paid_vnd + updated.guest_price_paid_vnd,
        payment_method: updated.payment_method,
        payment_status: updated.payment_status,
        payment_deadline: updated.payment_deadline,
        cancelled_at: updated.cancelled_at,
        created_at: updated.created_at,
        user_email: updated.user_email,
        user_name: updated.user_name,
        session_title: updated.session_title,
        session_date: updated.session_date,
        session_time: updated.session_time,
    }))
}

/// Query parameters for sessions list endpoint
#[derive(Deserialize)]
pub struct SessionsQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    pub search: Option<String>,
    pub status: Option<String>,
    pub organizer_id: Option<Uuid>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// List sessions with pagination (admin only)
pub async fn list_sessions(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<SessionsQuery>,
) -> Result<Json<PaginatedSessionsResponse>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let page = query.page.max(1);
    let per_page = query.per_page.clamp(1, 100);

    let (sessions, total) = admin::list_sessions_paginated(
        &state.db,
        admin::SessionsQueryParams {
            page,
            per_page,
            search: query.search,
            status: query.status,
            organizer_id: query.organizer_id,
            sort_by: query.sort_by,
            sort_order: query.sort_order,
        },
    )
    .await
    .map_err(response::db_error)?;

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    // Build response with participants for each session
    let mut data = Vec::with_capacity(sessions.len());
    for s in sessions {
        // Fetch participants preview (max 5) and count for each session
        let participants = sessions_queries::get_session_participants(&state.db, s.id, Some(5))
            .await
            .unwrap_or_default();
        let count = sessions_queries::count_session_participants(&state.db, s.id)
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

        data.push(AdminSessionResponse {
            id: s.id,
            organizer_id: s.organizer_id,
            organizer_name: s.organizer_name,
            title: s.title,
            date: s.date,
            time: s.time,
            end_time: s.end_time,
            location: s.location,
            courts: s.courts,
            total_slots: s.total_slots,
            available_slots: s.available_slots,
            price_vnd: s.price_vnd,
            cancelled: s.cancelled,
            created_at: s.created_at,
            participants_preview: if participant_infos.is_empty() { None } else { Some(participant_infos) },
            confirmed_count: Some(count),
        });
    }

    Ok(Json(PaginatedSessionsResponse {
        data,
        page_info: PageInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}

/// Role response
#[derive(Serialize)]
pub struct RoleResponse {
    pub id: Uuid,
    pub name: String,
}

/// List all available roles (admin only)
pub async fn list_roles(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<RoleResponse>>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let roles = admin::list_roles(&state.db)
        .await
        .map_err(response::db_error)?;

    let response: Vec<RoleResponse> = roles
        .into_iter()
        .map(|(id, name)| RoleResponse { id, name })
        .collect();

    Ok(Json(response))
}

// =============================================================================
// Profit & Expense Endpoints
// =============================================================================

/// Previous period profit stats
#[derive(Serialize)]
pub struct PreviousProfitStats {
    pub total_expenses_vnd: i64,
    pub net_profit_vnd: i64,
    pub profit_margin_percent: f64,
}

/// Profit stats response
#[derive(Serialize)]
pub struct ProfitStatsResponse {
    pub total_revenue_vnd: i64,
    pub total_expenses_vnd: i64,
    pub net_profit_vnd: i64,
    pub profit_margin_percent: f64,
    pub previous_period: Option<PreviousProfitStats>,
}

/// Get profit statistics (admin only)
pub async fn get_profit_stats(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<StatsQuery>,
) -> Result<Json<ProfitStatsResponse>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let period = parse_period(&query.period);
    let (since, days) = (period.since, period.days);

    let stats = admin::get_profit_stats(&state.db, since)
        .await
        .map_err(response::db_error)?;

    // Get previous period stats for comparison
    let previous_period = if let (Some(current_start), Some(period_days)) = (since, days) {
        let previous_start = current_start - Duration::days(period_days);
        let prev_stats = admin::get_profit_stats(&state.db, Some(previous_start))
            .await
            .map_err(response::db_error)?;

        Some(PreviousProfitStats {
            total_expenses_vnd: prev_stats.total_expenses_vnd,
            net_profit_vnd: prev_stats.net_profit_vnd,
            profit_margin_percent: prev_stats.profit_margin_percent,
        })
    } else {
        None
    };

    Ok(Json(ProfitStatsResponse {
        total_revenue_vnd: stats.total_revenue_vnd,
        total_expenses_vnd: stats.total_expenses_vnd,
        net_profit_vnd: stats.net_profit_vnd,
        profit_margin_percent: stats.profit_margin_percent,
        previous_period,
    }))
}

/// Session profit response
#[derive(Serialize)]
pub struct SessionProfitResponse {
    pub session_id: Uuid,
    pub title: String,
    pub date: String,
    pub revenue_vnd: i64,
    pub expenses_vnd: i64,
    pub profit_vnd: i64,
    pub profit_margin_percent: f64,
}

/// Query params for sessions profit endpoint
#[derive(Deserialize)]
pub struct SessionsProfitQuery {
    #[serde(default = "default_period")]
    pub period: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_limit() -> i32 {
    20
}

/// Get per-session profit breakdown (admin only)
pub async fn get_sessions_profit(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<SessionsProfitQuery>,
) -> Result<Json<Vec<SessionProfitResponse>>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let since = parse_period(&query.period).since;

    let summaries = admin::get_sessions_profit(&state.db, since, query.limit)
        .await
        .map_err(response::db_error)?;

    let response: Vec<SessionProfitResponse> = summaries
        .into_iter()
        .map(|s| SessionProfitResponse {
            session_id: s.session_id,
            title: s.title,
            date: s.date.format("%Y-%m-%d").to_string(),
            revenue_vnd: s.revenue_vnd,
            expenses_vnd: s.expenses_vnd,
            profit_vnd: s.profit_vnd,
            profit_margin_percent: s.profit_margin_percent,
        })
        .collect();

    Ok(Json(response))
}

/// Expense category response
#[derive(Serialize)]
pub struct ExpenseCategoryResponse {
    pub category: String,
    pub total_vnd: i64,
    pub percentage: f64,
}

/// Get expense breakdown by category (admin only)
pub async fn get_expenses_by_category(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<StatsQuery>,
) -> Result<Json<Vec<ExpenseCategoryResponse>>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let since = parse_period(&query.period).since;

    let categories = admin::get_expenses_by_category(&state.db, since)
        .await
        .map_err(response::db_error)?;

    let response: Vec<ExpenseCategoryResponse> = categories
        .into_iter()
        .map(|c| ExpenseCategoryResponse {
            category: c.category,
            total_vnd: c.total_vnd,
            percentage: c.percentage,
        })
        .collect();

    Ok(Json(response))
}

/// Daily profit data point response
#[derive(Serialize)]
pub struct DailyProfitDataPointResponse {
    pub date: String,
    pub revenue_vnd: i64,
    pub expenses_vnd: i64,
    pub profit_vnd: i64,
}

/// Get daily profit data for charts (admin only)
pub async fn get_daily_profit_data(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<StatsQuery>,
) -> Result<Json<Vec<DailyProfitDataPointResponse>>, ApiError> {
    require_role(&user, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    let since = parse_period(&query.period).since_or_default();

    let data = admin::get_daily_profit_data(&state.db, since)
        .await
        .map_err(response::db_error)?;

    let response: Vec<DailyProfitDataPointResponse> = data
        .into_iter()
        .map(|d| DailyProfitDataPointResponse {
            date: d.date,
            revenue_vnd: d.revenue,
            expenses_vnd: d.expenses,
            profit_vnd: d.profit,
        })
        .collect();

    Ok(Json(response))
}

// =============================================================================
// Ticket Management Endpoints
// =============================================================================

/// GET /api/admin/users/:id/tickets
/// Get user's ticket balance and recent transactions (admin only)
pub async fn get_user_tickets(
    AuthUser(admin): AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<AdminUserTicketsResponse>, ApiError> {
    require_role(&admin, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Check user exists
    let _user = users::find_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    // Get subscription and ticket balance
    let subscription = subscriptions::find_by_user_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?;

    // Get recent transactions (last 10)
    let (transactions, _) = ticket_transactions::list_user_transactions(&state.db, user_id, 1, 10)
        .await
        .map_err(response::db_error)?;

    // Convert transactions to response format with booking codes
    let mut recent_transactions = Vec::with_capacity(transactions.len());
    for tx in transactions {
        let booking_code = if let Some(booking_id) = tx.booking_id {
            bookings::find_by_id(&state.db, booking_id)
                .await
                .ok()
                .flatten()
                .map(|b| b.booking_code)
        } else {
            None
        };

        recent_transactions.push(TicketTransactionResponse {
            id: tx.id,
            transaction_type: tx.transaction_type,
            amount: tx.amount,
            balance_after: tx.balance_after,
            notes: tx.notes,
            booking_code,
            created_at: tx.created_at.naive_utc(),
        });
    }

    Ok(Json(AdminUserTicketsResponse {
        user_id,
        tickets_remaining: subscription.as_ref().map(|s| s.tickets_remaining).unwrap_or(0),
        has_active_subscription: subscription.as_ref().map(|s| s.is_active()).unwrap_or(false),
        current_period_end: subscription.and_then(|s| s.current_period_end.map(|dt| dt.naive_utc())),
        recent_transactions,
    }))
}

/// POST /api/admin/users/:id/tickets/grant
/// Grant bonus tickets to a user (admin only)
pub async fn grant_tickets(
    AuthUser(admin): AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<AdminGrantTicketsRequest>,
) -> Result<Json<TicketBalanceResponse>, ApiError> {
    require_role(&admin, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Validate request
    request.validate().map_err(|e| response::bad_request(&e.to_string()))?;

    // Check user exists
    let _user = users::find_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    // Get user's subscription
    let subscription = subscriptions::find_by_user_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::bad_request("User does not have a subscription"))?;

    // Add bonus tickets
    let new_balance = subscriptions::add_bonus_tickets(&state.db, subscription.id, request.amount)
        .await
        .map_err(response::db_error)?;

    // Log the ticket transaction
    ticket_transactions::create_with_pool(
        &state.db,
        user_id,
        Some(subscription.id),
        None,
        transaction_types::BONUS_MANUAL,
        request.amount,
        new_balance,
        request.reason.as_deref(),
        Some(admin.id),
    )
    .await
    .map_err(response::db_error)?;

    // Record the bonus ticket award
    ticket_transactions::create_bonus_ticket(
        &state.db,
        user_id,
        bonus_types::MANUAL,
        request.amount,
        request.reason.as_deref(),
        None,
        Some(admin.id),
        None,
    )
    .await
    .map_err(response::db_error)?;

    tracing::info!(
        "Admin {} granted {} tickets to user {}",
        admin.id,
        request.amount,
        user_id
    );

    Ok(Json(TicketBalanceResponse {
        tickets_remaining: new_balance,
        has_active_subscription: subscription.is_active(),
        current_period_end: subscription.current_period_end.map(|dt| dt.naive_utc()),
    }))
}

/// POST /api/admin/users/:id/tickets/revoke
/// Revoke tickets from a user (admin only)
pub async fn revoke_tickets(
    AuthUser(admin): AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<AdminGrantTicketsRequest>,
) -> Result<Json<TicketBalanceResponse>, ApiError> {
    require_role(&admin, "admin").map_err(|_| response::forbidden("Admin access required"))?;

    // Validate request
    request.validate().map_err(|e| response::bad_request(&e.to_string()))?;

    // Check user exists
    let _user = users::find_by_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::not_found("User"))?;

    // Get user's subscription
    let subscription = subscriptions::find_by_user_id(&state.db, user_id)
        .await
        .map_err(response::db_error)?
        .ok_or_else(|| response::bad_request("User does not have a subscription"))?;

    // Revoke tickets
    let new_balance = subscriptions::revoke_tickets(&state.db, subscription.id, request.amount)
        .await
        .map_err(response::db_error)?;

    // Log the ticket transaction
    ticket_transactions::create_with_pool(
        &state.db,
        user_id,
        Some(subscription.id),
        None,
        transaction_types::REVOKED,
        -request.amount, // negative for revocation
        new_balance,
        request.reason.as_deref(),
        Some(admin.id),
    )
    .await
    .map_err(response::db_error)?;

    tracing::info!(
        "Admin {} revoked {} tickets from user {}",
        admin.id,
        request.amount,
        user_id
    );

    Ok(Json(TicketBalanceResponse {
        tickets_remaining: new_balance,
        has_active_subscription: subscription.is_active(),
        current_period_end: subscription.current_period_end.map(|dt| dt.naive_utc()),
    }))
}
