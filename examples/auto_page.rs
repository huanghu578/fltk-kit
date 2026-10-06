//! Build an `AutoPage` whose layout is registered once and re-runs on resize.
//!
//! Run: cargo run --example auto_page

use std::rc::Rc;

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_kit::{AutoPage, RectExt};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(800, 600).with_label("AutoPage");

    let mut page = AutoPage::new(0, 0, 800, 600, "Table");
    page.group().begin();

    // These handles do not need `mut`: they are cloned into the layout
    // closure, and the clones are what get mutated.
    let tree = Frame::new(0, 0, 0, 0, "(tree)");
    let table = Frame::new(0, 0, 0, 0, "(table)");
    let btn_pick = Button::new(0, 0, 0, 0, "Pick");
    let btn_export = Button::new(0, 0, 0, 0, "Export");

    page.group().end();

    // Register the layout closure.
    page.on_layout({
        let (mut tree_c, mut table_c) = (tree.clone(), table.clone());
        let (mut pick_c, mut export_c) = (btn_pick.clone(), btn_export.clone());
        move |content| {
            let (content, btn_area) = content.cut_bottom(60);
            let buttons = btn_area.buttons_row(2, 200, 30, 20);
            buttons[0].apply_to(&mut pick_c);
            buttons[1].apply_to(&mut export_c);

            let (table_area, tree_area) = content.cut_left(250, 10);
            tree_area.apply_to(&mut tree_c);
            table_area.apply_to(&mut table_c);
        }
    });

    wind.end();
    wind.show();

    let page = Rc::new(page);
    page.relayout(wind.w(), wind.h());

    let page_cb = page.clone();
    wind.resize_callback(move |_, _, _, w, h| {
        page_cb.relayout(w, h);
    });

    app.run().unwrap();
}