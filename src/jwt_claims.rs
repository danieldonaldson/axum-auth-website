use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub exp: u64,
    // ... add other claims as needed
}

impl JwtClaims {
    pub fn new(username: &str) -> Self {
        let expiration = SystemTime::now()
            .checked_add(Duration::from_secs(3600 * 24 * 31)) // JWT expires in 1 month
            .expect("Failed to calculate expiration time");

        JwtClaims {
            username: username.to_owned(),
            exp: expiration
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
