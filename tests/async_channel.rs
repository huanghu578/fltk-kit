use fltk_kit::{CancelToken, Channel};

#[test]
fn test_cancel_token() {
    let token = CancelToken::new();
    assert!(!token.is_cancelled());
    token.cancel();
    assert!(token.is_cancelled());
}

#[test]
fn test_channel_send() {
    let (tx, _rx) = Channel::<i32>::new();
    assert!(tx.send(42).is_ok());
}

#[test]
fn test_channel_send_after_drop() {
    let (tx, rx) = Channel::<i32>::new();
    drop(rx); // receiver dropped
    assert!(tx.send(42).is_err());
}