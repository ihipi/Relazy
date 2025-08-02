// Organizations repository implementation
use crate::domain::{
    models::organization::Organization, repositories::organizations_repository::OrganizationsRepository,
};
use crate::infrastructure::api::github_api::GithubAPI;
use async_trait::async_trait;
use std::sync::Arc;

pub struct OrganizationsRepositoryImpl {
    github_api: Arc<GithubAPI>,
}

impl OrganizationsRepositoryImpl {
    pub fn new(github_api: Arc<GithubAPI>) -> Self {
        Self { github_api }
    }
}

#[async_trait]
impl OrganizationsRepository for OrganizationsRepositoryImpl {
    async fn get_organizations(&self) -> anyhow::Result<Vec<Organization>> {
        let url = format!("{}/user/orgs", self.github_api.api_url());
        let orgs = self
            .github_api
            .client()
            .get(&url)
            .send()
            .await?
            .json::<Vec<Organization>>()
            .await?;
        Ok(orgs)
    }
}
