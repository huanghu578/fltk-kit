use fltk_kit::Channel;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn sender_delivers_values() {
    let (tx, rx) = mpsc::channel::<i32>();
    tx.send(42).unwrap();
    assert_eq!(rx.recv().unwrap(), 42);
}

#[test]
fn sender_clone_shares_underlying_channel() {
    let (tx, rx) = mpsc::channel::<i32>();
    let tx2 = tx.clone();
    tx.send(1).unwrap();
    tx2.send(2).unwrap();
    assert_eq!(rx.recv().unwrap(), 1);
    assert_eq!(rx.recv().unwrap(), 2);
}

#[test]
fn cancel_token_starts_uncancelled() {
    use fltk_kit::CancelToken;
    let token = CancelToken::new();
    assert!(!token.is_cancelled());
    token.cancel();
    assert!(token.is_cancelled());
}

#[test]
fn cancel_token_clone_shares_state() {
    use fltk_kit::CancelToken;
    let token = CancelToken::new();
    let clone = token.clone();
    token.cancel();
    assert!(clone.is_cancelled());
}

/// Ignored by default: requires a display and an active FLTK event loop.
#[test]
#[ignore]
fn channel_delivers_to_poll_callback() {
    use fltk::app;
    use std::cell::Cell;
    use std::rc::Rc;

    let app = app::App::default();
    let received = Rc::new(Cell::new(0));

    let received_for_cb = received.clone();
    let tx = Channel::spawn_poll(0.05, move |v: i32| {
        received_for_cb.set(v);
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        tx.send_or_warn(7);
    });

    // Stop the event loop after 200 ms.
    app::add_timeout3(0.2, |_handle| {
        app::quit();
    });
    app.run().unwrap();

    assert_eq!(received.get(), 7);
}