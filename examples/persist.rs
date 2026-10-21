// 运行命令:
// cargo run --example persist --features persist

use fltk::{app, prelude::*, window};
use fltk_kit::Persist;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300).with_label("Persist Demo");
    
    // 启动前恢复上次关闭时的位置和大小
    Persist::restore(&mut wind, "my_app_key");

    // 关闭时自动保存
    wind.set_callback(move |w| {
        Persist::save(w, "my_app_key");
        w.hide();
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}