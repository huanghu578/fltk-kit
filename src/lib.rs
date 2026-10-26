//! # fltk-kit
//! Reusable utilities for `fltk-rs` applications.

pub mod async_kit;
pub mod core;
pub mod widget;

// Core Exports
pub use crate::core::app_info::AppInfo;
pub use crate::core::i18n::I18n;
pub use crate::core::rect::Rect;
pub use crate::core::settings::Settings;
pub use crate::core::text::Text;
pub use crate::core::validator::{NumberRange, Required, ValidationError, Validator};

// Widget Exports
pub use crate::widget::auto_page::AutoPage;
pub use crate::widget::dialogs::Dialogs;
pub use crate::widget::fonts::Fonts;
pub use crate::widget::help::HelpPage;
pub use crate::widget::icon::Icon;
pub use crate::widget::spinner::Spinner;
pub use crate::widget::windows::Windows; // <-- 新增 // <-- 新增

// Trait Exports
pub use crate::widget::form_ext::ValidatedInput;
pub use crate::widget::table::TableBindingExt;
pub use crate::widget::tree::TreeBindingExt;
pub use crate::widget::layout_ext::RectExt;

// Async Exports
pub use crate::async_kit::channel::{Channel, Sender};
pub use crate::async_kit::task::Task;
