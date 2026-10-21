//! File-based internationalization (i18n) support.
//! Reads from and writes to `i18n.toml` in the current working directory.
//!
//! This module is designed to be "bulletproof": it will never panic
//! if the translation file is missing, corrupted, or missing specific keys.
//! It gracefully falls back to the key name itself.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

/// The structure of `i18n.toml`.
#[derive(Serialize, Deserialize, Default)]
struct I18nConfig {
    /// The currently selected language (e.g., "zh-CN").
    current: String,
    /// A list of supported languages.
    languages: Vec<String>,
    /// Map of `language_code` -> `key` -> `translated_text`.
    translations: HashMap<String, HashMap<String, String>>,
}

struct I18nManager {
    config: I18nConfig,
    path: PathBuf,
}

// Global singleton with read-write lock for thread safety
static MANAGER: Lazy<RwLock<Option<I18nManager>>> = Lazy::new(|| RwLock::new(None));

pub struct I18n;

impl I18n {
    /// Initializes i18n from the given path (e.g., "i18n.toml").
    /// If the file does not exist, a generic template will be generated.
    /// If parsing fails, an empty config is used to prevent crashes.
    pub fn init(path: &str) {
        let path_buf = PathBuf::from(path);
        
        let config = match fs::read_to_string(&path_buf) {
            Ok(content) => {
                // 解析失败则降级为空配置，绝不 panic
                toml::from_str(&content).unwrap_or_else(|e| {
                    eprintln!("⚠️ Failed to parse i18n.toml: {}. Using empty config.", e);
                    Self::generic_config()
                })
            }
            Err(_) => {
                // 文件不存在，生成一个空白的通用模板并写入磁盘
                let cfg = Self::generic_config();
                if let Ok(toml_str) = toml::to_string_pretty(&cfg) {
                    let _ = fs::write(&path_buf, toml_str);
                }
                cfg
            }
        };

        let manager = I18nManager { config, path: path_buf };
        
        // 忽略锁中毒，保证稳健性
        if let Ok(mut lock) = MANAGER.write() {
            *lock = Some(manager);
        }
    }

    /// Generates a generic, empty template.
    /// Developers should fill in the translations manually in the generated file.
    fn generic_config() -> I18nConfig {
        let mut translations = HashMap::new();
        
        // 默认只有语言代码，没有任何具体的业务键
        translations.insert("en".to_string(), HashMap::new());
        translations.insert("zh-CN".to_string(), HashMap::new());

        I18nConfig {
            current: "en".to_string(),
            languages: vec!["en".to_string(), "zh-CN".to_string()],
            translations,
        }
    }

    /// Switches the current language and immediately saves it to disk.
    pub fn set_language(lang: &str) {
        if let Ok(mut lock) = MANAGER.write() {
            if let Some(manager) = lock.as_mut() {
                if manager.config.languages.contains(&lang.to_string()) {
                    manager.config.current = lang.to_string();
                    Self::save_to_disk(manager);
                } else {
                    eprintln!("⚠️ Language '{}' is not in the supported list.", lang);
                }
            }
        }
    }

    /// Gets the currently selected language.
    pub fn current_language() -> String {
        if let Ok(lock) = MANAGER.read() {
            if let Some(manager) = lock.as_ref() {
                return manager.config.current.clone();
            }
        }
        "en".to_string()
    }

    /// Gets the list of supported languages.
    pub fn languages() -> Vec<String> {
        if let Ok(lock) = MANAGER.read() {
            if let Some(manager) = lock.as_ref() {
                return manager.config.languages.clone();
            }
        }
        Vec::new()
    }

    /// Translates a key.
    /// Fallback chain:
    /// 1. Current language
    /// 2. English ("en")
    /// 3. The key itself (prevents UI from showing empty strings)
    pub fn translate(key: &str) -> String {
        let lock = match MANAGER.read() {
            Ok(l) => l,
            Err(_) => return key.to_string(), // 锁中毒也绝不 panic
        };

        let Some(manager) = lock.as_ref() else {
            return key.to_string();
        };

        let current = &manager.config.current;

        // 1. 尝试当前语言
        if let Some(dict) = manager.config.translations.get(current) {
            if let Some(text) = dict.get(key) {
                return text.clone();
            }
        }

        // 2. 回退到英文
        if let Some(dict) = manager.config.translations.get("en") {
            if let Some(text) = dict.get(key) {
                return text.clone();
            }
        }

        // 3. 终极回退：返回键名本身
        key.to_string()
    }

    fn save_to_disk(manager: &I18nManager) {
        if let Ok(toml_str) = toml::to_string_pretty(&manager.config) {
            let _ = fs::write(&manager.path, toml_str);
        }
    }
}

/// The macro to translate strings easily.
/// Usage: `btn.set_label(&tr!("file_open"));`
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::core::i18n::I18n::translate($key)
    };
}