// Organizations service
use crate::domain::{
    models::organization::Organization,
    repositories::organizations_repository::OrganizationsRepository,
};
use anyhow::Result;
use std::sync::Arc;

pub struct OrganizationsService {
    organizations_repository: Arc<dyn OrganizationsRepository + Send + Sync>,
}

impl OrganizationsService {
    pub fn new(organizations_repository: Arc<dyn OrganizationsRepository + Send + Sync>) -> Self {
        Self {
            organizations_repository,
        }
    }

    pub async fn get_organizations(&self) -> Result<Vec<Organization>> {
        self.organizations_repository.get_organizations().await
    }
}
