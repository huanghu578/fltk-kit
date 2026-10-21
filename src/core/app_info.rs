//! Compile-time application metadata collected from `Cargo.toml`.

pub struct AppInfo;

impl AppInfo {
    /// 包名 (`[package] name`)
    pub const NAME: &'static str = env!("CARGO_PKG_NAME");

    /// 版本号 (`[package] version`)
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// 描述/About 字符串 (`[package] description`)
    pub const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

    /// 作者 (`[package] authors`)
    pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    /// 仓库地址 (`[package] repository`)
    pub const REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");

    /// 许可证 (`[package] license`)
    pub const LICENSE: &'static str = env!("CARGO_PKG_LICENSE");

    /// 返回格式化好的多行文本，专为 `TextDisplay`、`HelpView` 或
    /// 多行 `Frame` 设计。
    pub fn format_about() -> String {
        let mut text = String::new();
        
        if !Self::ABOUT.is_empty() {
            text.push_str(&format!("{}\n\n", Self::ABOUT));
        }
        
        text.push_str(&format!("Version: {}\n", Self::VERSION));
        
        if !Self::AUTHORS.is_empty() {
            text.push_str(&format!("Authors: {}\n", Self::AUTHORS));
        }
        
        if !Self::LICENSE.is_empty() {
            text.push_str(&format!("License: {}\n", Self::LICENSE));
        }
        
        if !Self::REPOSITORY.is_empty() {
            text.push_str(&format!("Repository: {}\n", Self::REPOSITORY));
        }

        text
    }
}