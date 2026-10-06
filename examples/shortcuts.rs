//! Per-widget and global shortcuts.
//!
//! Run: cargo run --example shortcuts

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_kit::{Dialogs, Shortcuts};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(500, 300).with_label("Shortcuts");

    let mut btn_save = Button::new(20, 20, 150, 30, "Save (Ctrl+S)");
    let mut btn_help = Button::new(20, 60, 150, 30, "Help (F1)");
    let status = Frame::new(20, 120, 460, 30, "Ready");
    status.clone().set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    wind.end();
    wind.show();

    // Per-widget shortcuts.
    Shortcuts::bind(&mut btn_save, Shortcuts::ctrl_s());
    Shortcuts::bind(&mut btn_help, Shortcuts::f1());

    // Button callbacks.
    let wind_save = wind.clone();
    btn_save.set_callback(move |_| {
        Dialogs::info_center(&wind_save, "Saved.");
    });
    let wind_help = wind.clone();
    btn_help.set_callback(move |_| {
        Dialogs::info_center(&wind_help, "Help.");
    });

    // Global shortcuts (not tied to any widget).
    let mut mgr = Shortcuts::manager();
    let wind_for_global = wind.clone();
    mgr.register(Shortcuts::f5(), move || {
        Dialogs::info_center(&wind_for_global, "F5 pressed (global).");
    });
    mgr.register(Shortcuts::ctrl_o(), {
        let w = wind.clone();
        move || {
            if let Some(p) = Dialogs::file_open_center("Open", None) {
                Dialogs::info_center(&w, &format!("Open: {:?}", p));
            }
        }
    });
    mgr.attach(&mut wind);

    app.run().unwrap();
}