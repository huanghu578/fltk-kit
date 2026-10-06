//! Build a Tree from a folder, collapse parents, respond to clicks.
//!
//! Run: cargo run --example tree_helper

use fltk::{app, prelude::*, tree::Tree, window::Window};
use fltk_kit::TreeHelper;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 500).with_label("TreeHelper");

    let mut tree = Tree::new(10, 10, 380, 480, "");

    // Example: scan the current directory for .dwg / .dxf files.
    let dir = std::env::current_dir().unwrap();
    let folder_name = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("folder")
        .to_string();

    // `Vec` avoids the array-to-slice coercion that rust-analyzer misreports.
    let exts = vec!["dwg", "dxf"];
    let dir_str = dir.to_str().unwrap_or(".");

    let files = TreeHelper::scan_files(dir_str, &exts);

    println!("Scanned {} files in {:?}", files.len(), dir);

    TreeHelper::rebuild_from_files(&mut tree, &folder_name, &files);

    tree.set_callback(|t| {
        if let Some(path) = TreeHelper::selected_path(t) {
            println!("Selected: {}", path);
        }
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}