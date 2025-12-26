use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Proxy configuration for a profile
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxyConfig {
    pub enabled: bool,
    pub proxy_type: String,  // "http", "https", "socks5"
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Represents a browser profile with fingerprint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub user_agent: String,
    pub screen_width: i32,
    pub screen_height: i32,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub hardware_concurrency: i32,
    pub device_memory: i32,
    pub platform: String,
    pub timezone: String,
    pub language: String,
    pub default_url: String,
    // Proxy settings
    pub proxy_enabled: bool,
    pub proxy_type: String,
    pub proxy_host: String,
    pub proxy_port: i32,
    pub proxy_username: Option<String>,
    pub proxy_password: Option<String>,
    pub created_at: String,
    pub last_used: Option<String>,
}

impl Profile {
    pub fn get_proxy_config(&self) -> ProxyConfig {
        ProxyConfig {
            enabled: self.proxy_enabled,
            proxy_type: self.proxy_type.clone(),
            host: self.proxy_host.clone(),
            port: self.proxy_port,
            username: self.proxy_username.clone(),
            password: self.proxy_password.clone(),
        }
    }
}

/// Database wrapper for thread-safe access
pub struct Database {
    conn: Mutex<Connection>,
    profiles_dir: PathBuf,
}

impl Database {
    /// Initialize database at the given path
    pub fn new(db_path: &PathBuf, profiles_dir: PathBuf) -> Result<Self, DatabaseError> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::create_dir_all(&profiles_dir)?;

        let conn = Connection::open(db_path)?;
        
        // Create profiles table with proxy fields
        conn.execute(
            "CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                user_agent TEXT NOT NULL,
                screen_width INTEGER NOT NULL,
                screen_height INTEGER NOT NULL,
                webgl_vendor TEXT NOT NULL,
                webgl_renderer TEXT NOT NULL,
                hardware_concurrency INTEGER NOT NULL,
                device_memory INTEGER NOT NULL,
                platform TEXT NOT NULL,
                timezone TEXT NOT NULL,
                language TEXT NOT NULL,
                default_url TEXT NOT NULL DEFAULT 'https://www.google.com',
                proxy_enabled INTEGER NOT NULL DEFAULT 0,
                proxy_type TEXT NOT NULL DEFAULT 'http',
                proxy_host TEXT NOT NULL DEFAULT '',
                proxy_port INTEGER NOT NULL DEFAULT 0,
                proxy_username TEXT,
                proxy_password TEXT,
                created_at TEXT NOT NULL,
                last_used TEXT
            )",
            [],
        )?;

        // Migration: Add columns if they don't exist
        let migrations = [
            "ALTER TABLE profiles ADD COLUMN default_url TEXT NOT NULL DEFAULT 'https://www.google.com'",
            "ALTER TABLE profiles ADD COLUMN proxy_enabled INTEGER NOT NULL DEFAULT 0",
            "ALTER TABLE profiles ADD COLUMN proxy_type TEXT NOT NULL DEFAULT 'http'",
            "ALTER TABLE profiles ADD COLUMN proxy_host TEXT NOT NULL DEFAULT ''",
            "ALTER TABLE profiles ADD COLUMN proxy_port INTEGER NOT NULL DEFAULT 0",
            "ALTER TABLE profiles ADD COLUMN proxy_username TEXT",
            "ALTER TABLE profiles ADD COLUMN proxy_password TEXT",
        ];
        
        for migration in migrations {
            let _ = conn.execute(migration, []);
        }

        // Create settings table for extensibility
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // Create plugins/addons table for extensibility
        conn.execute(
            "CREATE TABLE IF NOT EXISTS plugins (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                config TEXT,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Database {
            conn: Mutex::new(conn),
            profiles_dir,
        })
    }

    /// Create a new profile
    pub fn create_profile(&self, profile: &Profile) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO profiles (
                id, name, user_agent, screen_width, screen_height,
                webgl_vendor, webgl_renderer, hardware_concurrency,
                device_memory, platform, timezone, language, default_url,
                proxy_enabled, proxy_type, proxy_host, proxy_port, proxy_username, proxy_password,
                created_at, last_used
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
            params![
                profile.id,
                profile.name,
                profile.user_agent,
                profile.screen_width,
                profile.screen_height,
                profile.webgl_vendor,
                profile.webgl_renderer,
                profile.hardware_concurrency,
                profile.device_memory,
                profile.platform,
                profile.timezone,
                profile.language,
                profile.default_url,
                profile.proxy_enabled,
                profile.proxy_type,
                profile.proxy_host,
                profile.proxy_port,
                profile.proxy_username,
                profile.proxy_password,
                profile.created_at,
                profile.last_used,
            ],
        )?;

        // Create profile data directory
        let profile_dir = self.profiles_dir.join(&profile.id);
        std::fs::create_dir_all(&profile_dir)?;

        Ok(())
    }

    /// Get all profiles
    pub fn get_all_profiles(&self) -> Result<Vec<Profile>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, user_agent, screen_width, screen_height,
                    webgl_vendor, webgl_renderer, hardware_concurrency,
                    device_memory, platform, timezone, language, default_url,
                    proxy_enabled, proxy_type, proxy_host, proxy_port, proxy_username, proxy_password,
                    created_at, last_used
             FROM profiles ORDER BY created_at DESC"
        )?;

        let profiles = stmt.query_map([], |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                user_agent: row.get(2)?,
                screen_width: row.get(3)?,
                screen_height: row.get(4)?,
                webgl_vendor: row.get(5)?,
                webgl_renderer: row.get(6)?,
                hardware_concurrency: row.get(7)?,
                device_memory: row.get(8)?,
                platform: row.get(9)?,
                timezone: row.get(10)?,
                language: row.get(11)?,
                default_url: row.get(12)?,
                proxy_enabled: row.get(13)?,
                proxy_type: row.get(14)?,
                proxy_host: row.get(15)?,
                proxy_port: row.get(16)?,
                proxy_username: row.get(17)?,
                proxy_password: row.get(18)?,
                created_at: row.get(19)?,
                last_used: row.get(20)?,
            })
        })?;

        let mut result = Vec::new();
        for profile in profiles {
            result.push(profile?);
        }
        Ok(result)
    }

    /// Get a single profile by ID
    pub fn get_profile(&self, id: &str) -> Result<Profile, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, user_agent, screen_width, screen_height,
                    webgl_vendor, webgl_renderer, hardware_concurrency,
                    device_memory, platform, timezone, language, default_url,
                    proxy_enabled, proxy_type, proxy_host, proxy_port, proxy_username, proxy_password,
                    created_at, last_used
             FROM profiles WHERE id = ?1"
        )?;

        let profile = stmt.query_row([id], |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                user_agent: row.get(2)?,
                screen_width: row.get(3)?,
                screen_height: row.get(4)?,
                webgl_vendor: row.get(5)?,
                webgl_renderer: row.get(6)?,
                hardware_concurrency: row.get(7)?,
                device_memory: row.get(8)?,
                platform: row.get(9)?,
                timezone: row.get(10)?,
                language: row.get(11)?,
                default_url: row.get(12)?,
                proxy_enabled: row.get(13)?,
                proxy_type: row.get(14)?,
                proxy_host: row.get(15)?,
                proxy_port: row.get(16)?,
                proxy_username: row.get(17)?,
                proxy_password: row.get(18)?,
                created_at: row.get(19)?,
                last_used: row.get(20)?,
            })
        }).map_err(|_| DatabaseError::ProfileNotFound(id.to_string()))?;

        Ok(profile)
    }

    /// Update profile
    pub fn update_profile(&self, profile: &Profile) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "UPDATE profiles SET
                name = ?2, user_agent = ?3, screen_width = ?4, screen_height = ?5,
                webgl_vendor = ?6, webgl_renderer = ?7, hardware_concurrency = ?8,
                device_memory = ?9, platform = ?10, timezone = ?11, language = ?12,
                default_url = ?13, proxy_enabled = ?14, proxy_type = ?15, proxy_host = ?16,
                proxy_port = ?17, proxy_username = ?18, proxy_password = ?19, last_used = ?20
             WHERE id = ?1",
            params![
                profile.id,
                profile.name,
                profile.user_agent,
                profile.screen_width,
                profile.screen_height,
                profile.webgl_vendor,
                profile.webgl_renderer,
                profile.hardware_concurrency,
                profile.device_memory,
                profile.platform,
                profile.timezone,
                profile.language,
                profile.default_url,
                profile.proxy_enabled,
                profile.proxy_type,
                profile.proxy_host,
                profile.proxy_port,
                profile.proxy_username,
                profile.proxy_password,
                profile.last_used,
            ],
        )?;

        if rows == 0 {
            return Err(DatabaseError::ProfileNotFound(profile.id.clone()));
        }
        Ok(())
    }

    /// Update last used timestamp
    pub fn update_last_used(&self, id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono_now();
        conn.execute(
            "UPDATE profiles SET last_used = ?2 WHERE id = ?1",
            params![id, now],
        )?;
        Ok(())
    }

    /// Delete a profile
    pub fn delete_profile(&self, id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM profiles WHERE id = ?1", [id])?;
        
        if rows == 0 {
            return Err(DatabaseError::ProfileNotFound(id.to_string()));
        }

        // Remove profile data directory
        let profile_dir = self.profiles_dir.join(id);
        if profile_dir.exists() {
            std::fs::remove_dir_all(&profile_dir)?;
        }

        Ok(())
    }

    /// Get profile data directory path
    pub fn get_profile_data_dir(&self, id: &str) -> PathBuf {
        self.profiles_dir.join(id)
    }

    /// Get cookies file path for a profile
    pub fn get_cookies_path(&self, id: &str) -> PathBuf {
        self.profiles_dir.join(id).join("cookies.json")
    }

    // Settings management for extensibility
    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let result = stmt.query_row([key], |row| row.get(0));
        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(DatabaseError::Sqlite(e)),
        }
    }
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    format!("{}", duration.as_secs())
}
