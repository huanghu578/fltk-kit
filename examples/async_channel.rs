// 运行命令:
// cargo run --example async_channel

use fltk::{app, enums, frame, prelude::*, window};
use fltk_kit::Task;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 200).center_screen();
    let mut frame = frame::Frame::new(20, 20, 360, 50, "Waiting for background task...");
    frame.set_label_size(16);

    Task::run(
        0.05, // 轮询间隔
        || {
            // 后台线程：模拟耗时操作
            std::thread::sleep(std::time::Duration::from_secs(2));
            "Task Completed Safely!"
        },
        move |msg| {
            // UI 线程回调：安全更新控件
            frame.set_label(msg);
            frame.set_label_color(enums::Color::Green);
        },
    );

    wind.end();
    wind.show();
    app.run().unwrap();
}