//! File system watching (RAII).
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

pub struct FileWatcher { _watcher: RecommendedWatcher }

impl FileWatcher {
    /// Calls `callback` on a background thread whenever the file changes.
    pub fn watch<P: AsRef<Path>, F>(path: P, mut callback: F) -> notify::Result<Self>
    where F: FnMut() + Send + 'static {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)?;
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        std::thread::spawn(move || {
            for res in rx {
                if res.is_ok() { callback(); }
            }
        });
        Ok(Self { _watcher: watcher })
    }
}