// 运行命令:
// cargo run --example full_features --features persist

use fltk::{app, button, input, misc::Progress, prelude::*, window};
use fltk_kit::{
    // 配置与窗口
    Settings, Persist,
    // UI 小工具
    Clipboard, Debouncer, ProgressBarExt, Theme,
    // 异步
    Channel, Task
};
use std::collections::HashMap;

fn main() {
    // 1. 初始化 i18n (模拟加载)
    

    // 2. 加载全局配置
    let mut settings = Settings::load("demo_app");
    
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(500, 400).with_label("Full Features");
    
    // 3. 恢复窗口位置（多屏保护版）
    Persist::restore(&mut wind, "main_win");

    // 4. 应用暗黑主题
    Theme::apply_dark(&mut wind);

    // 5. 防抖搜索输入框
    let mut search_input = input::Input::new(20, 20, 200, 30, "");
    search_input.set_callback(move |i| {
        let text = i.value();
        Debouncer::run(0.5, move || {
            if !text.is_empty() {
                println!("Executing search for: {}", text);
            }
        });
    });

    // 6. 进度条与异步任务
    let mut progress = Progress::new(20, 70, 460, 30, "");
    let mut start_btn = button::Button::new(20, 120, 150, 30, "hello");
    
    start_btn.set_callback(move |_| {
        let (tx, rx) = Channel::new();
        let mut pb = progress.clone();
        
        // 绑定进度条
        let mut pb_clone = pb.clone();
        pb.bind_progress(rx, move || {
            println!("Task Completed!");
        });

        // 启动后台任务
        Task::run(0.05, move || {
            for i in 0..=100 {
                std::thread::sleep(std::time::Duration::from_millis(30));
                let _ = tx.send(i as f64);
            }
        }, |_| {});
    });

    // 7. 剪贴板测试
    Clipboard::set_text("Copied from fltk-kit!");
    println!("Clipboard: {:?}", Clipboard::get_text());

    // 8. 保存配置与窗口
    wind.set_callback(move |w| {
        Persist::save(w, "main_win");
        settings.set("last_search", search_input.value());
        settings.save();
        w.hide();
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}