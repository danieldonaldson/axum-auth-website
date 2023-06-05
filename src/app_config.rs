use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub jwt_secret_key: String,
    pub users_table: String,
}
