// 运行命令:
// cargo run --example fs_watch --features fs-watch

use fltk::{app, enums, frame, prelude::*, window};
use fltk_kit::{Channel, FileWatcher};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 200).center_screen();
    let mut frame = frame::Frame::new(20, 20, 360, 50, "Watching: config.toml");
    
    std::fs::write("config.toml", "initial").ok();

    let (tx, rx) = Channel::new();
    
    // 监听文件，变化时发送 () 到 Channel
    let _watcher = FileWatcher::watch("config.toml", move || {
        let _ = tx.send(());
    }).unwrap();

    // UI 轮询接收
    rx.spawn_poll(0.05, move |_| {
        frame.set_label("File modified! Reloaded.");
        frame.set_label_color(enums::Color::Red);
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}