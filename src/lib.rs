//! # fltk-kit
//!
//! Reusable, high-level utilities and boilerplate reducers for [`fltk-rs`] applications.
//!
//! FLTK is incredibly fast and lightweight, but it requires a lot of boilerplate for modern 
//! desktop app development. `fltk-kit` fills these gaps without forcing you into a heavy framework.
//! It handles the painful parts: responsive layouts, CJK fonts, centered dialogs, thread-safe 
//! async updates, and cross-platform file handling.
//!
//! ## Features
//!
//! - **Responsive Layouts**: [`Rect`] and [`AutoPage`] make complex window resizing as easy as slicing a cake.
//! - **CJK Font Handling**: [`Fonts`] automatically applies the correct system fonts to prevent
//!   Chinese/Japanese/Korean characters from showing as squares (豆腐块).
//! - **Window Helpers**: [`Windows`] handles the tricky initialization sequence for maximizable,
//!   resizable windows.
//! - **Thread-safe Async**: [`Channel`] and [`Task`] safely pass messages from background worker
//!   threads to the FLTK UI thread.
//! - **UI Extensions**: Bind validators to inputs ([`ValidatedInput`]), and fill `SmartTable` or
//!   `Tree` with data in a single line.
//! - **Icons & Spinner**: Embed window icons ([`Icon`]) and display a beautiful, zero-dependency
//!   loading spinner ([`Spinner`]).
//! - **Global Settings**: [`Settings`] provides a robust, dot-notation configuration manager.
//! - **App Metadata**: [`AppInfo`] easily extracts `Cargo.toml` metadata for "About" dialogs.
//!
//! ## Quick Start
//!
//! ```no_run
//! use fltk::{app, button, prelude::*, text::TextDisplay};
//! use fltk_kit::{AutoPage, Rect, Windows};
//!
//! fn main() {
//!     let app = app::App::default();
//!     let mut win = Windows::create_maximized(600, 400, "Layout Demo");
//!
//!     let mut page = AutoPage::new(0, 0, 600, 400, "");
//!     page.set_top(20);
//!     page.group().begin();
//!
//!     let mut btn = button::Button::new(0, 0, 0, 0, "Click");
//!     let mut text = TextDisplay::new(0, 0, 0, 0, "Stretches automatically.");
//!     page.group().end(); 
//!     page.finish();
//!
//!     page.on_layout(move |content: Rect| {
//!         let mut c = content;
//!         let mut bottom = c.cut_bottom(40);
//!         btn.resize(bottom.x, bottom.y, 80, bottom.h);
//!         text.resize(c.x, c.y, c.w, c.h);
//!     });
//!
//!     win.resize_callback(move |w,_,_,_,_| { page.relayout(w.w(), w.h()); });
//!     app.run().unwrap();
//! }
//! ```
//!
//! [`fltk-rs`]: https://github.com/fltk-rs/fltk-rs

pub mod async_kit;
pub mod core;
pub mod widget;

// ==========================================
// Core Exports
// ==========================================

pub use crate::core::app_info::AppInfo;
pub use crate::core::i18n::I18n;
pub use crate::core::rect::Rect;
pub use crate::core::settings::Settings;
pub use crate::core::text::Text;
pub use crate::core::validator::{NumberRange, Required, ValidationError, Validator};

// ==========================================
// Widget Exports
// ==========================================

pub use crate::widget::auto_page::AutoPage;
pub use crate::widget::dialogs::Dialogs;
pub use crate::widget::fonts::Fonts;
pub use crate::widget::help::HelpPage;
pub use crate::widget::spinner::Spinner;
pub use crate::widget::windows::Windows;
pub use crate::widget::icon::Icon;

// ==========================================
// Trait Exports
// ==========================================

pub use crate::widget::form_ext::ValidatedInput;
pub use crate::widget::layout_ext::RectExt;
pub use crate::widget::table::TableBindingExt;
pub use crate::widget::tree::TreeBindingExt;

// ==========================================
// Async Exports
// ==========================================

pub use crate::async_kit::channel::{Channel, Sender};
pub use crate::async_kit::task::Task;