// 运行命令:
// cargo run --example tree

use fltk::{app, prelude::*, tree::Tree, window};
use fltk_kit::widget::tree::{TreeBindingExt, TreeHelper};
use std::fs;

fn main() {
    // 准备测试数据
    let _ = fs::create_dir("test_dir");
    let _ = fs::write("test_dir/file1.txt", "test");
    let _ = fs::write("test_dir/file2.rs", "test");
    let _ = fs::write("test_dir/a.dwg", "test"); // 同名不同后缀
    let _ = fs::write("test_dir/a.dxf", "test"); // 同名不同后缀

    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(400, 500)
        .with_label("Tree Demo (3-Level)")
        .center_screen();

    let mut tree = Tree::new(10, 10, 380, 480, "");
    tree.set_show_root(true);

    // 1. 绑定基础数据
    let data = vec![
        ("ROOT".to_string(), "Fruits".to_string()),
        ("ROOT".to_string(), "Animals".to_string()),
        ("Fruits".to_string(), "Apple".to_string()),
        ("Fruits".to_string(), "Banana".to_string()),
        ("Animals".to_string(), "Dog".to_string()),
        ("Animals".to_string(), "Cat".to_string()),
    ];
    tree.bind_data(&data);

    // 2. 追加 TreeHelper 扫描结果 (展示同名不同后缀)
    let files = TreeHelper::scan_files_with_ext("test_dir", &["dwg", "dxf"]);
    if !files.is_empty() {
        let _ = tree.add("CAD Files");
        for file in files {
            let path = format!("CAD Files/{}", file);
            let _ = tree.add(&path);
        }
    }

    // 3. 再追加一个普通扫描 (无后缀)
    let text_files = TreeHelper::scan_files("test_dir", &["txt", "rs"]);
    if !text_files.is_empty() {
        let _ = tree.add("Source Files");
        for file in text_files {
            let path = format!("Source Files/{}", file);
            let _ = tree.add(&path);
        }
    }

    // 4. 清理并展开节点
    let _ = fs::remove_dir_all("test_dir");

    let _ = tree.open("ROOT", false);
    let _ = tree.open("Fruits", false);
    let _ = tree.open("Animals", false);
    let _ = tree.open("CAD Files", false);
    let _ = tree.open("Source Files", false);

    wind.end();
    wind.show();
    app.run().unwrap();
}
