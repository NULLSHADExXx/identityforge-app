mod commands;
mod database;
mod fingerprint;
mod launcher;

use tauri::{Manager, WindowEvent};

use commands::AppState;
use database::Database;
use launcher::BrowserLauncher;
use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Set up logging in debug mode
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");
            
            let db_path = app_data_dir.join("identityforge.db");
            let profiles_dir = app_data_dir.join("profiles");

            log::info!("Database path: {:?}", db_path);
            log::info!("Profiles directory: {:?}", profiles_dir);

            let db = Database::new(&db_path, profiles_dir)
                .expect("Failed to initialize database");

            // Initialize launcher
            let launcher = BrowserLauncher::new();

            // Create app state
            let state = AppState {
                db: Arc::new(db),
                launcher: Arc::new(launcher),
            };

            // Manage state
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Profile commands
            commands::get_profiles,
            commands::get_profile,
            commands::create_profile,
            commands::update_profile,
            commands::delete_profile,
            commands::delete_all_inactive_profiles,
            commands::bulk_create_profiles,
            commands::regenerate_fingerprint,
            // Launcher commands
            commands::launch_profile,
            commands::close_profile_window,
            commands::get_active_profiles,
            commands::navigate_profile,
            // Cookie commands
            commands::export_cookies,
            commands::import_cookies,
            commands::clear_cookies,
            // Settings commands
            commands::get_setting,
            commands::set_setting,
            // Utility commands
            commands::preview_fingerprint,
        ])
        .on_window_event(|window, event| {
            // Handle window close events for profile windows
            if let WindowEvent::CloseRequested { .. } = event {
                let label = window.label();
                
                // Check if this is a profile window (starts with "profile_")
                if label.starts_with("profile_") {
                    // Extract profile_id from window label
                    // Label format: "profile_{uuid_with_underscores}"
                    let profile_id = label
                        .strip_prefix("profile_")
                        .map(|s| s.replace("_", "-"))
                        .unwrap_or_default();
                    
                    log::info!("Profile window closed: {}", profile_id);
                    
                    // Get the launcher from app state and remove the window
                    if let Some(state) = window.try_state::<AppState>() {
                        state.launcher.on_window_closed(&profile_id);
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
