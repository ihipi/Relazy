// Organization model
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: i64,
    pub login: String,
    pub avatar_url: String,
    pub description: Option<String>,
}
