//! build.rs
use std::path::Path;

fn main() {
    // 仅在 Windows 下编译，并且文件存在时才注入资源
    #[cfg(windows)]
    {
        if Path::new("app.ico").exists() {
            let mut res = winres::WindowsResource::new();
            res.set_icon("assets/icon.ico");

            // 🌟 动态读取用户项目的 Cargo.toml 元数据
            let description = std::env::var("CARGO_PKG_DESCRIPTION").unwrap_or_default();
            let product_name = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
            let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();
            let authors = std::env::var("CARGO_PKG_AUTHORS").unwrap_or_default();

            if !description.is_empty() {
                res.set("FileDescription", &description);
            }
            if !product_name.is_empty() {
                res.set("ProductName", &product_name);
            }
            if !version.is_empty() {
                res.set("FileVersion", &version);
                res.set("ProductVersion", &version);
            }
            if !authors.is_empty() {
                // 将多个作者用逗号分隔（Cargo.toml 中的 authors 默认是冒号分隔）
                res.set("LegalCopyright", &authors.replace(":", ", "));
            }

            if let Err(e) = res.compile() {
                eprintln!("Warning: Failed to compile Windows resources: {}", e);
            }
        }
    }
}