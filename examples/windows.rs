//! Create a maximized window with an icon.
//!
//! Run: cargo run --example windows

use fltk::{app};
use fltk_kit::{Windows};

fn main() {
    let app = app::App::default();

    // Create and show maximized.
    let mut wind = Windows::create_maximized(800, 600, "Windows helper");

    // Set an icon (create assets/icon.png to test).
    // Icon::set_embd_png(&mut wind, include_bytes!("../assets/icon.png"));

    // Minimum size.
    Windows::set_min_size(&mut wind, 400, 300);

    app.run().unwrap();
}