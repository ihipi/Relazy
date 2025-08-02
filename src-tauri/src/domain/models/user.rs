// User model
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
    pub name: String,
}
