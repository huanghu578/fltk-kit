//! Background task execution with UI-thread result delivery.
//!
//! FLTK is single-threaded: widget updates must happen on the UI thread.
//! `Task` runs a closure on a background thread and delivers its result
//! to a callback that is guaranteed to run on the UI thread.

use crate::async_kit::channel::Channel;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Cooperative cancellation token.
///
/// **Note**: `cancel()` merely sets a flag. The worker closure must
/// periodically call [`is_cancelled`] to actually stop. Rust cannot
/// forcibly terminate a running thread.
#[derive(Clone, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    /// Creates a new, uncancelled token.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancellation. Safe to call from any thread.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    /// Returns `true` if cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

pub struct Task;

impl Task {
    /// Runs `worker` on a background thread and delivers its result to
    /// `on_ui` on the UI thread.
    ///
    /// `interval` is the polling interval in seconds (e.g. `0.05`).
    /// The callback runs exactly once and then the polling loop stops.
    ///
    /// Returns immediately; the result is delivered asynchronously.
    pub fn run<W, R, C>(interval: f64, worker: W, on_ui: C)
    where
        W: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
        C: FnMut(R) + 'static,
    {
        let (tx, rx) = Channel::new();

        std::thread::spawn(move || {
            let result = worker();
            let _ = tx.send(result);
        });

        rx.spawn_poll_once(interval, on_ui);
    }

    /// Like [`Task::run`], but the worker receives a [`CancelToken`]
    /// that it can poll. Returns the token so the caller (typically on
    /// the UI thread, e.g. a "Cancel" button) can request cancellation.
    ///
    /// The worker only stops when it **checks** `token.is_cancelled()`.
    /// For long loops, check every iteration; for blocking I/O, check
    /// between I/O calls.
    pub fn run_cancellable<W, R, C>(interval: f64, worker: W, on_ui: C) -> CancelToken
    where
        W: FnOnce(&CancelToken) -> R + Send + 'static,
        R: Send + 'static,
        C: FnMut(R) + 'static,
    {
        let token = CancelToken::new();
        let worker_token = token.clone();

        let (tx, rx) = Channel::new();

        std::thread::spawn(move || {
            let result = worker(&worker_token);
            let _ = tx.send(result);
        });

        rx.spawn_poll_once(interval, on_ui);

        token
    }
}
