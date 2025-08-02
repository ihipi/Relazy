// Auth events
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri::Manager;

#[derive(Debug, Clone, Serialize)]
pub enum AuthEvent {
    LoggedIn,
    LoggedOut,
}

pub fn emit_auth_event(app: &AppHandle, event: AuthEvent) {
    app.emit("auth_event", event).expect("Failed to emit auth event");
}
