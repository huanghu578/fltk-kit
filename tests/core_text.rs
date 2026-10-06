use fltk_kit::Text;

#[test]
fn truncate_short_string_unchanged() {
    assert_eq!(Text::truncate("hello", 10), "hello");
}

#[test]
fn truncate_long_string_appends_ellipsis() {
    let out = Text::truncate("hello world", 8);
    assert!(out.ends_with('…'));
    assert_eq!(out.chars().count(), 8);
}

#[test]
fn human_bytes_zero() {
    assert_eq!(Text::human_bytes(0), "0 B");
}

#[test]
fn human_bytes_kb() {
    assert_eq!(Text::human_bytes(1024), "1.0 KB");
}

#[test]
fn human_bytes_mb() {
    assert_eq!(Text::human_bytes(1024 * 1024), "1.0 MB");
}

#[test]
fn human_count_thousands() {
    assert_eq!(Text::human_count(1234), "1,234");
    assert_eq!(Text::human_count(1234567), "1,234,567");
    assert_eq!(Text::human_count(100), "100");
}

#[test]
fn truncate_display_ascii() {
    let out = Text::truncate_display("hello world", 6);
    assert!(out.ends_with('…'));
}

#[test]
fn truncate_display_cjk_counts_double() {
    // Each CJK char is width 2, so max_width 5 fits at most 2 chars plus ellipsis.
    let out = Text::truncate_display("中文字符串", 5);
    assert!(out.ends_with('…'));
    assert!(out.chars().count() <= 3);
}