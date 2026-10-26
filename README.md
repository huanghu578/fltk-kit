# fltk-kit 🧰

Reusable, high-level utilities and boilerplate reducers for [`fltk-rs`](https://github.com/fltk-rs/fltk-rs) applications.

FLTK is incredibly fast and lightweight, but it requires a lot of boilerplate for modern desktop app development. `fltk-kit` fills these gaps without forcing you into a heavy framework. It handles the painful parts: responsive layouts, CJK fonts, centered dialogs, thread-safe async updates, and cross-platform file handling.

## Features

- **Responsive Layouts**: `Rect` and `AutoPage` make complex window resizing as easy as slicing a cake.
- **CJK Font Handling**: `Fonts` automatically applies the correct system fonts to prevent Chinese/Japanese/Korean characters from showing as squares.
- **Window Helpers**: `Windows` handles the tricky initialization sequence for maximizable, resizable windows.
- **Thread-safe Async**: `Channel` and `Task` safely pass messages from background worker threads to the FLTK UI thread.
- **UI Extensions**: Bind validators to inputs (`ValidatedInput`), and fill `SmartTable` or `Tree` with data in a single line.
- **Icons & Spinner**: Embed window/exe icons (`Icon`, `build.rs`) and display a beautiful, zero-dependency loading spinner (`Spinner`).
- **Global Settings**: `Settings` provides a robust, dot-notation configuration manager with default values.
- **App Metadata**: `AppInfo` easily extracts `Cargo.toml` metadata for "About" dialogs.

## Installation

Add `fltk-kit` to your `Cargo.toml`:
```toml
[dependencies]
fltk-kit = "0.5.1"
```
Exe Icons (Windows)
To set your .exe icon, add a build.rs in your project root:
```rust
fn main() {
    if std::path::Path::new("assets/icon.ico").exists() {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}
```
(Requires winres in [build-dependencies])

### Quick Start Examples
#### 1. Responsive Layouts (AutoPage)
Stop calculating x, y, w, h manually. Slice the content area and let AutoPage handle the rest.

```rust
use fltk::{app, button, prelude::*, text::TextDisplay};
use fltk_kit::{AutoPage, Rect, Windows};

let mut win = Windows::create_maximized(600, 400, "Layout Demo");
let mut page = AutoPage::new(0, 0, 600, 400, "");
page.set_top(20);
page.group().begin();

let mut btn = button::Button::new(0, 0, 0, 0, "Click");
let mut text = TextDisplay::new(0, 0, 0, 0, "Stretches automatically.");
page.group().end(); page.finish();

page.on_layout(move |content: Rect| {
    let mut c = content;
    let mut bottom = c.cut_bottom(40);
    btn.resize(bottom.x, bottom.y, 80, bottom.h);
    text.resize(c.x, c.y, c.w, c.h);
});
```
#### 2. CJK Fonts (Fonts)
```rust
use fltk_kit::Fonts;

// Automatically selects "Microsoft YaHei" on Windows or "PingFang SC" on macOS
Fonts::setup_auto(14); 
```
#### 3. Form Validation (ValidatedInput)
```rust
use fltk::{input, prelude::*};
use fltk_kit::{ValidatedInput, Required};

let mut input = input::Input::new(10, 10, 200, 25, "Name:");
input.set_align(fltk::enums::Align::Left);
// Turns red and shows a tooltip if the user leaves it empty
input.bind_validator(Required); 
```
#### 4. Data Binding (TableBindingExt / TreeBindingExt)
```rust
use fltk_table::SmartTable;
use fltk_kit::TableBindingExt;

let data = vec![vec!["ID", "Name"], vec!["1", "Alice"]];
let mut table = SmartTable::default();
table.bind_data(&data);
```
#### 5. Async Tasks (Task)
Safely update the UI after a long-running background task.

```rust
use fltk_kit::Task;

Task::run(0.05, 
    || { std::thread::sleep(std::time::Duration::from_secs(2)); "Done!" },
    move |msg| { println!("UI Thread received: {}", msg); }
);
```
#### 6. Global Settings (Settings)
A robust TOML configuration manager supporting nested keys and arrays.

```rust
use fltk_kit::Settings;

let mut settings = Settings::load("my_app");
if settings.is_new() {
    settings.set_default("window.width", 800);
    settings.set_default("recent_files", vec!["a.txt", "b.txt"]);
    settings.save();
}

let width: i32 = settings.get("window.width").unwrap_or(800);
settings.set("window.width", 1024);
settings.save();
```
#### 7. About Dialogs & Spinner
```rust
use fltk_kit::{AppInfo, Spinner};

println!("App Name: {}", AppInfo::NAME);
println!("{}", AppInfo::format_about());

let mut spinner = Spinner::new(10, 10, 50, 50);
spinner.show(); // Displays a smooth, rotating loading spinner
```
##  Examples
Run the included examples to see the library in action:

bash
cargo run --example layout
cargo run --example form_table
cargo run --example async_task
cargo run --example about
cargo run --example icons
## License
MIT OR Apache-2.0