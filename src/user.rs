use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub known_as: String,
    pub password: String,
    // ... add other user fields as needed
}
