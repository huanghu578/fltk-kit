//! Help page utilities and a ready-to-use Help widget.

use fltk::{button::Button, enums::Align, frame::Frame, group::Group, prelude::*, window::Window};
use std::path::{Path, PathBuf};

pub struct HelpPage {
    pub group: Group,
}

impl HelpPage {
    /// 创建一个帮助页面。
    /// `file_name`: 要打开的文件名（如 "help.pdf" 或 "help.html"）。
    /// 程序会优先在可执行文件所在目录查找，其次在当前工作目录查找。
    pub fn new(x: i32, y: i32, w: i32, h: i32, file_name: &str) -> Self {
        let group = Group::new(x, y, w, h, "");
        group.begin();

        // 1. 标题与说明文本
        let mut title = Frame::new(20, 40, w - 40, 40, "帮助与支持");
        title.set_label_size(24);
        title.set_align(Align::Center);

        let mut desc = Frame::new(20, 90, w - 40, 30, "点击下方按钮查看用户手册");
        desc.set_label_size(14);
        desc.set_align(Align::Center);
        desc.set_label_color(fltk::enums::Color::Dark3);

        // 2. 打开按钮 (居中)
        let btn_w = 200;
        let btn_h = 50;
        let btn_x = (w - btn_w) / 2;
        let mut btn = Button::new(btn_x, 150, btn_w, btn_h, "📖 打开帮助文档");
        btn.set_label_size(16);

        // 3. 绑定点击事件
        let file_name = file_name.to_string();
        let _: Option<Window> = None;

        // 尝试获取父窗口，用于弹窗居中
        if let Some(parent_win) = group.window() {
            // 我们需要一个具体的 Window 类型，但 window() 返回 Box<dyn WindowExt>
            // 这里做一个降级处理：如果拿不到具体的 Window，就不居中弹窗
            // 简单起见，直接使用 fltk::dialog::alert
            let _ = parent_win;
        }

        btn.set_callback(move |_| {
            if let Some(path) = Self::find_file(&file_name) {
                Self::open_file(&path);
            } else {
                // 使用原生 alert 兜底，避免类型不匹配
                fltk::dialog::alert(0, 0, &format!("找不到帮助文件: {}", file_name));
            }
        });

        group.end();
        Self { group }
    }

    /// 查找文件（优先找当前 exe 所在目录，其次找当前工作目录）
    fn find_file(file_name: &str) -> Option<PathBuf> {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let p = exe_dir.join(file_name);
                if p.exists() {
                    return Some(p);
                }
            }
        }

        let p = Path::new(file_name);
        if p.exists() {
            return Some(p.to_path_buf());
        }

        None
    }

    /// 调用系统默认程序打开文件
    fn open_file(path: &Path) {
        let abs_path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

        // 转换为标准的 file:// URL
        // Windows 需要 file:///C:/... (三个斜杠)
        let mut url = abs_path.display().to_string().replace("\\", "/");
        if !url.starts_with('/') {
            url.insert(0, '/');
        }
        let full_url = format!("file://{}", url);

        // 关键修复：FLTK 1.5.23 使用 utils::open_uri，而不是 app::openurl
        // open_uri 返回 Result<(), FltkError>
        if let Err(e) = fltk::utils::open_uri(&full_url) {
            eprintln!("Failed to open help file: {}", e);
        }
    }
}

// 方便用户直接访问内部的 group
impl std::ops::Deref for HelpPage {
    type Target = Group;
    fn deref(&self) -> &Self::Target {
        &self.group
    }
}

impl std::ops::DerefMut for HelpPage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.group
    }
}
