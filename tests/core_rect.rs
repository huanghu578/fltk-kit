use fltk_kit::Rect;

#[test]
fn cut_top_returns_remaining_and_cut() {
    let r = Rect::new(0, 0, 100, 200);
    let (rest, cut) = r.cut_top(50);
    assert_eq!(cut, Rect::new(0, 0, 100, 50));
    assert_eq!(rest, Rect::new(0, 50, 100, 150));
}

#[test]
fn cut_bottom_returns_remaining_and_cut() {
    let r = Rect::new(0, 0, 100, 200);
    let (rest, cut) = r.cut_bottom(30);
    assert_eq!(cut, Rect::new(0, 170, 100, 30));
    assert_eq!(rest, Rect::new(0, 0, 100, 170));
}

#[test]
fn cut_left_with_gap() {
    let r = Rect::new(0, 0, 100, 50);
    let (rest, cut) = r.cut_left(30, 10);
    assert_eq!(cut, Rect::new(0, 0, 30, 50));
    assert_eq!(rest, Rect::new(40, 0, 60, 50));
}

#[test]
fn cut_right_with_gap() {
    let r = Rect::new(0, 0, 100, 50);
    let (rest, cut) = r.cut_right(30, 10);
    assert_eq!(cut, Rect::new(70, 0, 30, 50));
    assert_eq!(rest, Rect::new(0, 0, 60, 50));
}

#[test]
fn split_h_ratio() {
    let r = Rect::new(0, 0, 100, 50);
    let (left, right) = r.split_h(0.5, 10);
    assert_eq!(left, Rect::new(0, 0, 45, 50));
    assert_eq!(right, Rect::new(55, 0, 45, 50));
}

#[test]
fn buttons_row_centers_and_spaces() {
    let r = Rect::new(0, 0, 800, 60);
    let buttons = r.buttons_row(2, 280, 30, 20);
    assert_eq!(buttons.len(), 2);
    // total width = 280*2 + 20 = 580, start_x = (800-580)/2 = 110
    assert_eq!(buttons[0].x, 110);
    assert_eq!(buttons[1].x, 410);
    // vertical center: y = (60-30)/2 = 15
    assert_eq!(buttons[0].y, 15);
}

#[test]
fn buttons_row_empty() {
    let r = Rect::new(0, 0, 800, 60);
    assert!(r.buttons_row(0, 280, 30, 20).is_empty());
}

#[test]
fn form_row_label_left_field_right() {
    let r = Rect::new(0, 0, 400, 30);
    let (field, label) = r.form_row(100, 10);
    assert_eq!(label, Rect::new(0, 0, 100, 30));
    assert_eq!(field, Rect::new(110, 0, 290, 30));
}

#[test]
fn grid_cells() {
    let r = Rect::new(0, 0, 100, 100);
    let cells = r.grid(2, 2, 0);
    assert_eq!(cells.len(), 4);
    assert_eq!(cells[0], Rect::new(0, 0, 50, 50));
    assert_eq!(cells[3], Rect::new(50, 50, 50, 50));
}

#[test]
fn inset_shrinks_all_sides() {
    let r = Rect::new(10, 10, 100, 100);
    assert_eq!(r.inset(5), Rect::new(15, 15, 90, 90));
}