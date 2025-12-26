use crate::database::Database;
use crate::fingerprint::{generate_spoof_script, Fingerprint};
use std::collections::HashMap;
use std::sync::Mutex;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LauncherError {
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
    #[error("Database error: {0}")]
    Database(#[from] crate::database::DatabaseError),
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
}

/// Manages active browser windows
pub struct BrowserLauncher {
    active_windows: Mutex<HashMap<String, String>>, // profile_id -> window_label
}

impl BrowserLauncher {
    pub fn new() -> Self {
        BrowserLauncher {
            active_windows: Mutex::new(HashMap::new()),
        }
    }

    /// Launch a browser window for a profile
    pub fn launch_profile(
        &self,
        app: &AppHandle,
        db: &Database,
        profile_id: &str,
        start_url: Option<&str>,
    ) -> Result<String, LauncherError> {
        // Check if window already exists
        {
            let existing_label = {
                let windows = self.active_windows.lock().unwrap();
                windows.get(profile_id).cloned()
            };
            
            if let Some(label) = existing_label {
                if let Some(window) = app.get_webview_window(&label) {
                    window.show()?;
                    window.set_focus()?;
                    return Ok(label);
                }
            }
        }

        // Get profile from database
        let profile = db.get_profile(profile_id)?;
        
        // Get profile data directory for isolation
        let data_dir = db.get_profile_data_dir(profile_id);
        
        // Ensure data directory exists
        std::fs::create_dir_all(&data_dir).ok();
        
        // Create unique window label
        let window_label = format!("profile_{}", profile_id.replace("-", "_"));
        
        // Generate fingerprint from profile (including proxy settings)
        let fingerprint = Fingerprint {
            user_agent: profile.user_agent.clone(),
            platform: profile.platform.clone(),
            screen_width: profile.screen_width,
            screen_height: profile.screen_height,
            webgl_vendor: profile.webgl_vendor.clone(),
            webgl_renderer: profile.webgl_renderer.clone(),
            hardware_concurrency: profile.hardware_concurrency,
            device_memory: profile.device_memory,
            timezone: profile.timezone.clone(),
            language: profile.language.clone(),
            default_url: profile.default_url.clone(),
            proxy_enabled: profile.proxy_enabled,
            proxy_type: profile.proxy_type.clone(),
            proxy_host: profile.proxy_host.clone(),
            proxy_port: profile.proxy_port,
            proxy_username: profile.proxy_username.clone(),
            proxy_password: profile.proxy_password.clone(),
        };
        
        // Generate the spoof script with persistent noise seed based on profile ID
        let spoof_script = generate_spoof_script(&fingerprint, profile_id);
        
        // Determine URL to load
        let url_str = start_url
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| {
                if profile.default_url.is_empty() {
                    "https://www.google.com"
                } else {
                    &profile.default_url
                }
            });
        
        // Build the webview window with isolation
        let window = WebviewWindowBuilder::new(
            app,
            &window_label,
            WebviewUrl::External(url_str.parse().unwrap_or_else(|_| "https://www.google.com".parse().unwrap()))
        )
        .title(format!("IdentityForge - {}", profile.name))
        .inner_size(
            profile.screen_width as f64 * 0.8,
            profile.screen_height as f64 * 0.8
        )
        .min_inner_size(800.0, 600.0)
        .data_directory(PathBuf::from(&data_dir))
        .user_agent(&profile.user_agent)
        .initialization_script(&spoof_script)
        .build()?;
        
        // Track the window
        {
            let mut windows = self.active_windows.lock().unwrap();
            windows.insert(profile_id.to_string(), window_label.clone());
        }

        // Update last used timestamp
        db.update_last_used(profile_id).ok();

        // Navigate to URL after window is created (backup method)
        let url_clone = url_str.to_string();
        let _ = window.eval(&format!("setTimeout(() => {{ if (!window.location.href || window.location.href === 'about:blank') {{ window.location.href = '{}'; }} }}, 500);", url_clone));

        Ok(window_label)
    }

    /// Close a profile's browser window
    pub fn close_profile(&self, app: &AppHandle, profile_id: &str) -> Result<(), LauncherError> {
        let label = {
            let mut windows = self.active_windows.lock().unwrap();
            windows.remove(profile_id)
        };

        if let Some(label) = label {
            if let Some(window) = app.get_webview_window(&label) {
                window.close()?;
            }
        }

        Ok(())
    }

    /// Check if a profile has an active window
    pub fn is_profile_active(&self, profile_id: &str) -> bool {
        let windows = self.active_windows.lock().unwrap();
        windows.contains_key(profile_id)
    }

    /// Get all active profile IDs
    pub fn get_active_profile_ids(&self) -> Vec<String> {
        let windows = self.active_windows.lock().unwrap();
        windows.keys().cloned().collect()
    }

    /// Called when a window is closed externally (via X button)
    pub fn on_window_closed(&self, profile_id: &str) {
        let mut windows = self.active_windows.lock().unwrap();
        windows.remove(profile_id);
        log::info!("Profile {} marked as inactive", profile_id);
    }

    /// Navigate a profile's window to a new URL
    pub fn navigate(
        &self,
        app: &AppHandle,
        profile_id: &str,
        url: &str,
    ) -> Result<(), LauncherError> {
        let label = {
            let windows = self.active_windows.lock().unwrap();
            windows.get(profile_id).cloned()
        };

        if let Some(label) = label {
            if let Some(window) = app.get_webview_window(&label) {
                // Use JavaScript to navigate
                window.eval(&format!("window.location.href = '{}';", url))?;
                return Ok(());
            }
        }

        Err(LauncherError::ProfileNotFound(profile_id.to_string()))
    }
}

impl Default for BrowserLauncher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_creation() {
        let launcher = BrowserLauncher::new();
        assert!(!launcher.is_profile_active("test"));
        assert!(launcher.get_active_profile_ids().is_empty());
    }
}
