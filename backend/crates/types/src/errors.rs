use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Payment error: {0}")]
    Payment(String),

    #[error("External service error: {0}")]
    ExternalService(String),
}

impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::Database(_) | Self::Internal(_) => 500,
            Self::NotFound(_) => 404,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::BadRequest(_) | Self::Validation(_) => 400,
            Self::Conflict(_) => 409,
            Self::Payment(_) => 402,
            Self::ExternalService(_) => 502,
        }
    }
}
