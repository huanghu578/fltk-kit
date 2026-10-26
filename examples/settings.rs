// 运行命令:
// cargo run --example settings

use fltk::{app, button, frame, input, prelude::*, window};
use fltk_kit::Settings;
use serde::{Deserialize, Serialize};

// 定义一个结构体用于测试结构体数组
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let app = app::App::default();

    // ==========================================
    // 1. 加载配置与首次运行判断
    // ==========================================
    let mut settings = Settings::load("my_app");

    if settings.is_new() {
        println!("[首次运行] 正在生成默认配置...");
        // 基础类型
        settings.set_default("theme", "Dark");

        // 嵌套点号语法
        settings.set_default("window.width", 1024);
        settings.set_default("window.height", 768);

        // 数组类型
        settings.set_default("recent_files", vec!["doc1.txt", "doc2.txt"]);

        // 结构体数组
        let default_users = vec![
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
        ];
        settings.set_default("users", default_users);

        settings.save(); // 落盘
    } else {
        println!("[日常运行] 已加载本地配置文件。");
    }

    // ==========================================
    // 2. 从磁盘读取各类数据
    // ==========================================
    let theme: String = settings.get("theme").unwrap_or_else(|| "Light".to_string());
    let width: i32 = settings.get("window.width").unwrap_or(800);
    let recent_files: Vec<String> = settings.get("recent_files").unwrap_or_default();
    let users: Vec<User> = settings.get("users").unwrap_or_default();

    // ==========================================
    // 3. 构建 UI 显示读取到的数据
    // ==========================================
    let mut win = window::Window::default()
        .with_size(500, 400)
        .with_label("Settings Read/Write Demo")
        .center_screen();

    let mut info_frame = frame::Frame::new(20, 20, 460, 160, "");
    info_frame.set_label_size(14);
    info_frame.set_align(fltk::enums::Align::TopLeft | fltk::enums::Align::Inside);

    let mut user_str = String::new();
    for u in &users {
        user_str.push_str(&format!("  - ID: {}, Name: {}\n", u.id, u.name));
    }

    info_frame.set_label(&format!(
        "当前配置读取结果:\n\n主题: {}\n窗口宽度: {}\n最近文件: {:?}\n\n用户列表:\n{}",
        theme, width, recent_files, user_str
    ));

    // ==========================================
    // 4. 模拟用户修改配置
    // ==========================================
    let mut input_width = input::Input::new(150, 200, 100, 30, "新窗口宽度:");
    input_width.set_align(fltk::enums::Align::Left);
    input_width.set_value(&width.to_string());

    let mut btn_save = button::Button::new(150, 260, 200, 40, "保存并新增用户");

    let mut status_label = frame::Frame::new(20, 320, 460, 40, "");
    status_label.set_label_color(fltk::enums::Color::Dark3);
    status_label.set_align(fltk::enums::Align::Center);

    let mut info_clone = info_frame.clone();
    let mut status_clone = status_label.clone();

    btn_save.set_callback(move |_| {
        // 修改单值
        if let Ok(new_width) = input_width.value().parse::<i32>() {
            settings.set("window.width", new_width);
        }

        // 修改数组：模拟增加了一个用户
        let mut current_users: Vec<User> = settings.get("users").unwrap_or_default();
        current_users.push(User {
            id: current_users.len() as u32 + 1,
            name: "Charlie".to_string(),
        });
        settings.set("users", current_users.clone());

        // 修改嵌套的深层值
        settings.set("window.position.x", 100);
        settings.set("window.position.y", 100);

        // 保存到磁盘
        settings.save();

        // 更新界面反馈
        status_clone.set_label("✅ 已保存到 settings.toml，请去文件查看嵌套结构！");
        status_clone.set_label_color(fltk::enums::Color::Green);

        // 刷新展示面板
        let mut user_str = String::new();
        for u in &current_users {
            user_str.push_str(&format!("  - ID: {}, Name: {}\n", u.id, u.name));
        }
        info_clone.set_label(&format!(
            "当前配置读取结果:\n\n主题: {}\n窗口宽度: {}\n最近文件: {:?}\n\n用户列表:\n{}",
            theme,
            input_width.value(),
            recent_files,
            user_str
        ));
    });

    win.end();
    win.show();
    app.run().unwrap();
}
