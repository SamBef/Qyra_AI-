use thiserror::Error;

/// Top-level error type for Qyra.
#[derive(Debug, Error)]
pub enum QyraError {
    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Permission denied: {0}")]
    Forbidden(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl QyraError {
    pub fn http_status(&self) -> u16 {
        match self {
            QyraError::Auth(_) => 401,
            QyraError::NotFound(_) => 404,
            QyraError::Validation(_) => 422,
            QyraError::Forbidden(_) => 403,
            QyraError::Config(_) | QyraError::Database(_) | QyraError::Internal(_) => 500,
        }
    }
}

impl From<anyhow::Error> for QyraError {
    fn from(e: anyhow::Error) -> Self {
        QyraError::Internal(e.to_string())
    }
}
