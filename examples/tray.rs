// 运行命令:
// cargo run --example tray --features tray

use fltk::{app, frame, prelude::*, window};
use fltk_kit::async_kit::channel::Channel;
use fltk_kit::widget::tray::TrayIcon;

fn main() {
    let _app = app::App::default();
    
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Tray Demo - Close me to hide")
        .center_screen();

    let mut label = frame::Frame::new(20, 20, 360, 200, "I am running in the background!");
    label.set_label_size(18);

    let (tx, rx) = Channel::new();

    // 1. 创建托盘
    let tx_clone = tx.clone();
    let tray_result = TrayIcon::create(
        "dp.ico",
        "fltk-kit Tray App",
        move || { let _ = tx_clone.send(()); },
        move || { std::process::exit(0); },
    );

    if let Err(e) = tray_result {
        println!("⚠️ 托盘创建失败: {}", e);
    }

    // 2. 监听托盘点击，恢复主窗口
    let mut win_clone = win.clone();
    rx.spawn_poll(0.1, move |_| {
        win_clone.show();
    });

    // 3. 拦截主窗口关闭，隐藏而不销毁
    let mut win_handle = win.clone();
    win.set_callback(move |_w| {
        win_handle.hide();
        println!("主窗口已隐藏到托盘。");
    });

    win.end();
    win.show();

    // 4. 关键：不使用 app.run()，而是用循环 + wait_for 维持事件循环
    // 这样即使主窗口隐藏了，程序也不会退出
    loop {
        if app::wait_for(0.1).is_err() {
            break;
        }
    }
}