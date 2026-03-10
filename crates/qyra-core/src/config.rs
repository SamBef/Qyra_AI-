use serde::Deserialize;

/// Top-level application configuration loaded from environment variables.
/// Values are sourced from `.env` in development and real env vars in production.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// Application environment: "development" | "production"
    #[serde(default = "default_env")]
    pub app_env: String,

    /// Port the Relay server listens on
    #[serde(default = "default_port")]
    pub port: u16,

    /// SurrealDB connection URL (e.g. ws://localhost:8000)
    pub surrealdb_url: String,

    /// SurrealDB namespace
    #[serde(default = "default_ns")]
    pub surrealdb_ns: String,

    /// SurrealDB database name
    #[serde(default = "default_db")]
    pub surrealdb_db: String,

    /// SurrealDB root username
    pub surrealdb_user: String,

    /// SurrealDB root password
    pub surrealdb_pass: String,

    /// Supabase project URL for auth
    pub supabase_url: String,

    /// Supabase anon public key
    pub supabase_anon_key: String,

    /// JWT secret for signing access tokens
    pub jwt_secret: String,

    /// Upstash Redis URL
    pub redis_url: String,

    /// Upstash Redis token
    pub redis_token: String,
}

fn default_env() -> String {
    "development".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_ns() -> String {
    "qyra".to_string()
}

fn default_db() -> String {
    "qyra_db".to_string()
}

impl AppConfig {
    /// Load config from environment variables (.env file in dev).
    pub fn load() -> anyhow::Result<Self> {
        // Load .env file if present (silently skip if not found)
        let _ = dotenvy::dotenv();

        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .build()?
            .try_deserialize::<AppConfig>()?;

        Ok(config)
    }

    pub fn is_development(&self) -> bool {
        self.app_env == "development"
    }
}
