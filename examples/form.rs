// 运行命令:
// cargo run --example form

use fltk::{app, enums, frame, input, prelude::*, window};
use fltk_table::{SmartTable, TableOpts};
use fltk_kit::{NumberRange, Required, TableBindingExt, Text, ValidatedInput};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(600, 500).center_screen();

    // 1. 表单
    let mut name_input = input::Input::new(80, 20, 140, 25, "Name:");
    name_input.set_align(enums::Align::Left);
    name_input.bind_validator(Required);

    let mut age_input = input::Input::new(80, 60, 140, 25, "Age (18-99):");
    age_input.set_align(enums::Align::Left);
    age_input.bind_validator(NumberRange { min: 18.0, max: 99.0 });

    let mut info_frame = frame::Frame::new(20, 100, 560, 20, "");
    info_frame.set_label(&format!(
        "Disk: {} | Rows: {}",
        Text::human_bytes(1_234_567),
        Text::human_count(12_345)
    ));
    info_frame.set_label_color(enums::Color::Dark3);

    // 2. 表格
    let data = vec![
        vec!["ID", "Name", "Role"],
        vec!["1", "Alice", "Admin"],
        vec!["2", "Bob", "User"],
        vec!["3", "Charlie", "Developer"],
    ];

    let mut table = SmartTable::default()
        .with_size(560, 300)
        .with_pos(20, 140)
        .with_opts(TableOpts {
            rows: data.len() as i32,
            cols: data[0].len() as i32,
            editable: false,
            ..Default::default()
        });

    table.set_col_header(false);
    table.set_row_header(false);
    table.set_row_header_width(0);
    table.set_col_header_height(0);

    table.bind_data(&data);

    wind.end();
    wind.show();
    app.run().unwrap();
}