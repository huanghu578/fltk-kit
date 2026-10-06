//! Low-level `Channel`: send values from a worker thread to the UI.
//!
//! Run: cargo run --example channel_demo


use std::thread;
use std::time::Duration;

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_kit::Channel;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(500, 200).with_label("Channel demo");

    let mut btn_start = Button::new(20, 20, 150, 30, "Start");
    let status = Frame::new(20, 80, 460, 100, "Idle");
    status.clone().set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    wind.end();
    wind.show();

    // The UI side: create a channel and get a sender.
    let status_c = status.clone();
    let tx = Channel::spawn_poll(0.1, move |msg: String| {
        let mut s = status_c.clone();
        s.set_label(&msg);
        s.redraw();
    });

    // The worker side: on button click, spawn a thread that sends values.
    btn_start.set_callback(move |_| {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 1..=10 {
                thread::sleep(Duration::from_millis(200));
                let _ = tx.send(format!("Step {}/10", i));
            }
            let _ = tx.send("Done".to_string());
        });
    });

    app.run().unwrap();
}