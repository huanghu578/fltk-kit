// 运行命令:
// cargo run --example table

use fltk::{app, prelude::*, window};
use fltk_kit::widget::table::{TableBindingExt, TableHelper};
use fltk_table::{SmartTable, TableOpts};
use ndarray::Array2;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(600, 750)
        .with_label("Table Demo (3 Approaches)")
        .center_screen();

    // ==========================================
    // 1. 演示 TableBindingExt (普通二维数据)
    // ==========================================
    let data = vec![
        vec!["ID", "Name", "Role"],
        vec!["1", "Alice", "Admin"],
        vec!["2", "Bob", "User"],
        vec!["3", "Charlie", "Developer"],
    ];

    let mut table1 = SmartTable::default()
        .with_size(560, 180)
        .with_pos(20, 20)
        .with_opts(TableOpts {
            rows: data.len() as i32,
            cols: data[0].len() as i32,
            editable: false,
            ..Default::default()
        });

    // 强制关闭行头，并关闭默认列头（这样不会显示A, B, C）
    table1.set_row_header(false);
    table1.set_col_header(false);
    // 告诉底层，我们不需要行头标签
    table1.set_row_header_width(0);

    table1.bind_data(&data);

    // ==========================================
    // 2. 演示 TableHelper::fill_from_array (无表头 ndarray)
    // ==========================================
    let mut array_no_header = Array2::from_elem((3, 2), String::new());
    array_no_header[[0, 0]] = "Key".to_string();
    array_no_header[[0, 1]] = "Value".to_string();
    array_no_header[[1, 0]] = "Version".to_string();
    array_no_header[[1, 1]] = "0.2.0".to_string();
    array_no_header[[2, 0]] = "Status".to_string();
    array_no_header[[2, 1]] = "Stable".to_string();

    let mut table2 = TableHelper::new_standard(20, 230, 560, 180);
    TableHelper::fill_from_array(&mut table2, &array_no_header);

    // ==========================================
    // 3. 演示 TableHelper::fill_from_array_with_header (带表头 ndarray)
    // ==========================================
    let mut array_with_header = Array2::from_elem((4, 3), String::new());
    // 第一行作为列头（表头）
    array_with_header[[0, 0]] = "File Name".to_string();
    array_with_header[[0, 1]] = "Size".to_string();
    array_with_header[[0, 2]] = "Type".to_string();
    // 后续行作为数据
    array_with_header[[1, 0]] = "a.dwg".to_string();
    array_with_header[[1, 1]] = "2.5 MB".to_string();
    array_with_header[[1, 2]] = "CAD".to_string();

    array_with_header[[2, 0]] = "a.dxf".to_string();
    array_with_header[[2, 1]] = "1.8 MB".to_string();
    array_with_header[[2, 2]] = "CAD".to_string();

    array_with_header[[3, 0]] = "readme.txt".to_string();
    array_with_header[[3, 1]] = "4 KB".to_string();
    array_with_header[[3, 2]] = "Text".to_string();

    let mut table3 = TableHelper::new_standard(20, 440, 560, 180);
    TableHelper::fill_from_array_with_header(&mut table3, &array_with_header);

    // 收尾
    wind.end();
    wind.show();
    app.run().unwrap();
}
