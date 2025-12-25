use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use loafy_db::{queries::users, models::UserWithRole, PgPool};
use loafy_integrations::supabase::SupabaseAuth;
use loafy_types::api::admin::SuspendedUserError;
use loafy_types::AppError;

/// Extractor for authenticated user (required)
/// Usage: async fn handler(AuthUser(user): AuthUser)
pub struct AuthUser(pub UserWithRole);

/// Extractor for optional authenticated user
/// Usage: async fn handler(OptionalAuthUser(user): OptionalAuthUser)
#[allow(dead_code)]
pub struct OptionalAuthUser(pub Option<UserWithRole>);

/// Application state containing Supabase client and database pool
#[derive(Clone)]
pub struct AppState {
    pub supabase: SupabaseAuth,
    pub db: PgPool,
}

/// Auth error that can be returned from extractors
pub enum AuthError {
    /// Standard unauthorized error with message
    Unauthorized(String),
    /// User account is suspended
    Suspended(SuspendedUserError),
}

impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, msg).into_response()
            }
            AuthError::Suspended(error) => {
                (StatusCode::FORBIDDEN, Json(error)).into_response()
            }
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                AuthError::Unauthorized("Missing or invalid Authorization header".to_string())
            })?;

        let token = bearer.token();

        // Get app state using FromRef
        let app_state: AppState = AppState::from_ref(state);

        // Verify JWT token (async)
        let claims = app_state
            .supabase
            .verify_token(token)
            .await
            .map_err(|e| {
                AuthError::Unauthorized(format!("Invalid token: {}", e))
            })?;

        // Get Supabase user ID from claims (stored as auth_provider_id in our DB)
        let supabase_user_id = &claims.sub;

        // Fetch user from database by auth_provider_id
        let user = users::find_with_role_by_auth_provider_id(&app_state.db, supabase_user_id)
            .await
            .map_err(|e| {
                AuthError::Unauthorized(format!("Database error: {}", e))
            })?
            .ok_or_else(|| {
                AuthError::Unauthorized("User not found".to_string())
            })?;

        // Check if user is suspended
        if user.is_suspended() {
            return Err(AuthError::Suspended(SuspendedUserError {
                error: "account_suspended".to_string(),
                reason: user.suspension_reason().unwrap_or("No reason provided").to_string(),
                until: user.suspension_until(),
            }));
        }

        Ok(AuthUser(user))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract Authorization header (optional)
        let auth_header = parts.extract::<TypedHeader<Authorization<Bearer>>>().await;

        let token = match auth_header {
            Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
            Err(_) => return Ok(OptionalAuthUser(None)), // No auth header, return None
        };

        // Get app state using FromRef
        let app_state: AppState = AppState::from_ref(state);

        // Verify JWT token (async)
        let claims = match app_state.supabase.verify_token(&token).await {
            Ok(claims) => claims,
            Err(_) => return Ok(OptionalAuthUser(None)), // Invalid token, return None
        };

        // Get Supabase user ID from claims
        let supabase_user_id = &claims.sub;

        // Fetch user from database by auth_provider_id
        let user = users::find_with_role_by_auth_provider_id(&app_state.db, supabase_user_id)
            .await
            .map_err(|e| {
                AuthError::Unauthorized(format!("Database error: {}", e))
            })?;

        // Check if user is suspended
        if let Some(ref u) = user {
            if u.is_suspended() {
                return Err(AuthError::Suspended(SuspendedUserError {
                    error: "account_suspended".to_string(),
                    reason: u.suspension_reason().unwrap_or("No reason provided").to_string(),
                    until: u.suspension_until(),
                }));
            }
        }

        Ok(OptionalAuthUser(user))
    }
}

/// Helper to check if user has required role
pub fn require_role(user: &UserWithRole, required_role: &str) -> Result<(), AppError> {
    match required_role {
        "admin" if !user.is_admin() => {
            Err(AppError::Forbidden)
        }
        "organizer" if !user.is_organizer() => {
            Err(AppError::Forbidden)
        }
        _ => Ok(()),
    }
}
