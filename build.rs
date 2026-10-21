fn main() {
    // 告诉 Cargo 重新运行的条件
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // 生成一个假的 dependencies 列表
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("dependencies.rs");
    
    let code = r#"
        pub const DEPENDENCIES: &[&str] = &[
            "fltk = 1.5.23",
            "fltk-table = 0.1",
            "ndarray = 0.15",
        ];
    "#;
    
    std::fs::write(dest_path, code).unwrap();
}