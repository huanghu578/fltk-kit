// 运行命令:
// cargo run --example about_page

use fltk::{app, group::Group, prelude::*, text::TextDisplay, window};
use fltk_kit::AppInfo;
// 引入 build.rs 生成的常量
include!(concat!(env!("OUT_DIR"), "/dependencies.rs"));
fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(500, 400)
        .with_label("About Page Demo")
        .center_screen();

    // 模拟一个 Group（可以作为 Tab 的一页）
    let mut group = Group::new(10, 20, 480, 360, "About");
    group.set_frame(fltk::enums::FrameType::BorderFrame);

    // 使用 TextDisplay 展示多行文本最方便
    let mut text_display = TextDisplay::new(20, 40, 460, 320, "");
    
    // 设置多行字符串
    let mut about_text = AppInfo::format_about();
    about_text.push_str("\n\nDependencies:\n");
    for dep in DEPENDENCIES {
        about_text.push_str(&format!("{}\n", dep));
    }

    text_display.set_buffer(Some(fltk::text::TextBuffer::default()));
    
    if let Some(mut buf) = text_display.buffer() {
        buf.set_text(&about_text);
    }
    
    // 如果喜欢大标题，还可以单独加一个 Frame 显示 AppInfo::NAME
    // let mut title = frame::Frame::new(20, 20, 460, 40, AppInfo::NAME);
    // title.set_label_size(24);

    group.end();

    wind.end();
    wind.show();
    app.run().unwrap();
}