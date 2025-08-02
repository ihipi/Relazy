// Organization commands
use crate::{
    application::services::organizations_service::OrganizationsService,
    domain::models::organization::Organization,
    infrastructure::{
        api::github_api::GithubAPI,
        repositories::organizations_repository_impl::OrganizationsRepositoryImpl,
    },
    presentation::state::AppState,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_organizations(state: State<'_, AppState>) -> Result<Vec<Organization>, String> {
    let token = state.token.lock().unwrap().clone().ok_or("Not logged in")?;
    let github_api = Arc::new(GithubAPI::new(&token));
    let organizations_repository =
        Arc::new(OrganizationsRepositoryImpl::new(github_api.clone()));
    let organizations_service = OrganizationsService::new(organizations_repository);
    organizations_service
        .get_organizations()
        .await
        .map_err(|e| e.to_string())
}
