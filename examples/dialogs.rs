//! All dialog variants centered on the parent window.
//!
//! Run: cargo run --example dialogs

use fltk::{app, button::Button, group::Group, prelude::*, window::Window};
use fltk_kit::Dialogs;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 400).with_label("Dialogs");

    let col = Group::new(20, 20, 360, 360, "");
    col.begin();

    let mut info = Button::new(0, 0, 200, 30, "Info");
    let mut warning = Button::new(0, 40, 200, 30, "Warning");
    let mut error = Button::new(0, 80, 200, 30, "Error");
    let mut confirm = Button::new(0, 120, 200, 30, "Confirm");
    let mut ask = Button::new(0, 160, 200, 30, "Ask");
    let mut input = Button::new(0, 200, 200, 30, "Input");
    let mut password = Button::new(0, 240, 200, 30, "Password");
    let mut open = Button::new(0, 280, 200, 30, "Open file");
    let mut dir = Button::new(0, 320, 200, 30, "Choose folder");

    col.end();
    wind.end();
    wind.show();

    let wind_for_cb = wind.clone();

    info.set_callback({
        let w = wind_for_cb.clone();
        move |_| Dialogs::info_center(&w, "Info message.")
    });
    warning.set_callback({
        let w = wind_for_cb.clone();
        move |_| Dialogs::warning_center(&w, "Warning message.")
    });
    error.set_callback({
        let w = wind_for_cb.clone();
        move |_| Dialogs::error_center(&w, "Error message.")
    });
    confirm.set_callback({
        let w = wind_for_cb.clone();
        move |_| {
            let ok = Dialogs::confirm_center(&w, "Proceed?");
            println!("confirm: {}", ok);
        }
    });
    ask.set_callback({
        let w = wind_for_cb.clone();
        move |_| {
            let yes = Dialogs::ask_center(&w, "Overwrite?");
            println!("ask: {}", yes);
        }
    });
    input.set_callback({
        let w = wind_for_cb.clone();
        move |_| {
            if let Some(s) = Dialogs::input_center(&w, "Your name:", "") {
                println!("input: {}", s);
            }
        }
    });
    password.set_callback({
        let w = wind_for_cb.clone();
        move |_| {
            if let Some(s) = Dialogs::password_center(&w, "Password:", "") {
                println!("password: {}", s);
            }
        }
    });
    open.set_callback(move |_| {
        if let Some(p) = Dialogs::file_open_center("Open file", None) {
            println!("file: {:?}", p);
        }
    });
    dir.set_callback(move |_| {
        if let Some(p) = Dialogs::dir_center("Choose folder") {
            println!("dir: {:?}", p);
        }
    });

    app.run().unwrap();
}