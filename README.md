# fltk-kit 🧰

[![Crates.io](https://img.shields.io/crates/v/fltk-kit.svg)](https://crates.io/crates/fltk-kit)
[![Docs.rs](https://docs.rs/fltk-kit/badge.svg)](https://docs.rs/fltk-kit)
[![License](https://img.shields.io/crates/l/fltk-kit.svg)](#license)

Reusable, high-level utilities and boilerplate reducers for [`fltk-rs`](https://github.com/fltk-rs/fltk-rs) applications.

FLTK is incredibly fast and lightweight, but it requires a lot of boilerplate for modern desktop app development. `fltk-kit` fills these gaps without forcing you into a heavy framework. It handles the painful parts: responsive layouts, CJK fonts, centered dialogs, thread-safe async updates, and cross-platform file handling.

---

## Table of contents

- [Features](#features)
- [Installation](#installation)
- [Quick start](#quick-start)
- [Module Highlights](#module-highlights)
- [Exe Icons (Windows)](#exe-icons-windows)
- [API overview](#api-overview)
- [Design notes](#design-notes)
- [License](#license)

---

## Features

- **Responsive Layouts** — `Rect` and `AutoPage` make complex window resizing as easy as slicing a cake. Never manually calculate `x, y, w, h` again.
- **CJK Font Handling** — `Fonts` automatically applies the correct system fonts to prevent Chinese/Japanese/Korean characters from showing as squares (豆腐块).
- **Thread-safe Async** — `Channel` and `Task` safely pass messages from background worker threads back to the FLTK UI thread without data races.
- **UI Extensions** — Bind validators to inputs (`ValidatedInput`), and fill `SmartTable` or `Tree` with data in a single line.
- **Icons & Spinner** — Embed window icons (`Icon`) and display a beautiful, zero-dependency loading spinner (`Spinner`).
- **Global Settings** — `Settings` provides a robust, dot-notation configuration manager with default values.
- **App Metadata** — `AppInfo` easily extracts `Cargo.toml` metadata for "About" dialogs.

---

## Installation

Add `fltk-kit` to your `Cargo.toml`:

```toml
[dependencies]
fltk = "1.5.23"
fltk-table = "0.1"
fltk-kit = "0.5.4"
```
## Quick start
```rust
use fltk::{app, button, prelude::*, text::TextDisplay};
use fltk_kit::{AutoPage, Rect, Windows};

fn main() {
    let app = app::App::default();
    
    // 创建并自动最大化窗口
    let mut win = Windows::create_maximized(600, 400, "Layout Demo");

    // 初始化 AutoPage
    let mut page = AutoPage::new(0, 0, 600, 400, "");
    page.set_top(20);
    page.group().begin();

    let mut btn = button::Button::new(0, 0, 0, 0, "Click");
    let mut text = TextDisplay::new(0, 0, 0, 0, "Stretches automatically.");
    page.group().end(); 
    page.finish();

    // 核心：定义响应式布局
    page.on_layout(move |content: Rect| {
        let mut c = content;
        let mut bottom = c.cut_bottom(40);
        btn.resize(bottom.x, bottom.y, 80, bottom.h);
        text.resize(c.x, c.y, c.w, c.h);
    });

    // 拦截窗口拉伸
    win.resize_callback(move |w,_,_,_,_| { page.relayout(w.w(), w.h()); });
    app.run().unwrap();
}
```
Module Highlights
### 1. Responsive Layout (AutoPage & Rect)
FLTK lacks a modern layout engine. AutoPage lets you define layout rules via a closure, which automatically re-runs every time the window is resized.

### 2. CJK Fonts (Fonts)
```rust
use fltk_kit::Fonts;

// Automatically selects "Microsoft YaHei" on Windows or "PingFang SC" on macOS
Fonts::setup_auto(14); 
```
### 3. Form Validation (ValidatedInput)
```rust
use fltk::{app, input, prelude::*};
use fltk_kit::{ValidatedInput, Required};

let mut input = input::Input::new(10, 10, 200, 25, "Name:");
input.set_align(fltk::enums::Align::Left);

// Turns red and shows a tooltip if the user leaves it empty
input.bind_validator(Required); 
```
### 4. Data Binding (TableBindingExt / TreeBindingExt)
```rust
use fltk_table::{SmartTable, TableOpts};
use fltk_kit::TableBindingExt;

let data = vec![vec!["ID", "Name"], vec!["1", "Alice"]];
let mut table = SmartTable::default()
    .with_opts(TableOpts { rows: 2, cols: 2, ..Default::default() });

table.bind_data(&data);
```
### 5. Async Tasks (Task)
Safely update the UI after a long-running background task without freezing the main thread.

```rust
use fltk_kit::Task;

Task::run(0.05, 
    || { std::thread::sleep(std::time::Duration::from_secs(2)); "Done!" },
    move |msg| { println!("UI Thread received: {}", msg); }
);
```
### 6. Global Settings (Settings)
A robust TOML configuration manager supporting nested keys, arrays, and default values.

```rust
use fltk_kit::Settings;

let mut settings = Settings::load("my_app","setting.toml");
if settings.is_new() {
    settings.set_default("window.width", 800);
    settings.set_default("recent_files", vec!["a.txt", "b.txt"]);
    settings.save();
}

let width: i32 = settings.get("window.width").unwrap_or(800);
settings.set("window.width", 1024);
settings.save();
```
### 7. Loading Spinner (Spinner)
A pure code-drawn loading indicator that doesn't rely on GIFs or external assets.

```rust
use fltk_kit::Spinner;

let mut spinner = Spinner::new(10, 10, 50, 50);
spinner.show(); // Starts rotating automatically
```
Exe Icons (Windows)
To set your .exe icon, add winres as a build dependency and create a build.rs in your project root:

```toml
[build-dependencies]
winres = "0.1"
rust
// build.rs
use std::path::Path;

fn main() {
    #[cfg(windows)]
    {
        if Path::new("assets/icon.ico").exists() {
            let mut res = winres::WindowsResource::new();
            res.set_icon("assets/icon.ico");
            
            // Automatically read Cargo.toml metadata
            if let Ok(desc) = std::env::var("CARGO_PKG_DESCRIPTION") { res.set("FileDescription", &desc); }
            if let Ok(name) = std::env::var("CARGO_PKG_NAME") { res.set("ProductName", &name); }
            if let Ok(ver) = std::env::var("CARGO_PKG_VERSION") { res.set("FileVersion", &ver); }
            
            res.compile().unwrap();
        }
    }
}
```
## API overview
### Widgets & Helpers
| Item | Description |
|------|-------------|
| `Rect` | Geometry primitive with `cut_top`, `cut_bottom`, etc. |
| `AutoPage` | Responsive container that triggers layout closures on resize. |
| `Windows` | Helpers to create properly maximizable and resizable windows. |
| `Dialogs` | Centered message/confirm dialogs relative to parent window. |
| `Fonts` | Cross-platform CJK font selector. |
| `Spinner` | Pure vector loading animation widget. |
| `Icon` | Helper to set window icons from Base64 or embedded PNG. |


### Extension Traits
| Trait | Description |
|-------|-------------|
| `RectExt` | Apply `Rect` to any widget (`widget.apply_rect(r)`). |
| `ValidatedInput` | Bind logical validators to input fields with visual feedback. |
| `TableBindingExt` | Fill `SmartTable` with a 2D slice in one line. |
| `TreeBindingExt` | Build tree hierarchies from `(parent, child)` tuples. |

### Async
| Item | Description |
|------|-------------|
| `Channel` | Thread-safe message passing to the UI thread. |
| `Task` | Spawn background jobs and deliver results on the UI thread. |

### Core
| Item | Description |
|------|-------------|
| `Settings` | Persistent TOML configuration manager with dot-notation support. |
| `AppInfo` | Extract `Cargo.toml` metadata for "About" pages. |

## Design notes
Thread Safety by Default: FLTK is single-threaded. Channel and Task enforce safe UI updates by routing all results back to the main event loop via timeouts.

Layout as Slicing: Rect cuts provide a declarative way to split real estate, avoiding the infamous "nested group" spaghetti code common in native FLTK.

CJK Awareness: GUI frameworks often struggle with mixed ASCII/CJK widths. Fonts and Text ensure that your strings and UI components resize predictably.

No Magic: The library doesn't hide FLTK. It extends it. You still use Button, Input, and Window directly.

## License
Licensed under either of:
Apache License, Version 2.0 (LICENSE-APACHE)
MIT license (LICENSE-MIT)
at your option.