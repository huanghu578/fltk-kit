//! Path manipulation utilities.

use std::path::Path;

pub struct Paths;

impl Paths {
    /// Safely gets the file extension in lowercase.
    pub fn get_ext_lowercase<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
    }
}