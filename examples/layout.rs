// 运行命令:
// cargo run --example layout

use fltk::{app, enums, frame, prelude::*, window};
use fltk_kit::{Rect, RectExt, Theme};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(Theme::WIN_W, Theme::WIN_H)
        .center_screen();
    
    let mut space = Rect::new(0, 0, wind.w(), wind.h());
    
    // 顶部 50px 切给工具栏
    let top_rect = space.cut_top(50);
    let mut top_bar = frame::Frame::new(0, 0, 0, 0, "Top Toolbar");
    top_bar.apply_rect(top_rect);
    top_bar.set_frame(enums::FrameType::UpBox);

    // 左侧 200px 切给侧边栏
    let left_rect = space.cut_left(200);
    let mut sidebar = frame::Frame::new(0, 0, 0, 0, "Sidebar");
    sidebar.apply_rect(left_rect);
    sidebar.set_frame(enums::FrameType::DownBox);

    // 剩下的全给主内容区
    let mut content = frame::Frame::new(0, 0, 0, 0, "Main Content");
    content.apply_rect(space);

    wind.end();
    wind.show();
    app.run().unwrap();
}