//! Tree widget helpers.

use fltk::{prelude::*, tree::Tree};

pub struct TreeHelper;

impl TreeHelper {
    /// Rebuild a tree from `(parent, child)` pairs and collapse parents.
    pub fn rebuild_from_paths(tree: &mut Tree, paths: &[(String, String)]) {
        tree.clear();

        let mut tops: Vec<String> = Vec::new();
        for (a, b) in paths {
            let _ = tree.add(&format!("{}/{}", a, b));
            tops.push(a.clone());
        }
        tops.sort();
        tops.dedup();

        for a in &tops {
            let _ = tree.close(a, false);
        }
        tree.redraw();
    }

    /// Rebuild a tree where the root label is `folder_name` and every
    /// entry in `files` is a child.
    pub fn rebuild_from_files(tree: &mut Tree, folder_name: &str, files: &[String]) {
        tree.clear();
        for name in files {
            let _ = tree.add(name);
        }
        tree.set_root_label(folder_name);
        tree.set_show_root(true);
        tree.redraw();
    }

    /// Path of the first selected item, if any.
    pub fn selected_path(tree: &Tree) -> Option<String> {
        let item = tree.first_selected_item()?;
        tree.item_pathname(&item).ok()
    }

    /// Scan a directory for files with the given extensions.
    /// Returns the file stems (without extensions), sorted.
    pub fn scan_files(dir: &str, extensions: &[&str]) -> Vec<String> {
        let entries = match std::fs::read_dir(dir) {
            Ok(it) => it,
            Err(_) => return Vec::new(),
        };

        let mut files: Vec<String> = Vec::new();
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_file() {
                continue;
            }
            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if extensions.iter().any(|e| e.eq_ignore_ascii_case(&ext_lower)) {
                    if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                        files.push(stem.to_string());
                    }
                }
            }
        }
        files.sort();
        files
    }

    /// Scan a directory for files with the given extensions.
    /// Returns the full file names (including extensions), sorted.
    pub fn scan_files_with_ext(dir: &str, extensions: &[&str]) -> Vec<String> {
        let entries = match std::fs::read_dir(dir) {
            Ok(it) => it,
            Err(_) => return Vec::new(),
        };

        let mut files: Vec<String> = Vec::new();
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_file() {
                continue;
            }
            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if extensions.iter().any(|e| e.eq_ignore_ascii_case(&ext_lower)) {
                    // 使用 file_name 而不是 file_stem，保留扩展名
                    if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                        files.push(name.to_string());
                    }
                }
            }
        }
        files.sort();
        files
    }
}

// ==========================================
// 新增：Extension Trait
// ==========================================

pub trait TreeBindingExt {
    /// Binds data represented as a list of `(parent_name, child_name)`.
    fn bind_data(&mut self, data: &[(String, String)]);
}

impl TreeBindingExt for Tree {
    fn bind_data(&mut self, data: &[(String, String)]) {
        self.clear();
        self.set_show_root(true);

        // 使用 FLTK 原生的 "父/子" 路径解析机制直接建树
        for (parent, child) in data {
            if parent == "ROOT" || parent == "My Project" {
                let _ = self.add(child);
            } else {
                let _ = self.add(&format!("{}/{}", parent, child));
            }
        }

        let _ = self.open("ROOT", false);
        self.redraw();
    }
}