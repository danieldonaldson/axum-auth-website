use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

pub const EXPIRY_TIME: u64 = 60 * 60 * 24 * 31; // JWT expires in 1 month
pub const REFRESH_TIME_BEFORE_EXPIRY: u64 = 60 * 60 * 24 * 8; // Refresh it 8 days before it expires

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub exp: u64,
    pub group: u8,
}

impl JwtClaims {
    pub fn new(username: &str) -> Self {
        let expiration = SystemTime::now()
            .checked_add(Duration::from_secs(EXPIRY_TIME))
            .expect("Failed to calculate expiration time");

        JwtClaims {
            username: username.to_owned(),
            exp: expiration
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            group: 1,
        }
    }
}
