//! Layout a page using `Rect` primitives only.
//!
//! Run: cargo run --example rect_layout

use fltk::{app, button::Button, frame::Frame, prelude::*, tree::Tree, window::Window};
use fltk_kit::{Rect, RectExt, Theme};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(800, 600).with_label("Rect layout");

    let tree = Tree::new(0, 0, 0, 0, "");
    let table = Frame::new(0, 0, 0, 0, "(table area)");
    let btn_pick = Button::new(0, 0, 0, 0, "Pick folder");
    let btn_export = Button::new(0, 0, 0, 0, "Export XLSX");

    wind.end();
    wind.show();

    // Clone widget handles for the layout closure.
    let (mut tree_c, mut table_c) = (tree.clone(), table.clone());
    let (mut pick_c, mut export_c) = (btn_pick.clone(), btn_export.clone());

    // The closure mutates the captured handles, so it must be `mut`.
    let mut layout = move |w: i32, h: i32| {
        let content = Rect::content(w, h, Theme::PAGE_TOP, Theme::BTN_H + 40, Theme::PAD_MD);

        let (content, btn_area) = content.cut_bottom(Theme::BTN_H + 30);
        let buttons = btn_area.buttons_row(2, 200, Theme::BTN_H, Theme::PAD_MD);
        buttons[0].apply_to(&mut pick_c);
        buttons[1].apply_to(&mut export_c);

        let (table_area, tree_area) = content.cut_left(250, Theme::PAD_MD);
        tree_area.apply_to(&mut tree_c);
        table_area.apply_to(&mut table_c);
    };

    // Initial layout.
    layout(wind.w(), wind.h());

    // Re-layout on resize.
    wind.resize_callback(move |_, _, _, w, h| {
        layout(w, h);
    });

    app.run().unwrap();
}