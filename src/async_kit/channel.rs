//! Thread-safe channels for delivering values from worker threads to the
//! FLTK UI thread.
//!
//! FLTK is single-threaded: all widget updates must happen on the main
//! thread. `Channel` wraps `std::sync::mpsc` and provides a polling loop
//! driven by `fltk::app::add_timeout3`, so callbacks always run on the UI
//! thread.

use fltk::app;
use std::sync::mpsc::{channel as std_channel, Receiver, Sender as StdSender};

/// The sending half of a [`Channel`]. Clone freely; each clone is an
/// independent producer.
pub struct Sender<T>(StdSender<T>);

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Sender<T> {
    /// Send a value to the UI thread. Fails if the receiver has been
    /// dropped (i.e. the polling loop has stopped).
    pub fn send(&self, val: T) -> Result<(), std::sync::mpsc::SendError<T>> {
        self.0.send(val)
    }
}

/// The receiving half of a [`Channel`]. Owns the `Receiver` and drives
/// the polling loop.
pub struct Channel<T> {
    receiver: Receiver<T>,
}

impl<T: Send + 'static> Channel<T> {
    /// Creates a channel. Returns the sender and the channel itself.
    ///
    /// Call [`Channel::spawn_poll`] or [`Channel::spawn_poll_once`] on the
    /// returned channel to start delivering messages to the UI thread.
    pub fn new() -> (Sender<T>, Self) {
        let (tx, rx) = std_channel();
        (Sender(tx), Self { receiver: rx })
    }

    /// Polls the channel every `interval` seconds on the UI thread.
    ///
    /// `callback` runs for **every** message. Polling continues until all
    /// `Sender` clones have been dropped, at which point it stops
    /// automatically.
    pub fn spawn_poll<F>(self, interval: f64, mut callback: F)
    where
        F: FnMut(T) + 'static,
    {
        let receiver = self.receiver;
        app::add_timeout3(interval, move |handle| {
            let mut closed = false;
            loop {
                match receiver.try_recv() {
                    Ok(msg) => callback(msg),
                    Err(std::sync::mpsc::TryRecvError::Empty) => break,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        closed = true;
                        break;
                    }
                }
            }
            if !closed {
                app::repeat_timeout3(interval, handle);
            }
            // If closed, we simply don't repeat: the polling loop ends.
        });
    }

    /// Polls the channel every `interval` seconds and invokes `callback`
    /// **exactly once** with the first message received, then stops.
    ///
    /// If the `Sender` is dropped before any message arrives, the polling
    /// loop also stops without invoking `callback`.
    pub fn spawn_poll_once<F>(self, interval: f64, mut callback: F)
    where
        F: FnMut(T) + 'static,
    {
        let receiver = self.receiver;
        app::add_timeout3(interval, move |handle| {
            match receiver.try_recv() {
                Ok(msg) => {
                    callback(msg);
                    // Do NOT repeat: one-shot delivery is complete.
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    app::repeat_timeout3(interval, handle);
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    // Sender dropped without sending: stop silently.
                }
            }
        });
    }
}
