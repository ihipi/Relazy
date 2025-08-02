// Auth service
use crate::domain::models::user::User;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AuthService {
    async fn get_current_user(&self) -> Result<User>;
}
