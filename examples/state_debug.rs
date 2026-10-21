use fltk::{app, prelude::*, window};
use fltk_kit::State;

fn main() {
    let app = app::App::default();
    State::init("debug_test"); // 换一个新的应用名
    println!("--- 启动 ---");
    
    // 1. 启动时读一下
    let val = State::get::<String>("my_key");
    println!("读取到的值: {:?}", val);

    let mut win = window::Window::default().with_size(300, 200).with_label("Debug");
    win.end();
    win.show();
    
    // 2. 点右上角 X 退出时保存
    let mut win_clone = win.clone();
    win.set_callback(move |_| {
        println!("准备保存...");
        State::set("my_key", "hello_state".to_string());
        State::save();
        println!("保存完毕！去 AppData 看看吧。");
        win_clone.hide();
    });

    app.run().unwrap();
}