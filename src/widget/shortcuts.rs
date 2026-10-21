//! Keyboard shortcut parsing and managing.

use fltk::enums::{Key, Shortcut, EventState};

/// Parses a string like `"Ctrl+S"` or `"Cmd+Shift+Z"` into an FLTK `Shortcut`.
///
/// Returns `None` if the string format is invalid.
pub fn parse_shortcut(s: &str) -> Option<Shortcut> {
    let mut key: Option<Key> = None;
    let mut state = EventState::empty();

    for part in s.split('+') {
        match part.trim().to_lowercase().as_str() {
            "ctrl" => state.insert(EventState::Ctrl),
            "alt" => state.insert(EventState::Alt),
            "shift" => state.insert(EventState::Shift),
            "cmd" | "meta" | "super" => state.insert(EventState::Meta),
            k if k.chars().count() == 1 => {
                key = Some(Key::from_char(k.chars().next().unwrap()));
            }
            "f1" => key = Some(Key::F1),
            "f2" => key = Some(Key::F2),
            "f3" => key = Some(Key::F3),
            "f4" => key = Some(Key::F4),
            "f5" => key = Some(Key::F5),
            "enter" | "return" => key = Some(Key::Enter),
            "escape" | "esc" => key = Some(Key::Escape),
            "space" => key = Some(Key::from_char(' ')),
            _ => return None,
        }
    }

    match key {
        Some(k) if !state.is_empty() => {
            let bits = state.bits() | k.bits();
            Some(Shortcut::from_bits_truncate(bits))  // 加上 Some(...)
        }
        _ => None,
    }
}

/// Placeholder for a global shortcut manager.
/// You can expand this in the future to handle application-wide hotkeys.
pub struct ShortcutManager;

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self
    }
}

/// Predefined shortcuts or helper methods.
pub struct Shortcuts;

impl Shortcuts {
    /// Helper to get Ctrl+S (or Cmd+S on macOS).
    pub fn ctrl_s() -> Shortcut {
        parse_shortcut("Ctrl+S").unwrap()
    }

    /// Helper to get F1.
    pub fn f1() -> Shortcut {
        parse_shortcut("F1").unwrap()
    }

    /// Helper to get F5.
    pub fn f5() -> Shortcut {
        parse_shortcut("F5").unwrap()
    }
}