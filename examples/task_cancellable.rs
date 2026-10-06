//! Cancellable task: a long computation that the user can stop.
//!
//! Run: cargo run --example task_cancellable

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_kit::{CancelToken, Dialogs, Task};

struct Ui {
    wind: Window,
    btn_run: Button,
    btn_cancel: Button,
    status: Frame,
    token: Option<CancelToken>,
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(560, 220).with_label("Cancellable task");

    let btn_run = Button::new(20, 20, 150, 30, "Run");
    let btn_cancel = Button::new(180, 20, 150, 30, "Cancel");
    let mut status = Frame::new(20, 80, 520, 100, "Idle");
    status.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    wind.end();
    wind.show();

    let ui = Rc::new(RefCell::new(Ui {
        wind,
        btn_run,
        btn_cancel,
        status,
        token: None,
    }));

    // ---- Run ----
    let ui_run = ui.clone();
    ui.borrow_mut().btn_run.set_callback(move |_| {
        {
            let u = ui_run.borrow();
            let mut s = u.status.clone();
            s.set_label("Running (click Cancel to stop)...");
            s.redraw();
        }

        let ui_done = ui_run.clone();
        let token = Task::run_cancellable(
            0.1,
            |cancel: &CancelToken| {
                let mut out = Vec::new();
                for i in 0..10_000u64 {
                    if cancel.is_cancelled() {
                        break;
                    }
                    thread::sleep(Duration::from_millis(1));
                    out.push(i);
                }
                out
            },
            move |out| {
                let u = ui_done.borrow();
                let mut s = u.status.clone();
                s.set_label(&format!("Finished with {} items", out.len()));
                s.redraw();

                Dialogs::success_center(
                    &u.wind,
                    &format!("Task returned {} items.", out.len()),
                );
            },
        );

        ui_run.borrow_mut().token = Some(token);
    });

    // ---- Cancel ----
    let ui_cancel = ui.clone();
    ui.borrow_mut().btn_cancel.set_callback(move |_| {
        let u = ui_cancel.borrow();
        if let Some(token) = &u.token {
            token.cancel();
            let mut s = u.status.clone();
            s.set_label("Cancellation requested.");
            s.redraw();
        }
    });

    app.run().unwrap();
}