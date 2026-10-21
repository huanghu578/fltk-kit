// 运行命令:
// cargo run --example icon --features icon

use fltk::{app, frame, prelude::*, window};
use fltk_kit::Icon;

// 1x1 红色像素 PNG（有效 Base64）
const PNG_B64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(300, 200).with_label("Icon Demo").center_screen();

    // 设置窗口图标
    Icon::set_base64_png(&mut wind, PNG_B64);

    let mut frame = frame::Frame::new(20, 20, 260, 160, "");
    // 在 Frame 中显示
    Icon::show_base64_in_frame(&mut frame, PNG_B64);

    wind.end();
    wind.show();
    app.run().unwrap();
}