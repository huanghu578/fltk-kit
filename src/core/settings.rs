//! Cross-platform global settings manager.
//! Requires the `persist` feature.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Settings {
    data: HashMap<String, toml::Value>,
    #[serde(skip)]
    path: PathBuf,
}

impl Settings {
    /// Loads settings from the user's config directory.
    pub fn load(app_name: &str) -> Self {
        let mut path = std::env::var("APPDATA").map(PathBuf::from)
            .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|_| PathBuf::from("."));
        path.push(app_name);
        path.push("settings.toml");

        let data = if let Ok(content) = fs::read_to_string(&path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Self { data, path }
    }

    /// Sets a value. Call `save()` to persist.
    pub fn set<T: Serialize>(&mut self, key: &str, value: T) {
        if let Ok(val) = toml::Value::try_from(value) {
            self.data.insert(key.to_string(), val);
        }
    }

    /// Gets a value.
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.data.get(key)?.clone().try_into().ok()
    }

    /// Saves to disk.
    pub fn save(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = toml::to_string(&self.data) {
            let _ = fs::write(&self.path, content);
        }
    }
}