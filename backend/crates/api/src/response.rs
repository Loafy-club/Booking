//! Common API response helpers to reduce error handling boilerplate

use axum::http::StatusCode;

/// Type alias for the common error response tuple
pub type ApiError = (StatusCode, String);

/// Create a NOT_FOUND error response
pub fn not_found(resource: &str) -> ApiError {
    (StatusCode::NOT_FOUND, format!("{} not found", resource))
}

/// Create an INTERNAL_SERVER_ERROR response from a database error
pub fn db_error<E: std::fmt::Display>(err: E) -> ApiError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Database error: {}", err),
    )
}

/// Create an INTERNAL_SERVER_ERROR response with a custom message
pub fn internal_error<E: std::fmt::Display>(err: E) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/// Create an INTERNAL_SERVER_ERROR response with a formatted message
pub fn internal_error_msg<E: std::fmt::Display>(prefix: &str, err: E) -> ApiError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("{}: {}", prefix, err),
    )
}

/// Create a BAD_REQUEST error response
pub fn bad_request(message: impl Into<String>) -> ApiError {
    (StatusCode::BAD_REQUEST, message.into())
}

/// Create an UNAUTHORIZED error response
pub fn unauthorized(message: impl Into<String>) -> ApiError {
    (StatusCode::UNAUTHORIZED, message.into())
}

/// Create a FORBIDDEN error response
pub fn forbidden(message: impl Into<String>) -> ApiError {
    (StatusCode::FORBIDDEN, message.into())
}

/// Create a CONFLICT error response
#[allow(dead_code)]
pub fn conflict(message: impl Into<String>) -> ApiError {
    (StatusCode::CONFLICT, message.into())
}

