// Organizations repository implementation
use crate::domain::{
    models::organization::Organization,
    repositories::organizations_repository::OrganizationsRepository,
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
        let orgs = self
            .github_api
            .octo()
            .current()
            .list_org_memberships_for_authenticated_user()
            .send()
            .await?;
        Ok(orgs
            .into_iter()
            .map(|org| Organization {
                id: None,
                login: org.organization.login,
                url: org.organization_url.to_string(),
                avatar_url: org.organization.avatar_url.to_string(),
                description: org.organization.description,
            })
            .collect::<Vec<Organization>>())
    }
}
