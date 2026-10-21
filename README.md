# fltk-kit 🧰

Reusable utilities and boilerplate reducers for [`fltk-rs`](https://github.com/fltk-rs/fltk-rs) applications.

`fltk-kit` fills the gaps that almost every FLTK project ends up writing by hand: geometry helpers, layout management, form validation, table/tree data binding, dialog wrappers, icon loading, background tasks, window persistence, and file system watching.

## Architecture

The library is organized into three distinct layers:

| Layer | Purpose | FLTK dependency |
|-------|---------|-----------------|
| `core` | Pure logic: geometry, validators, string utilities | None |
| `widget` | FLTK widget helpers, extension traits, and UI components | Yes |
| `async_kit` | Channels and background task utilities | Yes (for UI polling) |

## Features

### Core Layer
- **`Rect`** — Geometry primitives with `cut_*` layout helpers.
- **`Theme`** — Shared spacing, size, and font constants.
- **`Text` / `Paths`** — CJK-aware string truncation, human-readable bytes/counts, path helpers.
- **`Validator`** — Pure logic validators (`Required`, `NumberRange`) decoupled from UI.

### Widget Layer
- **`RectExt`** — Apply `core::Rect` directly to any FLTK widget (`btn.apply_rect(rect)`).
- **`ValidatedInput`** — Bind validators to `Input` widgets. Automatically turns text red and shows a tooltip on error.
- **`TableBindingExt`** — Fill a `SmartTable` with 2D slices in one line (`table.bind_data(&data)`).
- **`TreeBindingExt`** — Build hierarchical `Tree` structures from a list of parent-child pairs.
- **`Fonts`** — Three-tier font selection (user preference → system language → fallback).
- **`Dialogs`** — Centered message, confirm, input, and file dialogs relative to the parent window.
- **`AutoPage`** — Stores a layout closure and re-lays out on window resize.
- **`TreeHelper` / `TableHelper`** — Common complex operations for Tree and SmartTable (e.g., scanning directories, ndarray binding).
- **`Windows` / `Shortcuts`** — Window creation helpers, global hotkeys, and `parse_shortcut("Ctrl+S")`.

### Async Layer
- **`Channel`** — Thread-safe communication bridging worker threads to the FLTK UI thread.
- **`Task` / `CancelToken`** — Spawn background jobs and safely deliver results (or check for cancellation) on the UI thread.

### Optional Features
- **`Persist`** (`persist` feature) — Save and restore window position and size using TOML.
- **`FileWatcher`** (`fs-watch` feature) — Monitor file system changes and safely forward events to the UI thread.
- **`Icon`** (`icon` feature) — Set window icons from embedded PNG/ICO bytes, Base64 strings, or display them in a `Frame`.

## Installation

```toml
[dependencies]
fltk-kit = { version = "0.2", features = ["full"] }
Feature flags
Feature	Enables	Dependencies
persist	Persist (window state)	serde, toml
fs-watch	FileWatcher	notify
icon	Icon (Base64 image helpers)	base64
full	All of the above	—
Quick Start
1. Responsive Layout (Rect)
rust
use fltk_kit::{Rect, RectExt};

let mut space = Rect::new(0, 0, 800, 600);
let top = space.cut_top(50); // Cut top 50px
btn.apply_rect(top);         // Apply to widget
2. Form Validation (ValidatedInput)
rust
use fltk_kit::{ValidatedInput, Required};

let mut input = fltk::input::Input::new(10, 10, 200, 25, "Name:");
input.bind_validator(Required); // Turns red if empty
3. Data Binding (TableBindingExt & TreeBindingExt)
rust
use fltk_table::SmartTable;
use fltk_kit::TableBindingExt;

let data = vec![
    vec!["ID", "Name"], 
    vec!["1", "Alice"]
];
let mut table = SmartTable::default();
table.bind_data(&data);
4. Text Utilities
rust
use fltk_kit::Text;

assert_eq!(Text::truncate_display("Hello 世界", 8), "Hello 世…");
assert_eq!(Text::human_bytes(1024), "1.0 KB");
assert_eq!(Text::human_count(1_234_567), "1,234,567");
5. Background Work (Task)
rust
use fltk_kit::Task;

Task::run(
    0.05, // polling interval
    move || { /* heavy work in background */ 42 },
    move |result| { 
        // Runs on UI thread safely!
        println!("{}", result); 
    },
);
6. File Watching (fs-watch)
rust
#[cfg(feature = "fs-watch")]
{
    use fltk_kit::{Channel, FileWatcher};

    let (tx, rx) = Channel::new();
    let _watcher = FileWatcher::watch("config.toml", move || { let _ = tx.send(()); })?;
    
    // Poll on UI thread
    rx.spawn_poll(0.05, move |_| { /* reload config */ });
}
7. Window Persistence (persist)
rust
#[cfg(feature = "persist")]
{
    use fltk_kit::Persist;

    Persist::restore(&mut window, "my_app");       // before show()
    window.set_callback(move |w| {
        Persist::save(w, "my_app");                 // on close
        w.hide();
    });
}
Examples
Run the included examples to see the library in action:

bash
cargo run --example basic
cargo run --example layout
cargo run --example form
cargo run --example table
cargo run --example tree
cargo run --example async_channel
cargo run --example dialogs

# Examples requiring optional features:
cargo run --example fs_watch --features fs-watch
cargo run --example icon --features icon
cargo run --example persist --features persist
License
MIT OR Apache-2.0