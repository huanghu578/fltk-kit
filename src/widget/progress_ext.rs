//! Extension trait for linking a ProgressBar to an async Channel.
use fltk::misc::Progress;
use crate::async_kit::channel::Channel;

pub trait ProgressBarExt {
    /// Listens to a channel of `f64` (0.0 to 100.0) and updates the progress bar.
    fn bind_progress<F>(&mut self, rx: Channel<f64>, on_complete: F)
    where F: FnOnce() + 'static;
}

impl ProgressBarExt for Progress {
    fn bind_progress<F>(&mut self, rx: Channel<f64>, on_complete: F)
    where F: FnOnce() + 'static 
    {
        let mut pb = self.clone();
        let mut on_complete = Some(on_complete);

        rx.spawn_poll(0.05, move |val| {
            pb.set_value(val);
            if val >= 100.0 {
                if let Some(cb) = on_complete.take() {
                    cb();
                }
            }
        });
    }
}