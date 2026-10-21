// 运行命令:
// cargo run --example state

use fltk::{app, group::{Group, Tabs}, input, prelude::*, window};
use fltk_table::{SmartTable, TableOpts};
use fltk_kit::{State, Persist};
use fltk_kit::widget::state_ext::{TabsStateExt, TableStateExt, save_table_widths};

fn main() {
    let app = app::App::default();
    State::init("my_app");

    let mut win = window::Window::default()
        .with_size(600, 400)
        .with_label("State Restore Demo")
        .center_screen();
    Persist::restore(&mut win, "main_win");

    // 1. 输入框
    let mut search = input::Input::new(90, 10, 200, 30, "Search:");
    search.set_align(fltk::enums::Align::Left);
    if let Some(last_search) = State::get::<String>("last_search") {
        println!("[启动] 读取到搜索词: {}", last_search);
        search.set_value(&last_search);
    }

    // 实时保存输入框
    search.set_callback(move |s| {
        State::set("last_search", s.value());
        State::save();
    });

    // 2. Tab 与 Table
    let mut tabs = Tabs::new(20, 50, 560, 330, "");
    
    let g1 = Group::new(20, 80, 560, 300, "Settings");
    g1.end();

    let g2 = Group::new(20, 80, 560, 300, "Data Table");
    let mut table = SmartTable::default()
        .with_size(540, 280)
        .with_pos(30, 90)
        .with_opts(TableOpts { rows: 4, cols: 3, ..Default::default() });
    table.set_col_header(true);
    table.set_col_header_value(0, "ID");
    table.set_cell_value(0, 0, "1");
    table.set_cell_value(0, 1, "Alice");
    g2.end();

    tabs = tabs.bind_state("active_tab");
    table = table.bind_col_widths_state("main_table_widths");
    tabs.end();

    // 3. 退出拦截：最后抓取一次表格和 Tab 状态
    let table_for_close = table.clone();
    let tabs_for_close = tabs.clone();
    let mut win_clone = win.clone();
    
    win.handle(move |_, ev| {
        if ev == fltk::enums::Event::Close {
            println!("[退出] 最后保存表格与 Tab...");
            save_table_widths(&table_for_close, "main_table_widths");
            
            if let Some(selected) = tabs_for_close.value() {
                for i in 0..tabs_for_close.children() {
                    if let Some(child) = tabs_for_close.child(i) {
                        if child.as_widget_ptr() == selected.as_widget_ptr() {
                            State::set("active_tab", i);
                            break;
                        }
                    }
                }
            }
            
            State::save();
            Persist::save(&win_clone, "main_win");
            win_clone.hide();
            fltk::app::quit();
            return true;
        }
        false
    });

    win.end();
    win.show();

    // 4. 🌟 终极恢复逻辑：双重延时强刷
    let mut tabs_late = tabs.clone();
    let mut table_late = table.clone();

    // 第一次强刷：0.5 秒后
    fltk::app::add_timeout3(0.5, move |_| {
        println!("[恢复] 第一波同步...");
        apply_state(&mut tabs_late, &mut table_late);

        // 第二次强刷：再过 0.2 秒，防止被底层重新排版覆盖
        let mut tabs_very_late = tabs_late.clone();
        let mut table_very_late = table_late.clone();
        fltk::app::add_timeout3(0.2, move |_| {
            println!("[恢复] 第二波强刷...");
            apply_state(&mut tabs_very_late, &mut table_very_late);
            fltk::app::awake(); // 强行唤醒主循环刷新
        });
    });

    app.run().unwrap();
}

/// 独立的状态应用函数，防止借用冲突
fn apply_state(tabs: &mut Tabs, table: &mut SmartTable) {
    // 恢复 Tab
    if let Some(idx) = State::get::<i32>("active_tab") {
        if idx >= 0 && idx < tabs.children() {
            if let Some(child) = tabs.child(idx) {
                let mut grp = unsafe { Group::from_widget_ptr(child.as_widget_ptr()) };
                let _ = tabs.set_value(&mut grp);
                tabs.do_callback(); // 核心：强行触发 Tab 的底层重绘和事件
                tabs.redraw();
            }
        }
    }

    // 恢复表格列宽
    if let Some(widths) = State::get::<Vec<i32>>("main_table_widths") {
        println!("[恢复] 正在应用表格宽度: {:?}", widths);
        for (c, w) in widths.iter().enumerate() {
            table.set_col_width(c as i32, *w);
        }
        table.redraw();
    }
}