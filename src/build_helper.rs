use std::env;
use std::fs;
use std::path::Path;

/// 在用户项目的 build.rs 中调用。
/// 读取 Cargo.toml 解析依赖，并生成 dependencies.rs 供代码嵌入。
pub fn collect_dependencies() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dependencies.rs");
    
    // 获取当前 crate 的元数据
    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps() // 不解析整棵树，只读当前项目
        .exec()
        .expect("Failed to get cargo metadata");

    let mut deps_list = Vec::new();

    // 遍历当前包的依赖
    if let Some(root_pkg) = metadata.root_package() {
        for dep in &root_pkg.dependencies {
            // 过滤掉可选依赖没启用的，或者只保留正常的依赖
            let version = dep.req.to_string();
            let dep_name = &dep.name;
            deps_list.push(format!("{} = {}", dep_name, version));
        }
    }

    let code = format!(
        "pub const DEPENDENCIES: &[&str] = &[\n{},\n];\n",
        deps_list.iter().map(|s| format!("    \"{}\"", s)).collect::<Vec<_>>().join("\n")
    );
    fs::write(dest_path, code).unwrap();
}
// src/build_helper.rs
/// 在用户的 build.rs 中调用，用于设置 Windows Exe 图标。
/// icon_path 是相对于用户项目根目录的 .ico 文件路径。
pub fn set_exe_icon(icon_path: &str) {
    #[cfg(all(windows, feature = "exe-icon"))]
    {
        let mut res = tauri_winres::WindowsResource::new();
        res.set_icon(icon_path);
        // 如果编译失败（比如缺少 rc.exe），则输出警告而不是 panic，避免阻塞构建
        if let Err(e) = res.compile() {
            println!("cargo:warning=Failed to set exe icon: {}", e);
        }
    }
    // 在非 Windows 平台，这个函数什么都不做
    #[cfg(not(all(windows, feature = "exe-icon")))]
    {
        let _ = icon_path; // 消除 unused 警告
    }
}