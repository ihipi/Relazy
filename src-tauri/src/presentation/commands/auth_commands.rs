use crate::domain::models::user::User;
use crate::infrastructure::api::github_api::GithubAPI;
// Auth commands
use crate::presentation::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn login(token: String, state: State<'_, AppState>) -> Result<(), String> {
    let github_api = GithubAPI::new(&token);

    match github_api.get_authenticated_user().await {
        Ok(user) => {
            let app_user = User {
                login: user.login.clone(),
                avatar_url: format!("{}", user.avatar_url),
                html_url: format!("{}", user.html_url),
                name: user.name.unwrap_or_else(|| "Unknown".to_string()),
            };
            let mut state_token = state.token.lock().unwrap();
            *state_token = Some(token);
            let mut state_user = state.user.lock().unwrap();
            *state_user = Some(app_user);
            Ok(())
        }
        Err(e) => Err(format!("Failed to authenticate user: {}", e)),
    }
}
