// 运行命令:
// cargo run --example spinner

use fltk::{app, button, input, prelude::*, window};
use fltk_kit::{Spinner, Task};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(400, 350)
        .with_label("Interactive Spinner Demo")
        .center_screen();

    // 1. 创建一个输入框（用于测试任务期间是否卡顿）
    let input_field = input::Input::new(100, 30, 200, 30, "Type:");

    // 2. 创建 Spinner
    let mut spinner = Spinner::new(150, 90, 100, 100);
    spinner.hide();

    // 3. 创建启动按钮
    let mut btn = button::Button::new(150, 250, 100, 40, "Start Task");

    let mut spinner_clone = spinner.clone();
    let mut btn_clone = btn.clone();
    let input_clone = input_field.clone();

    btn.set_callback(move |_| {
        // 任务开始：显示动画，禁用按钮
        spinner_clone.show();
        btn_clone.deactivate();

        let mut spinner_in_task = spinner_clone.clone();
        let mut btn_in_task = btn_clone.clone();
        let mut input_in_task = input_clone.clone();

        // 启动后台任务
        Task::run(
            0.05,
            || {
                // 👇 这里是后台线程，绝对不要碰 UI
                // 模拟耗时 3 秒的复杂计算
                std::thread::sleep(std::time::Duration::from_secs(3));
                "Task Done!"
            },
            move |msg| {
                // 👇 这里是 UI 主线程（任务结束自动触发）
                println!("{}", msg);

                // 任务结束：隐藏动画，恢复输入框和按钮
                spinner_in_task.hide();
                btn_in_task.activate();
                input_in_task.activate();
            },
        );
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
