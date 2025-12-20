use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use loafy_db::{queries::users, models::UserWithRole, PgPool};
use loafy_integrations::supabase::SupabaseAuth;
use loafy_types::AppError;
use uuid::Uuid;

/// Extractor for authenticated user (required)
/// Usage: async fn handler(AuthUser(user): AuthUser)
pub struct AuthUser(pub UserWithRole);

/// Extractor for optional authenticated user
/// Usage: async fn handler(OptionalAuthUser(user): OptionalAuthUser)
pub struct OptionalAuthUser(pub Option<UserWithRole>);

/// Application state containing Supabase client and database pool
#[derive(Clone)]
pub struct AppState {
    pub supabase: SupabaseAuth,
    pub db: PgPool,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid Authorization header".to_string(),
                )
            })?;

        let token = bearer.token();

        // Get app state
        let app_state = AppState::from_ref(state);

        // Verify JWT token
        let claims = app_state
            .supabase
            .verify_token(token)
            .map_err(|e| {
                (
                    StatusCode::UNAUTHORIZED,
                    format!("Invalid token: {}", e),
                )
            })?;

        // Get user ID from claims
        let user_id: Uuid = claims
            .sub
            .parse()
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid user ID in token".to_string(),
                )
            })?;

        // Fetch user from database
        let user = users::find_with_role_by_id(&app_state.db, user_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "User not found".to_string(),
                )
            })?;

        Ok(AuthUser(user))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract Authorization header (optional)
        let auth_header = parts.extract::<TypedHeader<Authorization<Bearer>>>().await;

        let token = match auth_header {
            Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
            Err(_) => return Ok(OptionalAuthUser(None)), // No auth header, return None
        };

        // Get app state
        let app_state = AppState::from_ref(state);

        // Verify JWT token
        let claims = match app_state.supabase.verify_token(&token) {
            Ok(claims) => claims,
            Err(_) => return Ok(OptionalAuthUser(None)), // Invalid token, return None
        };

        // Get user ID from claims
        let user_id: Uuid = match claims.sub.parse() {
            Ok(id) => id,
            Err(_) => return Ok(OptionalAuthUser(None)),
        };

        // Fetch user from database
        let user = users::find_with_role_by_id(&app_state.db, user_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                )
            })?;

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
        "moderator" if !user.is_moderator() => {
            Err(AppError::Forbidden)
        }
        _ => Ok(()),
    }
}
