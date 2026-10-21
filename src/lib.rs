//! # fltk-kit
//! A collection of reusable utilities to eliminate boilerplate in `fltk-rs`.
//!
//! The library is organized into three layers:
//! - [`core`]: Pure logic (geometry, validation, strings). Zero FLTK dependencies.
//! - [`widget`]: Extension traits, layout helpers, and high-level UI components.
//! - [`async_kit`]: Safe, channel-based utilities for background tasks.

// --- 模块声明 ---
pub mod core;
pub mod widget;
pub mod async_kit;

// ==========================================
// Core Layer (纯逻辑，无 FLTK 依赖)
// ==========================================
pub use crate::core::rect::Rect;
pub use crate::core::theme::Theme;
pub use crate::core::text::Text;
pub use crate::core::paths::Paths;
pub use crate::core::i18n::{I18n};
pub use crate::core::validator::{Validator, ValidationError, Required, NumberRange};
// Core 导出
pub use crate::core::state::State;


// ==========================================
// Widget Layer (UI 辅助与扩展 Trait)
// ==========================================

// 1. 基础辅助组件 (原 0.1.0 版本核心功能)
pub use crate::widget::dialogs::Dialogs;
pub use crate::widget::fonts::{Fonts, FontSetup, FontSource};
pub use crate::widget::auto_page::AutoPage;
pub use crate::widget::tree::{TreeHelper, TreeBindingExt};
pub use crate::widget::windows::Windows;
pub use crate::widget::clipboard::Clipboard;
pub use crate::widget::progress_ext::ProgressBarExt;
pub use crate::async_kit::debouncer::Debouncer;

// 2. 扩展 Trait (消除样板代码)
pub use crate::widget::layout_ext::RectExt;
pub use crate::widget::form_ext::ValidatedInput;
pub use crate::widget::table::{TableHelper, TableBindingExt};
pub use crate::widget::shortcuts::{parse_shortcut, Shortcuts, ShortcutManager};
pub use crate::widget::help::HelpPage;
pub use crate::core::app_info::AppInfo;
pub use crate::widget::tray::TrayIcon;
pub use crate::widget::spinner::Spinner;
// Widget 导出
pub use crate::widget::state_ext::{TabsStateExt, TableStateExt, save_table_widths};
// 3. 可选功能模块
#[cfg(feature = "icon")]
pub use crate::widget::icon::Icon;


pub use crate::widget::persist::Persist;

// ==========================================
// Async Layer (异步与线程通信)
// ==========================================
pub use crate::async_kit::channel::{Channel, Sender};
pub use crate::async_kit::task::{Task, CancelToken};

#[cfg(feature = "fs-watch")]
pub use crate::async_kit::fs_watch::FileWatcher;

pub use crate::core::settings::Settings;

pub mod build_helper;