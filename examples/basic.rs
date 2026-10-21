// 运行命令:
// cargo run --example basic

use fltk::{app, prelude::*};
use fltk_kit::{Fonts, Windows};

fn main() {
    let app = app::App::default();

    // 三层字体选择机制：用户指定 -> 系统语言 -> 回退
    let _font_setup = Fonts::setup_auto(Some(&["Microsoft YaHei"]), Some("zh-CN"), 14);

    // 创建并居中窗口
    let mut wind = Windows::create_maximized(800, 600, "FLTK-Kit Basic 例子");

    wind.end();
    wind.show();
    app.run().unwrap();
}