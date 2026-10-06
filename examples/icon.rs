//! Set the window icon from embedded bytes and show a Base64 image in a Frame.
//!
//! Run: cargo run --example icon

use fltk::{app, frame::Frame,  prelude::*, window::Window};
use fltk_kit::Icon;

fn main() {
    let app = app::App::default();

    let mut wind = Window::default().with_size(600, 400).with_label("Icon demo");

    // 1. Window icon from embedded bytes (falls back silently if missing).
    // Create an `assets/icon.png` next to Cargo.toml to test.
    // Icon::set_embd_png(&mut wind, include_bytes!("../assets/icon.png"));

    // 2. Window icon from Base64 (no-op if the data is invalid).
    let b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+XkS8AAAAASUVORK5CYII=";
    Icon::set_base64_png(&mut wind, b64);

    // 3. Show an image in a Frame.
    let mut frame = Frame::new(20, 20, 200, 200, "");
    Icon::show_base64_in_frame(&mut frame, b64);

    // 4. Only generate the image.
    if let Some(_png) = Icon::png_from_bytes(include_bytes!("../assets/icon.png")) {
        // ...
    }

    wind.end();
    wind.show();
    app.run().unwrap();
}