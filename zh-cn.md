# fltk-kit 项目说明

## 一、项目概述

`fltk-kit` 是一个为 [fltk-rs](https://github.com/fltk-rs/fltk-rs) 应用提供通用工具的扩展库。它把 FLTK 项目中反复出现的样板代码集中封装起来，让业务代码只关注业务逻辑，不用每次重写字体设置、弹窗居中、异步通道、布局计算这些通用功能。

### 解决什么问题

写一个 FLTK 桌面应用，通常会遇到这些重复劳动：

- **布局**：FLTK 没有布局引擎，所有控件坐标手算，窗口缩放时还要重算。
- **中文字体**：默认字体不含中文字形，需要遍历系统字体找到可用的。
- **弹窗居中**：`dialog::message` 接收屏幕坐标，想让它出现在母窗口中间要自己算。
- **异步任务**：FLTK 是单线程 UI 模型，工作线程的结果要通过 `mpsc` + 定时轮询回传，样板代码一堆。
- **图标加载**：从 PNG、ICO、Base64、文件路径加载窗口图标，每个来源都要写一遍容错。

`fltk-kit` 把这些都封装成类型 + 关联函数，一行调用解决。

## 二、模块结构

库分为三层，依赖关系清晰：  

fltk-kit
├── core ← 纯逻辑，不依赖 FLTK  
│ ├── rect 几何计算（矩形切分、对齐、按钮排列）  
│ ├── theme 通用常量（间距、尺寸、字号）  
│ ├── text 文本处理（截断、字节格式化、千分位）  
│ └── paths 路径辅助（exe 目录、纯文件名、纯文件夹名）  
│  
├── widget ← FLTK 控件辅助  
│ ├── fonts 字体选择（用户 > 系统语言 > 兜底，三级优先级）  
│ ├── icon 窗口图标与图像加载（PNG/ICO/Base64/字节）  
│ ├── dialogs 居中弹窗与文件选择器  
│ ├── auto_page 自动缩放的页面容器  
│ ├── layout_ext Rect 到 FLTK 控件的桥接（RectExt trait）  
│ ├── tree Tree 控件辅助  
│ ├── table SmartTable 辅助  
│ ├── windows 窗口创建与最大化  
│ └── shortcuts 快捷键常量与全局管理器  
│  
└── async_kit ← 异步与多线程  
├── channel 工作线程 → UI 线程的消息通道  
└── task 后台任务（含取消）  

text

**分层的意义**：

- `core` 层不依赖 FLTK，可以被任何 Rust 项目复用，单元测试也容易写。
- `widget` 层依赖 `fltk`，提供 GUI 辅助。
- `async_kit` 层依赖 `fltk`，处理异步。

## 三、核心类型与用法

### 1. Rect — 布局的基础

`Rect` 是一个纯几何矩形 `(x, y, w, h)`，提供纯函数的切分操作，用于组合任意布局。

**基本构造**：

```rust
use fltk_kit::{Rect, Theme};

// 整个窗口
let win = Rect::window(800, 600);

// 页面内容区：顶部从 40 开始，底部预留 60，左右各留 10
let content = Rect::content(800, 600, 40, 60, Theme::PAD_MD);
切分操作（返回 (剩余, 切出)）：

rust
let (rest, top)    = content.cut_top(40);       // 从顶部切 40
let (rest, bottom) = content.cut_bottom(60);    // 从底部切 60
let (rest, left)   = content.cut_left(250, 10); // 从左边切 250，留 10 间距
let (rest, right)  = content.cut_right(200, 10); // 从右边切 200
常用模板：

rust
// 左固定右伸缩
let (right, left) = content.left_fixed(250, 10);

// 上固定下伸缩
let (bottom, top) = content.top_fixed(40, 10);

// 一行 N 个居中按钮
let buttons = btn_area.buttons_row(2, 280, 30, 20);
buttons[0].apply_to(&mut btn_save);
buttons[1].apply_to(&mut btn_cancel);

// 表单行：左标签右输入
let (field, label) = row.form_row(100, 10);

// 网格
let cells = content.grid(3, 4, 5);  // 3 行 4 列
应用到控件：

rust
use fltk_kit::RectExt;

tree_area.apply_to(&mut tree);
table_area.apply_to(&mut table);

// 批量
Rect::apply_all(&[rect1, rect2], &mut [&mut w1, &mut w2]);
关键点：Rect 本身不依赖 FLTK，apply_to 定义在 RectExt trait 上，所以 core::rect 保持纯净。

### 2. AutoPage — 自动布局的页面
AutoPage 封装一个 Group 和一个布局闭包。窗口缩放或页面切换时，调用 relayout 自动重排。

rust
use fltk::{prelude::*, tree::Tree, window::Window};
use fltk_kit::{AutoPage, RectExt};

let mut wind = Window::default().with_size(800, 600);
let mut page = AutoPage::new(0, 0, 800, 600, "表格");
page.group().begin();

let tree = Tree::new(0, 0, 0, 0, "");
let table = Tree::new(0, 0, 0, 0, "");
let btn = Button::new(0, 0, 0, 0, "确定");

page.group().end();

// 注册布局函数：每次 relayout 调用
page.on_layout({
    let (mut tree_c, mut table_c) = (tree.clone(), table.clone());
    let mut btn_c = btn.clone();
    move |content| {
        let (content, btn_area) = content.cut_bottom(60);
        let buttons = btn_area.buttons_row(1, 200, 30, 20);
        buttons[0].apply_to(&mut btn_c);

        let (table_area, tree_area) = content.cut_left(250, 10);
        tree_area.apply_to(&mut tree_c);
        table_area.apply_to(&mut table_c);
    }
});

// 窗口 resize 时重新布局
wind.resize_callback(move |_, _, _, w, h| {
    page.relayout(w, h);
});
关键点：on_layout 接受 FnMut(Rect)，因为布局闭包通常要 apply_to(&mut widget)。闭包捕获的控件句柄要声明为 mut，因为 apply_to 需要可变借用。

### 3. Fonts — 三级优先级的字体选择
按 用户指定 > 系统语言 > 兜底 的顺序找字体。

rust
use fltk_kit::{Fonts, FontSource};

let result = Fonts::setup_auto(
    Some(&["Source Han Sans", "Noto Sans CJK SC"]),  // 用户候选
    Some("zh-CN"),                                     // 系统语言
    14,                                                // 字号
);

match result.source {
    FontSource::User => println!("用了用户字体: {}", result.font_name.unwrap()),
    FontSource::System => println!("用了系统语言字体: {}", result.font_name.unwrap()),
    FontSource::Fallback => println!("用了兜底字体"),
    FontSource::Default => eprintln!("保持 FLTK 默认字体"),
}
内置候选列表：CANDIDATES_ZH_CN、CANDIDATES_ZH_TW、CANDIDATES_JA、CANDIDATES_KO、CANDIDATES_AR、CANDIDATES_LATIN，每个都覆盖 Windows / macOS / Linux 常见字体。

单独查找：

rust
if let Some((name, _font)) = Fonts::find_first(&["Consolas", "Monaco"]) {
    println!("找到等宽字体: {}", name);
}

if Fonts::is_available("Microsoft YaHei") {
    // ...
}

### 4. Icon — 窗口图标与图像  
从嵌入字节、Base64、文件路径加载图标或图像。所有 setter 返回 bool，失败时返回 false 而不 panic。

rust
use fltk_kit::Icon;

// 窗口图标
Icon::set_embd_png(&mut wind, include_bytes!("../assets/app.png"));
Icon::set_embd_ico(&mut wind, include_bytes!("../assets/app.ico"));
Icon::set_base64_png(&mut wind, ICON_B64);
Icon::set_file_png(&mut wind, "assets/icon.png");

// 显示 Base64 图像到 Frame，等比缩放到 Frame 尺寸
Icon::show_base64_in_frame(&mut frame, cover_b64);

// 只生成图像，自己控制
if let Some(img) = Icon::rgb_from_base64(b64) {
    let scaled = Icon::scale_rgb(img, 400, 300, true);
    frame.set_image(Some(scaled));
}

### 5. Dialogs — 居中弹窗
所有弹窗以母窗口为中心（不是屏幕）。传窗口引用即可。

rust
use fltk_kit::Dialogs;

// 信息类
Dialogs::info_center(&win, "操作完成");
Dialogs::success_center(&win, "导出成功");
Dialogs::warning_center(&win, "文件已存在");
Dialogs::error_center(&win, "保存失败");

// 确认类
if Dialogs::confirm_center(&win, "确定删除？") {
    // 用户点了 OK
}

if Dialogs::ask_center(&win, "覆盖已有文件？") {
    // 用户点了 Yes
}

// 输入类
if let Some(name) = Dialogs::input_center(&win, "图册名", "") {
    println!("输入: {}", name);
}

if let Some(pwd) = Dialogs::password_center(&win, "密码", "") {
    // ...
}

// 文件类（系统原生对话框，不需要窗口引用）
if let Some(path) = Dialogs::file_open_center("打开", Some("Excel\t*.xlsx")) {
    println!("选中: {:?}", path);
}

if let Some(dir) = Dialogs::dir_center("选择工作文件夹") {
    println!("目录: {:?}", dir);
}
关键点：Dialogs 用 win.x_root() 和 win.y_root() 获取窗口在屏幕上的位置，加上窗口尺寸计算对话框中心。这样多显示器、窗口拖动后都能正确居中。

### 6. TreeHelper / TableHelper — 控件辅助
TreeHelper：

rust
use fltk_kit::TreeHelper;

// 扫描文件夹中的指定扩展名文件
let exts: &[&str] = &["dwg", "dxf"];
let files = TreeHelper::scan_files("/path/to/dir", exts);

// 用文件夹名做根，文件列表做子节点
TreeHelper::rebuild_from_files(&mut tree, "我的文件夹", &files);

// 用 (父, 子) 路径列表重建，并折叠父节点
TreeHelper::rebuild_from_paths(&mut tree, &[
    ("图册A".to_string(), "图纸1".to_string()),
    ("图册A".to_string(), "图纸2".to_string()),
]);

// 获取当前选中节点的路径
if let Some(path) = TreeHelper::selected_path(&tree) {
    println!("选中: {}", path);
}
TableHelper：

rust
use fltk_kit::TableHelper;

// 创建标准配置的 SmartTable（只读、无表头、列宽可调）
let mut table = TableHelper::new_standard(10, 10, 780, 380);

// 从 Array2<String> 填充，自动均分列宽
TableHelper::fill_from_array(&mut table, &data);

// 设置表头
TableHelper::set_headers(&mut table, &["列1", "列2", "列3"]);

// 清空
TableHelper::clear(&mut table);

### 7. Windows — 窗口创建
rust
use fltk_kit::Windows;

// 创建窗口并最大化（正确的调用顺序）
let mut wind = Windows::create_maximized(800, 600, "我的应用");

// 显示并最大化已有窗口
Windows::show_maximized(&mut wind);

// 设置最小尺寸
Windows::set_min_size(&mut wind, 400, 300);

### 8. Shortcuts — 快捷键
按钮快捷键：

rust
use fltk_kit::Shortcuts;

Shortcuts::bind(&mut btn_save, Shortcuts::ctrl_s());
Shortcuts::bind(&mut btn_help, Shortcuts::f1());
全局快捷键（不绑定到控件）：

rust
let mut mgr = Shortcuts::manager();
mgr.register(Shortcuts::f1(), move || { /* 显示帮助 */ });
mgr.register(Shortcuts::f5(), move || { /* 刷新数据 */ });
mgr.register(Shortcuts::ctrl_o(), move || { /* 打开文件 */ });
mgr.attach(&mut wind);
预定义常量：f1() ~ f12()、ctrl_a() ~ ctrl_z()、ctrl_shift_s()、esc()、enter()、delete()、backspace()、alt(Key)、func(Key)。

### 9. Channel — 工作线程 → UI 线程
FLTK 是单线程 UI，工作线程不能直接更新控件。Channel 封装了标准的 mpsc + 定时轮询模式。

rust
use fltk_kit::Channel;
use std::thread;

// UI 线程：创建通道，注册回调
let tx = Channel::spawn_poll(0.1, {
    let ui = ui.clone();
    move |records: Vec<Record>| {
        ui.borrow_mut().show_records(&records);
    }
});

// 工作线程：算完发送
thread::spawn(move || {
    let records = load_records();
    tx.send_or_warn(records);
});
特性：

所有 Sender 被 drop 后，轮询自动停止，不空转。

回调被 catch_unwind 保护，一次 panic 不会停掉整个轮询。

send_or_warn 失败时打印警告，不 panic。

spawn_poll_with_handle 返回 AsyncHandle，可手动 stop()。

### 10. Task — 高级后台任务
Task::run 把"启动线程 + 发送结果 + UI 更新"合并成一次调用。

rust
use fltk_kit::Task;

Task::run(
    0.1,                                     // 轮询间隔（秒）
    || load_records(),                       // 工作线程
    move |records| {                         // UI 线程回调
        ui.borrow_mut().show_records(&records);
    },
);
带取消：

rust
use fltk_kit::{Task, CancelToken};

let token = Task::run_cancellable(
    0.1,
    |cancel: &CancelToken| {
        let mut out = Vec::new();
        for i in 0..1000 {
            if cancel.is_cancelled() {
                break;
            }
            out.push(heavy_step(i));
        }
        out
    },
    move |out| ui.borrow_mut().show_records(&out),
);

// 之后在 UI 线程取消
token.cancel();
关键约定：

work 闭包跑在工作线程，不能碰 UI 控件。

on_done 闭包跑在 UI 线程，负责更新控件。

work 闭包返回的数据必须 Send。

on_done 里不做重活，否则会卡 UI。

## 四、典型使用流程
主程序骨架
rust
use fltk::{app, prelude::*, window::Window};
use fltk_kit::{Fonts, Windows, Icon};

fn main() {
    // 1. 设置字体（在创建任何控件之前）
    Fonts::setup_by_language("zh-CN", 14);

    // 2. 创建 App
    let app = app::App::default();

    // 3. 创建窗口并最大化
    let mut wind = Windows::create_maximized(800, 600, "我的应用");

    // 4. 设置图标
    Icon::set_embd_png(&mut wind, include_bytes!("../assets/app.png"));

    // 5. 创建 Tabs 和各个页面...

    // 6. 运行事件循环
    app.run().unwrap();
}
页面布局
rust
use fltk_kit::{AutoPage, RectExt};

let mut page = AutoPage::new(0, 30, 800, 570, "表格");
page.group().begin();
// 创建控件...
page.group().end();

page.on_layout(move |content| {
    // 用 Rect 组合布局
});

// resize 时
wind.resize_callback(move |_, _, _, w, h| {
    page.relayout(w, h);
});
异步处理
rust
// 点击按钮 → 启动后台任务 → 结果更新到控件
btn_export.set_callback(move |_| {
    Dialogs::confirm_center(&win, "确定导出？");

    Task::run(
        0.1,
        || export_to_xlsx(&path),
        move |ok| {
            if ok {
                Dialogs::success_center(&win, "导出成功");
            } else {
                Dialogs::error_center(&win, "导出失败");
            }
        },
    );
});

## 五、示例程序
examples/ 目录下有 12 个示例，每个演示一个模块：

示例	演示内容
rect_layout	用 Rect 组合布局
auto_page	AutoPage 自动缩放
fonts	三级优先级字体选择
icon	图标与图像加载
dialogs	所有弹窗类型
tree_helper	Tree 构建与点击
table_helper	表格填充
windows	窗口创建
shortcuts	全局快捷键
channel_demo	低层通道
task_demo	高层任务
task_cancellable	可取消任务
运行方式：

bash
cargo run --example rect_layout
cargo run --example task_demo
六、测试
bash
cargo test                # 运行纯逻辑测试（26 个）
cargo test -- --ignored   # 运行需要显示环境的测试
覆盖：

Rect 几何操作（10 个测试）

Text 文本处理（8 个测试）

Paths 路径辅助（4 个测试）

Channel 通道行为（4 个测试 + 1 个 ignored）

CancelToken 取消令牌

GUI 相关的测试用 #[ignore] 标记，需要显示环境时手动运行。

七、关键设计原则
1. 分层明确
core 层不依赖 FLTK，纯逻辑，可被任何项目复用。widget 和 async_kit 依赖 FLTK，提供 GUI 相关辅助。

2. 单位结构体 + 关联函数
所有功能挂在类型下，调用统一为 Type::method(...)：

rust
Fonts::setup_auto(...)
Dialogs::info_center(...)
Rect::content(...)
Channel::spawn_poll(...)
Task::run(...)
IDE 补全友好，命名冲突免疫。

3. 明确失败
Icon::set_* 返回 bool，失败不 panic。

Fonts::setup_auto 返回 FontSetup，包含 success 和 source。

Sender::send 返回 Result<(), T>，失败可重试。

Task::run_cancellable 返回 CancelToken，可随时取消。

4. 不绑业务
Theme 只放跨项目通用常量（间距、字号），业务专属常量留在业务代码。

Rect 提供切分操作，不预设布局。

Channel / Task 只负责线程间通信，不掺业务逻辑。

5. 异步极简
Channel 处理低层"工作线程 → UI 线程"。

Task 进一步封装"启动线程 + 通道 + 回传"。

CancelToken 让长任务可被用户取消。

八、常见问题
1. RA 报错但 cargo check 通过
rust-analyzer 的数组 unsize coercion 推断偶尔落后于 rustc。以 cargo check / cargo build 为准。绕开方式：

rust
// RA 可能误报
let exts: &[&str] = &["dwg", "dxf"];

// 改用 Vec，RA 稳
let exts = vec!["dwg", "dxf"];
2. 弹窗位置不对
确保窗口已经 show()。win.x_root() 在 show() 之前可能返回 (0, 0)。

3. apply_to 找不到
需要 use fltk_kit::RectExt;。apply_to 定义在这个 trait 上。

4. AutoPage::on_layout 闭包报错
闭包是 FnMut，捕获的控件句柄要 let mut：

rust
page.on_layout({
    let mut tree_c = tree.clone();   // mut
    move |content| {
        tree_c.set_pos(...);          // 可变借用
    }
});
5. Task 回调里不能访问 Ui
Ui 是 Rc<RefCell<Ui>>，可以跨回调共享。但 work 闭包（工作线程）不能捕获 Ui，因为它不是 Send。只有 on_done 闭包（UI 线程）才能访问。

九、依赖
toml
[dependencies]
fltk = "1.5"
fltk-table = "0.3"
base64 = "0.22"
image = { version = "0.25", default-features = false, features = ["png", "jpeg", "bmp", "gif"] }
ndarray = "0.16"
十、总结
fltk-kit 的目标是消除 FLTK 项目的样板代码。它把布局、字体、弹窗、图标、异步这些通用功能集中封装，业务代码只专注业务。三层结构清晰，core 不依赖 FLTK 可以独立测试，widget 和 async_kit 提供 GUI 辅助。

核心用法一句话概括：

rust
Fonts::setup_by_language(lang, 14);       // 字体
Rect::content(w, h, top, bottom, pad);    // 布局
Dialogs::info_center(&win, "消息");        // 弹窗
Task::run(0.1, work, on_done);            // 异步
每个功能一行调用，这就是 fltk-kit 的价值。

text

把上面内容保存为 `README.zh-CN.md` 或 `docs/README.zh-CN.md` 即可。如果想在 `README.md` 里链接它，加一行：

```markdown
[中文说明](README.zh-CN.md)