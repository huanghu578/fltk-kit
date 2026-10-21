// 布局与基础扩展
pub mod layout_ext;
pub mod form_ext;
pub mod table;
pub mod shortcuts;

// 高频 UI 辅助组件 (保留自原版)
pub mod dialogs;
pub mod fonts;
pub mod auto_page;
pub mod tree;
pub mod windows;
pub mod help;
pub use help::HelpPage;
pub mod tray; // 添加这一行
pub mod spinner;
pub mod state_ext;
pub mod clipboard;
pub mod progress_ext;
// 可选功能
#[cfg(feature = "icon")]
pub mod icon;


pub mod persist;