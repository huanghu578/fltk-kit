//! Cross-platform global settings manager.
//! Supports dot-notation for nested keys (e.g., "window.width").

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct Settings {
    // 使用 toml::Table 替代 HashMap，方便递归嵌套
    data: toml::value::Table,
    #[serde(skip)]
    path: PathBuf,
    #[serde(skip)]
    is_new: bool,
}

impl Settings {
    /// 🌟 核心加载器：加载指定应用目录下的指定 TOML 文件。
    /// 自动处理 Windows (`AppData/Roaming`) 和 macOS/Linux (`~/.config`) 路径。
    /// 
    /// # Examples
    /// ```ignore
    /// let ui_cfg = Settings::load("MyApp", "ui.toml");
    /// let cad_cfg = Settings::load("MyApp", "cad.toml");
    /// ```
    pub fn load(app_name: &str, toml_name: &str) -> Self {
        let mut path = std::env::var("APPDATA")
            .map(PathBuf::from)
            .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|_| PathBuf::from("."));
        path.push(app_name);
        path.push(toml_name);

        Self::load_from_path(path)
    }

    /// 加载指定的绝对路径或相对路径。
    /// 适用于单程序多配置文件，或者绿色软件（配置跟随 exe）。
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();

        let (data, is_new) = if let Ok(content) = fs::read_to_string(&path) {
            (toml::from_str(&content).unwrap_or_default(), false)
        } else {
            (toml::value::Table::new(), true)
        };

        Self { data, path, is_new }
    }

    /// 返回是否是首次运行。
    pub fn is_new(&self) -> bool {
        self.is_new
    }

    /// Sets a value. Supports dot-notation: "window.width".
    pub fn set<T: Serialize>(&mut self, key: &str, value: T) {
        if let Ok(val) = toml::Value::try_from(value) {
            let parts: Vec<&str> = key.split('.').collect();
            if parts.len() == 1 {
                self.data.insert(key.to_string(), val);
            } else {
                let (table_path, key_name) = parts.split_at(parts.len() - 1);
                let table = Self::get_mut_table(&mut self.data, table_path);
                table.insert(key_name[0].to_string(), val);
            }
        }
    }

    /// Gets a value. Supports dot-notation: "window.width".
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        let parts: Vec<&str> = key.split('.').collect();
        let mut val_opt = None;

        if parts.len() == 1 {
            val_opt = self.data.get(key);
        } else {
            let mut current_val = self.data.get(parts[0])?;
            for i in 1..parts.len() {
                let table = current_val.as_table()?;
                if i == parts.len() - 1 {
                    val_opt = table.get(parts[i]);
                    break;
                } else {
                    current_val = table.get(parts[i])?;
                }
            }
        }

        val_opt.cloned().and_then(|v| v.try_into().ok())
    }

    /// Sets a value only if the key does not already exist.
    pub fn set_default<T: Serialize>(&mut self, key: &str, default_value: T) {
        if self.get::<toml::Value>(key).is_none() {
            self.set(key, default_value);
        }
    }

    /// Saves to disk.
    pub fn save(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = toml::to_string_pretty(&self.data) {
            let _ = fs::write(&self.path, content);
        }
    }

    /// 内部辅助：根据路径找到或创建对应的 Table 引用
    fn get_mut_table<'a>(
        table: &'a mut toml::value::Table,
        path: &[&str],
    ) -> &'a mut toml::value::Table {
        let mut current = table;
        for part in path {
            let entry = current
                .entry(part.to_string())
                .or_insert_with(|| toml::Value::Table(toml::value::Table::new()));
            if !entry.is_table() {
                *entry = toml::Value::Table(toml::value::Table::new());
            }
            current = entry.as_table_mut().unwrap();
        }
        current
    }
}