//! Windows-only System Tray implementation.
//! Requires the `tray` feature.

// ==========================================
// 1. Windows 且开启 tray feature 的真实实现
// ==========================================
#[cfg(all(windows, feature = "tray"))]
mod windows_impl {
    use fltk::{app};
    use std::sync::mpsc::channel;
    use tray_icon::{
        menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
        TrayIcon as WinTrayIcon, TrayIconBuilder,
    };

    pub struct TrayIcon {
        // 必须持有这个字段，否则托盘图标会在离开作用域时被销毁
        _tray: WinTrayIcon,
    }

    impl TrayIcon {
        /// 创建一个系统托盘
        /// icon_path: 托盘图标的路径 (建议使用 png 或 ico)
        /// tooltip: 鼠标悬停时的提示文本
        /// on_show: 点击“显示主窗口”时的回调
        /// on_exit: 点击“退出”时的回调
        pub fn create<F1, F2>(
            icon_path: &str,
            tooltip: &str,
            mut on_show: F1,
            mut on_exit: F2,
        ) -> Result<Self, String>
        where
            F1: FnMut() + Send + 'static,
            F2: FnMut() + Send + 'static,
        {
            // 1. 加载图标
            let img = image::open(icon_path)
                .map_err(|e| format!("Failed to load icon: {}", e))?
                .into_rgba8();
            let (width, height) = img.dimensions();
            let icon = tray_icon::Icon::from_rgba(img.into_raw(), width, height)
                .map_err(|e| format!("Failed to create icon: {}", e))?;

            // 2. 创建右键菜单
            let menu = Menu::new();
            let item_show = MenuItem::new("Show Window", true, None);
            let item_exit = MenuItem::new("Exit", true, None);
            menu.append(&item_show).map_err(|e| e.to_string())?;
            menu.append(&PredefinedMenuItem::separator())
                .map_err(|e| e.to_string())?;
            menu.append(&item_exit).map_err(|e| e.to_string())?;

            // 3. 创建托盘图标
            let tray = TrayIconBuilder::new()
                .with_menu(Box::new(menu))
                .with_tooltip(tooltip)
                .with_icon(icon)
                .build()
                .map_err(|e| format!("Failed to build tray: {}", e))?;

            // 4. 开启后台线程监听菜单事件
            let (tx, rx) = channel::<String>();
            let id_show = item_show.id().clone();
            let id_exit = item_exit.id().clone();

            // tray-icon 在独立线程中分发事件，我们通过 channel 桥接
            std::thread::spawn(move || {
                let menu_channel = MenuEvent::receiver();
                while let Ok(event) = menu_channel.recv() {
                    if event.id == id_show {
                        let _ = tx.send("show".to_string());
                    } else if event.id == id_exit {
                        let _ = tx.send("exit".to_string());
                    }
                }
            });

            // 5. 利用 FLTK 的 app::add_timeout3 在主线程轮询 channel
            app::add_timeout3(0.1, move |handle| {
                while let Ok(action) = rx.try_recv() {
                    if action == "show" {
                        on_show();
                    } else if action == "exit" {
                        on_exit();
                    }
                }
                app::repeat_timeout3(0.1, handle);
            });

            Ok(Self { _tray: tray })
        }
    }
}

// ==========================================
// 2. 非 Windows 或未开启 tray feature 的占位实现
// ==========================================
#[cfg(not(all(windows, feature = "tray")))]
pub struct TrayIcon;

#[cfg(not(all(windows, feature = "tray")))]
impl TrayIcon {
    /// 跨平台占位版本，永远返回 Err。
    pub fn create<F1, F2>(
        _icon_path: &str,
        _tooltip: &str,
        _on_show: F1,
        _on_exit: F2,
    ) -> Result<Self, String>
    where
        F1: FnMut(),
        F2: FnMut(),
    {
        // 显式消费掉闭包，避免编译器未使用警告和借用检查报错
        drop(_on_show);
        drop(_on_exit);

        Err("System tray is only supported on Windows with the 'tray' feature enabled.".to_string())
    }
}

// ==========================================
// 3. 统一导出
// ==========================================
#[cfg(all(windows, feature = "tray"))]
pub use windows_impl::TrayIcon;