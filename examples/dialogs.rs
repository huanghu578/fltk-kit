// 运行命令:
// cargo run --example dialogs

use fltk::{app, button, prelude::*, window};
use fltk_kit::Dialogs;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(400, 300)
        .center_screen();
    let mut btn = button::Button::new(150, 130, 100, 40, "Click Me");

    let mut win_clone = wind.clone();
    btn.set_callback(move |_| {
        // 以父窗口居中弹出确认框
        if Dialogs::confirm_center(&win_clone, "Do you want to close this window?") {
            win_clone.hide();
            // 优雅退出
            std::process::exit(0);
        }
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
