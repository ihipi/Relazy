pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use presentation::{
    commands::{auth_commands, organization_commands},
    state::AppState,
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            auth_commands::login,
            organization_commands::get_organizations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
