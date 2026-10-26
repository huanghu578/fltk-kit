// 运行命令:
// cargo run --example i18n

use fltk::{app, button, prelude::*, window};
use fltk_kit::tr;
use fltk_kit::I18n;

fn main() {
    let app = app::App::default();

    // 1. 初始化 i18n (自动在当前目录生成 i18n.toml)
    I18n::init("i18n.toml");

    // 获取当前语言，用作窗口标题
    let current_lang = I18n::current_language();

    let mut wind = window::Window::default()
        .with_size(400, 200)
        .with_label(&format!("{} - [{}]", tr!("app.title"), current_lang))
        .center_screen();

    // 2. 使用 tr! 宏获取翻译
    let mut btn_hello = button::Button::new(150, 50, 100, 40, tr!("hello").as_str());
    btn_hello.set_label_size(20);

    // 切换语言按钮
    let mut btn_lang = button::Button::new(150, 110, 100, 30, "Switch to English");

    let mut wind_clone = wind.clone();
    btn_lang.set_callback(move |btn| {
        let current = I18n::current_language();
        let next_lang = if current == "zh-CN" { "en" } else { "zh-CN" };

        // 3. 切换语言并自动保存到文件
        I18n::set_language(next_lang);

        // 4. 刷新 UI 文本
        btn_hello.set_label(&tr!("hello"));
        wind_clone.set_label(&format!("{} - [{}]", tr!("app.title"), next_lang));

        btn.set_label(if next_lang == "zh-CN" {
            "Switch to English"
        } else {
            "切换到中文"
        });
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
