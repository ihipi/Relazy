// Auth commands
use crate::presentation::state::AppState;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn login(token: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut self_token = state.token.lock().unwrap();
    *self_token = Some(token);
    Ok(())
}
