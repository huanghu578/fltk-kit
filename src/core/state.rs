//! Runtime UI state persistence (e.g., window layouts, active tabs, table columns).
//! Distinct from `Settings` which is for user preferences.

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use toml::Value;

#[derive(Serialize, Deserialize, Default)]
struct StateData {
    // 加上 flatten，让 toml 直接平铺读取里面的字段
    #[serde(flatten)]
    data: HashMap<String, Value>,
}

static STATE_MANAGER: Lazy<Mutex<Option<(StateData, PathBuf)>>> = Lazy::new(|| Mutex::new(None));

pub struct State;

impl State {
    /// Initialize the state manager. Usually called at app startup.
    pub fn init(app_name: &str) {
        let mut path = std::env::var("APPDATA").map(PathBuf::from)
            .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|_| PathBuf::from("."));
        path.push(app_name);
        path.push("state.toml");

        println!("[State] Init 路径: {:?}", path);

        let data = if let Ok(content) = fs::read_to_string(&path) {
            println!("[State] Init 读取到内容:\n{}", content);
            // 🌟 修复：不用 unwrap_or_default，明确打印解析错误
            match toml::from_str::<StateData>(&content) {
                Ok(d) => d,
                Err(e) => {
                    println!("❌ [State] 解析 TOML 失败: {}", e);
                    // 如果文件损坏，直接删掉它，重新开始！
                    let _ = fs::remove_file(&path);
                    StateData::default()
                }
            }
        } else {
            println!("[State] Init 失败: 文件不存在！");
            StateData::default()
        };

        *STATE_MANAGER.lock().unwrap() = Some((data, path));
    }

    pub fn get<T: DeserializeOwned + std::fmt::Debug>(key: &str) -> Option<T> {
        let lock = STATE_MANAGER.lock().unwrap();
        let (data, _) = lock.as_ref()?;
        let val = data.data.get(key)?.clone().try_into().ok();
        println!("[State] Get '{}' -> {:?}", key, val);
        val
    }

    pub fn save() {
        let lock = STATE_MANAGER.lock().unwrap();
        if let Some((data, path)) = lock.as_ref() {
            // 创建父文件夹，否则写入会失败
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(content) = toml::to_string_pretty(&data.data) {
                if let Ok(mut file) = fs::File::create(path) {
                    use std::io::Write;
                    let _ = file.write_all(content.as_bytes());
                    let _ = file.sync_all();
                }
            }
        }
    }

    /// Set a state value.
    pub fn set<T: Serialize>(key: &str, value: T) {
        let mut lock = STATE_MANAGER.lock().unwrap();
        if let Some((data, _)) = lock.as_mut() {
            if let Ok(val) = Value::try_from(value) {
                data.data.insert(key.to_string(), val);
            }
        }
    }
    
}