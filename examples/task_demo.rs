//! High-level `Task`: run a slow computation and update the UI.
//!
//! Run: cargo run --example task_demo

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_kit::{Dialogs, Task};

/// Simulated slow computation: return the squares of 0..n.
fn heavy_compute(n: u64) -> Vec<u64> {
    let mut out = Vec::with_capacity(n as usize);
    for i in 0..n {
        thread::sleep(Duration::from_millis(2));
        out.push(i * i);
    }
    out
}

struct Ui {
    wind: Window,
    btn_run: Button,
    status: Frame,
    result: Frame,
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(500, 300).with_label("Task demo");

    let btn_run = Button::new(20, 20, 150, 30, "Run 500 items");
    let mut status = Frame::new(20, 80, 460, 40, "Idle");
    status.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);
    let mut result = Frame::new(20, 130, 460, 150, "");
    result.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    wind.end();
    wind.show();

    let ui = Rc::new(RefCell::new(Ui { wind, btn_run, status, result }));

    let ui_run = ui.clone();
    ui.borrow_mut().btn_run.set_callback(move |_| {
        // 1. Update the status label on the UI thread.
        {
            let u = ui_run.borrow();
            let mut s = u.status.clone();
            s.set_label("Running...");
            s.redraw();
        }

        // 2. Start the background task.
        let ui_done = ui_run.clone();
        Task::run(
            0.1,
            || heavy_compute(500),
            move |squares| {
                let u = ui_done.borrow();
                let mut s = u.status.clone();
                let mut r = u.result.clone();

                let preview: Vec<String> = squares
                    .iter()
                    .take(20)
                    .map(|v| v.to_string())
                    .collect();
                let text = if squares.len() > 20 {
                    format!("{} ... (total {})", preview.join(", "), squares.len())
                } else {
                    preview.join(", ")
                };

                s.set_label(&format!("Done: {} items", squares.len()));
                r.set_label(&text);
                s.redraw();
                r.redraw();

                Dialogs::success_center(
                    &u.wind,
                    &format!("Computed {} items.", squares.len()),
                );
            },
        );
    });

    app.run().unwrap();
}