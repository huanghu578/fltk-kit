pub struct AppInfo;

impl AppInfo {
    pub const NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");
    pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    pub fn format_about() -> String {
        format!(
            "{}\n\nVersion: {}\nAuthors: {}",
            Self::ABOUT,
            Self::VERSION,
            Self::AUTHORS
        )
    }
}
