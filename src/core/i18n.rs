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
    ///
    /// 语言决策规则：
    /// - 文件存在且 `current` 非空 → 使用文件中的用户指定语言（不覆盖）。
    /// - 文件存在但 `current` 为空 → 使用操作系统语言，并立即写回磁盘。
    /// - 文件不存在 → 生成模板，`current` 取操作系统语言，并写盘。
    pub fn init(path: &str) {
        let path_buf = PathBuf::from(path);

        let (config, need_save) = match fs::read_to_string(&path_buf) {
            Ok(content) => {
                // 解析失败则降级为空配置，绝不 panic
                match toml::from_str::<I18nConfig>(&content) {
                    Ok(mut cfg) => {
                        // 关键：已存在文件但 current 为空时，补系统语言并写回
                        if cfg.current.trim().is_empty() {
                            cfg.current = Self::detect_os_language(&cfg.languages);
                            (cfg, true)
                        } else {
                            (cfg, false)
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️ Failed to parse i18n.toml: {}. Using empty config.", e);
                        (Self::generic_config(), true)
                    }
                }
            }
            Err(_) => {
                // 文件不存在，生成通用模板（current 已按系统语言填充）并写盘
                (Self::generic_config(), true)
            }
        };

        let manager = I18nManager {
            config,
            path: path_buf,
        };

        // 需要落盘时先写（此时 manager 还未进入全局锁，直接调用辅助函数）
        if need_save {
            Self::save_to_disk(&manager);
        }

        // 忽略锁中毒，保证稳健性
        if let Ok(mut lock) = MANAGER.write() {
            *lock = Some(manager);
        }
    }

    /// 生成通用模板。`current` 会优先采用操作系统语言，
    /// 若系统语言不在支持列表内则回退到 "en"。
    fn generic_config() -> I18nConfig {
        let languages = vec!["en".to_string(), "zh-CN".to_string()];
        let current = Self::detect_os_language(&languages);

        let mut translations = HashMap::new();
        translations.insert("en".to_string(), HashMap::new());
        translations.insert("zh-CN".to_string(), HashMap::new());

        I18nConfig {
            current,
            languages,
            translations,
        }
    }

    /// 探测操作系统语言并映射到支持列表中的语言代码。
    /// 匹配失败时返回 "en" 作为兜底。
    fn detect_os_language(supported: &[String]) -> String {
        // sys-locale 返回形如 "zh-CN" / "zh_CN" / "zh-Hans-CN" / "en-US" 的字符串
        let raw = sys_locale::get_locale().unwrap_or_default();
        if raw.is_empty() {
            return Self::fallback_language(supported);
        }

        // 1. 精确匹配（忽略大小写与 '-'/'_' 差异）
        let normalized = raw.replace('_', "-").to_lowercase();
        for lang in supported {
            if lang.replace('_', "-").to_lowercase() == normalized {
                return lang.clone();
            }
        }

        // 2. 按主语言子标签匹配（如 "zh-Hans-CN" → "zh" → 命中 "zh-CN"）
        let primary = normalized.split('-').next().unwrap_or("");
        for lang in supported {
            let lang_norm = lang.replace('_', "-").to_lowercase();
            if lang_norm.split('-').next() == Some(primary) {
                return lang.clone();
            }
        }

        // 3. 兜底
        Self::fallback_language(supported)
    }

    /// 支持列表里优先 "en"，否则取第一个。
    fn fallback_language(supported: &[String]) -> String {
        if supported.iter().any(|l| l == "en") {
            "en".to_string()
        } else {
            supported
                .first()
                .cloned()
                .unwrap_or_else(|| "en".to_string())
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
                let cur = manager.config.current.clone();
                if !cur.trim().is_empty() {
                    return cur;
                }
            }
        }
        // 全局未初始化或 current 为空时的兜底
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
