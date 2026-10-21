use fltk_kit::Rect;

#[test]
fn test_cut_top() {
    let mut r = Rect::new(0, 0, 100, 100);
    let top = r.cut_top(30);
    assert_eq!(top, Rect::new(0, 0, 100, 30));
    assert_eq!(r, Rect::new(0, 30, 100, 70));
}

#[test]
fn test_cut_bottom() {
    let mut r = Rect::new(0, 0, 100, 100);
    let bottom = r.cut_bottom(30);
    assert_eq!(bottom, Rect::new(0, 70, 100, 30));
    assert_eq!(r, Rect::new(0, 0, 100, 70));
}

#[test]
fn test_cut_left() {
    let mut r = Rect::new(0, 0, 100, 100);
    let left = r.cut_left(40);
    assert_eq!(left, Rect::new(0, 0, 40, 100));
    assert_eq!(r, Rect::new(40, 0, 60, 100));
}

#[test]
fn test_cut_right() {
    let mut r = Rect::new(0, 0, 100, 100);
    let right = r.cut_right(40);
    assert_eq!(right, Rect::new(60, 0, 40, 100));
    assert_eq!(r, Rect::new(0, 0, 60, 100));
}

#[test]
fn test_cut_exceeding_limits() {
    let mut r = Rect::new(0, 0, 50, 50);
    let top = r.cut_top(100); // 超过高度
    assert_eq!(top, Rect::new(0, 0, 50, 50));
    assert_eq!(r, Rect::new(0, 50, 50, 0));
}