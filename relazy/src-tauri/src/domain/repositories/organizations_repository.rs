// Organizations repository
use crate::domain::models::organization::Organization;
use async_trait::async_trait;

#[async_trait]
pub trait OrganizationsRepository {
    async fn get_organizations(&self) -> anyhow::Result<Vec<Organization>>;
}
