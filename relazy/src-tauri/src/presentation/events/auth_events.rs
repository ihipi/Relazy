// Auth events
use serde::Serialize;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize)]
pub enum AuthEvent {
    LoggedIn,
    LoggedOut,
}

impl AuthEvent {
    pub fn emit(&self, app: &AppHandle) {
        app.emit("auth-event", self).unwrap();
    }
}
