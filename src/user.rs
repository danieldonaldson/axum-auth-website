use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub known_as: String,
    pub password: String,
    pub group: u8, // ... add other user fields as needed
}
