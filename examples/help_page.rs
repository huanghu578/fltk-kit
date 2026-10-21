// 运行命令:
// cargo run --example help_page

use fltk::{app, prelude::*, window};
use fltk_kit::HelpPage;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(500, 400)
        .with_label("Help Demo")
        .center_screen();

    // 创建帮助页面，指定要打开的文件名
    // 你可以在项目根目录放一个 help.html 或 help.pdf
    let _help_page = HelpPage::new(10, 10, 480, 380, "help.html");

    wind.end();
    wind.show();
    app.run().unwrap();
}