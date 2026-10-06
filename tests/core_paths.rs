use fltk_kit::Paths;

#[test]
fn file_stem_strips_directory_and_extension() {
    assert_eq!(Paths::file_stem("/a/b/c/file.dwg"), "file");
    assert_eq!(Paths::file_stem("file.dxf"), "file");
    assert_eq!(Paths::file_stem("noext"), "noext");
}

#[test]
fn folder_name_returns_last_component() {
    assert_eq!(Paths::folder_name("/a/b/c"), "c");
    assert_eq!(Paths::folder_name("c"), "c");
}

#[test]
fn exe_dir_is_not_empty() {
    let dir = Paths::exe_dir();
    assert!(!dir.as_os_str().is_empty());
}

#[test]
fn beside_exe_joins_name() {
    let p = Paths::beside_exe("config.toml");
    assert!(p.ends_with("config.toml"));
}