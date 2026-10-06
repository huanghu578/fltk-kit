//! Fill a SmartTable from an `Array2<String>`.
//!
//! Run: cargo run --example table_helper

use fltk::{app, prelude::*, window::Window};
use fltk_kit::TableHelper;
use ndarray::Array2;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(800, 400).with_label("TableHelper");

    let mut table = TableHelper::new_standard(10, 10, 780, 380);

    // Build data: header row + 20 data rows.
    let mut data = Array2::<String>::from_elem((21, 4), String::new());
    let headers = ["Name", "Size", "Type", "Notes"];
    for (i, h) in headers.iter().enumerate() {
        data[[0, i]] = h.to_string();
    }
    for r in 1..21 {
        data[[r, 0]] = format!("Item {}", r);
        data[[r, 1]] = format!("{}", r * 100);
        data[[r, 2]] = "file".to_string();
        data[[r, 3]] = format!("note {}", r);
    }

    TableHelper::fill_from_array(&mut table, &data);

    wind.end();
    wind.show();
    app.run().unwrap();
}