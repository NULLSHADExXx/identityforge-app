use crate::database::{Database, Profile};
use crate::fingerprint::{Fingerprint, FingerprintGenerator};
use crate::launcher::BrowserLauncher;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, State};
use uuid::Uuid;

/// Application state shared across commands
pub struct AppState {
    pub db: Arc<Database>,
    pub launcher: Arc<BrowserLauncher>,
}

/// Response wrapper for API calls
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Proxy configuration input
#[derive(Deserialize, Default)]
pub struct ProxyInput {
    pub enabled: Option<bool>,
    pub proxy_type: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Input for creating a new profile
#[derive(Deserialize)]
pub struct CreateProfileInput {
    pub name: String,
    pub platform: Option<String>,
    pub default_url: Option<String>,
    pub proxy: Option<ProxyInput>,
}

/// Input for updating a profile
#[derive(Deserialize)]
pub struct UpdateProfileInput {
    pub id: String,
    pub name: Option<String>,
    pub user_agent: Option<String>,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
    pub webgl_vendor: Option<String>,
    pub webgl_renderer: Option<String>,
    pub hardware_concurrency: Option<i32>,
    pub device_memory: Option<i32>,
    pub platform: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub default_url: Option<String>,
    pub proxy: Option<ProxyInput>,
}

/// Input for launching a profile
#[derive(Deserialize)]
pub struct LaunchProfileInput {
    pub profile_id: String,
    pub start_url: Option<String>,
}

/// Profile with active status
#[derive(Serialize)]
pub struct ProfileWithStatus {
    #[serde(flatten)]
    pub profile: Profile,
    pub is_active: bool,
}

/// Cookie structure for import/export
#[derive(Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<f64>,
    pub http_only: bool,
    pub secure: bool,
    pub same_site: Option<String>,
}

// ============================================
// PROFILE COMMANDS
// ============================================

/// Get all profiles
#[tauri::command]
pub async fn get_profiles(state: State<'_, AppState>) -> Result<ApiResponse<Vec<ProfileWithStatus>>, ()> {
    match state.db.get_all_profiles() {
        Ok(profiles) => {
            let profiles_with_status: Vec<ProfileWithStatus> = profiles
                .into_iter()
                .map(|p| {
                    let is_active = state.launcher.is_profile_active(&p.id);
                    ProfileWithStatus {
                        profile: p,
                        is_active,
                    }
                })
                .collect();
            Ok(ApiResponse::ok(profiles_with_status))
        }
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Get a single profile by ID
#[tauri::command]
pub async fn get_profile(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<ApiResponse<Profile>, ()> {
    match state.db.get_profile(&profile_id) {
        Ok(profile) => Ok(ApiResponse::ok(profile)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Create a new profile with auto-generated fingerprint
#[tauri::command]
pub async fn create_profile(
    state: State<'_, AppState>,
    input: CreateProfileInput,
) -> Result<ApiResponse<Profile>, ()> {
    let mut generator = FingerprintGenerator::new();
    
    let fingerprint = match input.platform.as_deref() {
        Some(platform) => generator.generate_for_platform(platform),
        None => generator.generate(),
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let default_url = input.default_url.unwrap_or_else(|| "https://www.google.com".to_string());

    // Extract proxy settings
    let (proxy_enabled, proxy_type, proxy_host, proxy_port, proxy_username, proxy_password) = 
        if let Some(proxy) = input.proxy {
            (
                proxy.enabled.unwrap_or(false),
                proxy.proxy_type.unwrap_or_else(|| "http".to_string()),
                proxy.host.unwrap_or_default(),
                proxy.port.unwrap_or(0),
                proxy.username,
                proxy.password,
            )
        } else {
            (false, "http".to_string(), String::new(), 0, None, None)
        };

    let profile = Profile {
        id: Uuid::new_v4().to_string(),
        name: input.name,
        user_agent: fingerprint.user_agent,
        screen_width: fingerprint.screen_width,
        screen_height: fingerprint.screen_height,
        webgl_vendor: fingerprint.webgl_vendor,
        webgl_renderer: fingerprint.webgl_renderer,
        hardware_concurrency: fingerprint.hardware_concurrency,
        device_memory: fingerprint.device_memory,
        platform: fingerprint.platform,
        timezone: fingerprint.timezone,
        language: fingerprint.language,
        default_url,
        proxy_enabled,
        proxy_type,
        proxy_host,
        proxy_port,
        proxy_username,
        proxy_password,
        created_at: now,
        last_used: None,
    };

    match state.db.create_profile(&profile) {
        Ok(_) => Ok(ApiResponse::ok(profile)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Bulk create multiple profiles
#[tauri::command(rename_all = "camelCase")]
pub async fn bulk_create_profiles(
    state: State<'_, AppState>,
    count: i32,
    name_prefix: String,
    platform: Option<String>,
    default_url: Option<String>,
    proxy: Option<ProxyInput>,
) -> Result<ApiResponse<Vec<Profile>>, ()> {
    let mut generator = FingerprintGenerator::new();
    let mut created_profiles = Vec::new();
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let url = default_url.unwrap_or_else(|| "https://www.google.com".to_string());

    // Extract proxy settings
    let (proxy_enabled, proxy_type, proxy_host, proxy_port, proxy_username, proxy_password) = 
        if let Some(p) = proxy {
            (
                p.enabled.unwrap_or(false),
                p.proxy_type.unwrap_or_else(|| "http".to_string()),
                p.host.unwrap_or_default(),
                p.port.unwrap_or(0),
                p.username,
                p.password,
            )
        } else {
            (false, "http".to_string(), String::new(), 0, None, None)
        };

    for i in 1..=count {
        let fingerprint = match platform.as_deref() {
            Some(p) => generator.generate_for_platform(p),
            None => generator.generate(),
        };

        let profile = Profile {
            id: Uuid::new_v4().to_string(),
            name: format!("{} {}", name_prefix, i),
            user_agent: fingerprint.user_agent,
            screen_width: fingerprint.screen_width,
            screen_height: fingerprint.screen_height,
            webgl_vendor: fingerprint.webgl_vendor,
            webgl_renderer: fingerprint.webgl_renderer,
            hardware_concurrency: fingerprint.hardware_concurrency,
            device_memory: fingerprint.device_memory,
            platform: fingerprint.platform,
            timezone: fingerprint.timezone,
            language: fingerprint.language,
            default_url: url.clone(),
            proxy_enabled,
            proxy_type: proxy_type.clone(),
            proxy_host: proxy_host.clone(),
            proxy_port,
            proxy_username: proxy_username.clone(),
            proxy_password: proxy_password.clone(),
            created_at: now.clone(),
            last_used: None,
        };

        match state.db.create_profile(&profile) {
            Ok(_) => created_profiles.push(profile),
            Err(e) => return Ok(ApiResponse::err(format!("Failed at profile {}: {}", i, e))),
        }
    }

    Ok(ApiResponse::ok(created_profiles))
}

/// Update an existing profile
#[tauri::command]
pub async fn update_profile(
    state: State<'_, AppState>,
    input: UpdateProfileInput,
) -> Result<ApiResponse<Profile>, ()> {
    let mut profile = match state.db.get_profile(&input.id) {
        Ok(p) => p,
        Err(e) => return Ok(ApiResponse::err(e.to_string())),
    };

    if let Some(name) = input.name {
        profile.name = name;
    }
    if let Some(user_agent) = input.user_agent {
        profile.user_agent = user_agent;
    }
    if let Some(screen_width) = input.screen_width {
        profile.screen_width = screen_width;
    }
    if let Some(screen_height) = input.screen_height {
        profile.screen_height = screen_height;
    }
    if let Some(webgl_vendor) = input.webgl_vendor {
        profile.webgl_vendor = webgl_vendor;
    }
    if let Some(webgl_renderer) = input.webgl_renderer {
        profile.webgl_renderer = webgl_renderer;
    }
    if let Some(hardware_concurrency) = input.hardware_concurrency {
        profile.hardware_concurrency = hardware_concurrency;
    }
    if let Some(device_memory) = input.device_memory {
        profile.device_memory = device_memory;
    }
    if let Some(platform) = input.platform {
        profile.platform = platform;
    }
    if let Some(timezone) = input.timezone {
        profile.timezone = timezone;
    }
    if let Some(language) = input.language {
        profile.language = language;
    }
    if let Some(default_url) = input.default_url {
        profile.default_url = default_url;
    }

    // Update proxy settings if provided
    if let Some(proxy) = input.proxy {
        if let Some(enabled) = proxy.enabled {
            profile.proxy_enabled = enabled;
        }
        if let Some(proxy_type) = proxy.proxy_type {
            profile.proxy_type = proxy_type;
        }
        if let Some(host) = proxy.host {
            profile.proxy_host = host;
        }
        if let Some(port) = proxy.port {
            profile.proxy_port = port;
        }
        if proxy.username.is_some() {
            profile.proxy_username = proxy.username;
        }
        if proxy.password.is_some() {
            profile.proxy_password = proxy.password;
        }
    }

    match state.db.update_profile(&profile) {
        Ok(_) => Ok(ApiResponse::ok(profile)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Delete a profile
#[tauri::command(rename_all = "camelCase")]
pub async fn delete_profile(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<ApiResponse<()>, ()> {
    // Check if profile is active
    if state.launcher.is_profile_active(&profile_id) {
        return Ok(ApiResponse::err("Cannot delete an active profile. Close the browser window first.".to_string()));
    }

    match state.db.delete_profile(&profile_id) {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Delete all inactive profiles
#[tauri::command]
pub async fn delete_all_inactive_profiles(
    state: State<'_, AppState>,
) -> Result<ApiResponse<i32>, ()> {
    // Get all profiles
    let profiles = match state.db.get_all_profiles() {
        Ok(p) => p,
        Err(e) => return Ok(ApiResponse::err(e.to_string())),
    };

    let mut deleted_count = 0;
    
    for profile in profiles {
        // Skip active profiles
        if state.launcher.is_profile_active(&profile.id) {
            continue;
        }
        
        // Delete inactive profile
        if state.db.delete_profile(&profile.id).is_ok() {
            deleted_count += 1;
        }
    }

    Ok(ApiResponse::ok(deleted_count))
}

/// Regenerate fingerprint for a profile
#[tauri::command(rename_all = "camelCase")]
pub async fn regenerate_fingerprint(
    state: State<'_, AppState>,
    profile_id: String,
    platform: Option<String>,
) -> Result<ApiResponse<Profile>, ()> {
    let mut profile = match state.db.get_profile(&profile_id) {
        Ok(p) => p,
        Err(e) => return Ok(ApiResponse::err(e.to_string())),
    };

    let mut generator = FingerprintGenerator::new();
    let fingerprint = match platform.as_deref() {
        Some(p) => generator.generate_for_platform(p),
        None => generator.generate(),
    };

    profile.user_agent = fingerprint.user_agent;
    profile.screen_width = fingerprint.screen_width;
    profile.screen_height = fingerprint.screen_height;
    profile.webgl_vendor = fingerprint.webgl_vendor;
    profile.webgl_renderer = fingerprint.webgl_renderer;
    profile.hardware_concurrency = fingerprint.hardware_concurrency;
    profile.device_memory = fingerprint.device_memory;
    profile.platform = fingerprint.platform;
    profile.timezone = fingerprint.timezone;
    profile.language = fingerprint.language;
    // Keep the existing default_url and proxy settings

    match state.db.update_profile(&profile) {
        Ok(_) => Ok(ApiResponse::ok(profile)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

// ============================================
// LAUNCHER COMMANDS
// ============================================

/// Launch a browser window for a profile
#[tauri::command]
pub async fn launch_profile(
    app: AppHandle,
    state: State<'_, AppState>,
    input: LaunchProfileInput,
) -> Result<ApiResponse<String>, ()> {
    // Use provided URL, or profile's default URL will be used by launcher
    let start_url = input.start_url.as_deref();

    match state.launcher.launch_profile(&app, &state.db, &input.profile_id, start_url) {
        Ok(window_label) => {
            Ok(ApiResponse::ok(window_label))
        }
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Close a profile's browser window
#[tauri::command(rename_all = "camelCase")]
pub async fn close_profile_window(
    app: AppHandle,
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<ApiResponse<()>, ()> {
    match state.launcher.close_profile(&app, &profile_id) {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Get list of active profile IDs
#[tauri::command]
pub async fn get_active_profiles(state: State<'_, AppState>) -> Result<ApiResponse<Vec<String>>, ()> {
    Ok(ApiResponse::ok(state.launcher.get_active_profile_ids()))
}

/// Navigate a profile's browser to a URL
#[tauri::command(rename_all = "camelCase")]
pub async fn navigate_profile(
    app: AppHandle,
    state: State<'_, AppState>,
    profile_id: String,
    url: String,
) -> Result<ApiResponse<()>, ()> {
    match state.launcher.navigate(&app, &profile_id, &url) {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

// ============================================
// COOKIE COMMANDS
// ============================================

/// Export cookies for a profile
#[tauri::command(rename_all = "camelCase")]
pub async fn export_cookies(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<ApiResponse<String>, ()> {
    let cookies_path = state.db.get_cookies_path(&profile_id);
    
    if cookies_path.exists() {
        match std::fs::read_to_string(&cookies_path) {
            Ok(content) => Ok(ApiResponse::ok(content)),
            Err(e) => Ok(ApiResponse::err(format!("Failed to read cookies: {}", e))),
        }
    } else {
        // Return empty array if no cookies file exists
        Ok(ApiResponse::ok("[]".to_string()))
    }
}

/// Import cookies for a profile
#[tauri::command(rename_all = "camelCase")]
pub async fn import_cookies(
    state: State<'_, AppState>,
    profile_id: String,
    cookies_json: String,
) -> Result<ApiResponse<()>, ()> {
    // Validate JSON
    if serde_json::from_str::<Vec<Cookie>>(&cookies_json).is_err() {
        return Ok(ApiResponse::err("Invalid cookies JSON format".to_string()));
    }

    let cookies_path = state.db.get_cookies_path(&profile_id);
    
    // Ensure parent directory exists
    if let Some(parent) = cookies_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    match std::fs::write(&cookies_path, &cookies_json) {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(format!("Failed to save cookies: {}", e))),
    }
}

/// Clear cookies for a profile
#[tauri::command(rename_all = "camelCase")]
pub async fn clear_cookies(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<ApiResponse<()>, ()> {
    let cookies_path = state.db.get_cookies_path(&profile_id);
    
    if cookies_path.exists() {
        match std::fs::remove_file(&cookies_path) {
            Ok(_) => Ok(ApiResponse::ok(())),
            Err(e) => Ok(ApiResponse::err(format!("Failed to clear cookies: {}", e))),
        }
    } else {
        Ok(ApiResponse::ok(()))
    }
}

// ============================================
// SETTINGS COMMANDS
// ============================================

/// Get a setting value
#[tauri::command]
pub async fn get_setting(
    state: State<'_, AppState>,
    key: String,
) -> Result<ApiResponse<Option<String>>, ()> {
    match state.db.get_setting(&key) {
        Ok(value) => Ok(ApiResponse::ok(value)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Set a setting value
#[tauri::command]
pub async fn set_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<ApiResponse<()>, ()> {
    match state.db.set_setting(&key, &value) {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

// ============================================
// UTILITY COMMANDS
// ============================================

/// Preview a fingerprint without creating a profile
#[tauri::command]
pub async fn preview_fingerprint(platform: Option<String>) -> Result<ApiResponse<Fingerprint>, ()> {
    let mut generator = FingerprintGenerator::new();
    let fingerprint = match platform.as_deref() {
        Some(p) => generator.generate_for_platform(p),
        None => generator.generate(),
    };
    Ok(ApiResponse::ok(fingerprint))
}
