// 运行命令:
// cargo run --example settings

use fltk::{app, button, frame, input, prelude::*, window};
use fltk_kit::Settings;

fn main() {
    let app = app::App::default();

    // 1. 加载配置
    // 会在类似 C:\Users\Administrator\AppData\Roaming\my_app\settings.toml 的地方生成文件
    let mut settings = Settings::load("my_app");

    let mut win = window::Window::default()
        .with_size(500, 300)
        .with_label("Settings Demo")
        .center_screen();

    let mut info_frame = frame::Frame::new(20, 20, 460, 40, "");
    info_frame.set_label_size(14);
    info_frame.set_label_color(fltk::enums::Color::Dark3);
    info_frame.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    // 2. 读取配置（带默认值兜底）
    // 如果之前没存过，默认主题是 "dark"，字体大小是 14，自动保存开启
    let theme: String = settings.get("theme").unwrap_or_else(|| "dark".to_string());
    let font_size: i32 = settings.get("font_size").unwrap_or(14);
    let auto_save: bool = settings.get("auto_save").unwrap_or(true);

    info_frame.set_label(&format!(
        "当前配置:\n主题: {}\n字体大小: {}\n自动保存: {}",
        theme, font_size, auto_save
    ));

    // 3. 模拟修改配置并保存
    let mut input_font = input::Input::new(200, 90, 100, 30, "New Font Size:");
    input_font.set_align(fltk::enums::Align::Left);

    let mut btn_save = button::Button::new(150, 150, 200, 40, "Save Settings");

    let mut info_clone = info_frame.clone();
    btn_save.set_callback(move |_| {
        // 读取用户输入并转换
        if let Ok(new_size) = input_font.value().parse::<i32>() {
            settings.set("font_size", new_size);
            settings.set("last_updated", "just now");
            
            // 写入磁盘！
            settings.save();

            // 更新 UI 展示，证明已经生效
            info_clone.set_label(&format!(
                "✅ 已保存并重新加载!\n当前字体大小: {}\n(看控制台或本地文件)",
                new_size
            ));
            info_clone.set_label_color(fltk::enums::Color::Green);
        } else {
            info_clone.set_label("❌ 请输入有效的数字!");
            info_clone.set_label_color(fltk::enums::Color::Red);
        }
    });

    win.end();
    win.show();

    println!("请去修改文件测试，或者直接运行后输入数字保存。");
    app.run().unwrap();
}