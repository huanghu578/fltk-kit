//! Window state persistence.
use fltk::window::Window;
use fltk::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
struct WindowState { x: i32, y: i32, w: i32, h: i32 }

pub struct Persist;

impl Persist {
    fn config_path(key: &str) -> PathBuf {
        let mut path = std::env::var("APPDATA").map(PathBuf::from)
            .or_else(|_| std::env::var("HOME").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|_| PathBuf::from("."));
        path.push("fltk-kit-app"); path.push(format!("{}.toml", key)); path
    }

    /// Restore geometry before `window.show()`.
    pub fn restore(window: &mut Window, key: &str) {
        if let Ok(content) = fs::read_to_string(Self::config_path(key)) {
            if let Ok(state) = toml::from_str::<WindowState>(&content) {
                window.set_pos(state.x, state.y); window.set_size(state.w, state.h);
            }
        }
    }

    /// Save geometry on window close.
    pub fn save(window: &Window, key: &str) {
        let state = WindowState { x: window.x(), y: window.y(), w: window.w(), h: window.h() };
        if let Ok(content) = toml::to_string(&state) {
            let path = Self::config_path(key);
            if let Some(parent) = path.parent() { let _ = fs::create_dir_all(parent); }
            let _ = fs::write(path, content);
        }
    }
}