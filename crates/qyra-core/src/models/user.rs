use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// User tier as defined in PRD DOC 08
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserTier {
    Free,
    Pro,
    Biz,
}

impl Default for UserTier {
    fn default() -> Self {
        UserTier::Free
    }
}

/// Core user model (maps to `user` TABLE in SurrealDB — DOC 06)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub tier: UserTier,
    /// Preferred language: "en" | "tw" | "yo" | "ha" | "sw"
    #[serde(default = "default_language")]
    pub language: String,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn default_language() -> String {
    "en".to_string()
}

/// Payload for creating a new user
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(default = "default_language")]
    pub language: String,
}

/// Payload for user login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

/// Auth token response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}
