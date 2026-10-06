//! Three-tier font selection demo.
//!
//! Run: cargo run --example fonts

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_kit::{Fonts, FontSource};

fn main() {
    let app = app::App::default();

    // Three-tier selection.
    let result = Fonts::setup_auto(
        Some(&["Source Han Sans", "Noto Sans CJK SC"]), // user preference
        Some("zh-CN"),                                   // system language
        16,
    );

    let source = match result.source {
        FontSource::User => "user",
        FontSource::System => "system",
        FontSource::Fallback => "fallback",
        FontSource::Default => "default",
    };
    let name = result.font_name.unwrap_or_else(|| "FLTK default".into());

    let mut wind = Window::default().with_size(600, 200).with_label("Fonts");

    // Pre-build the strings, then pass `&str` to `Frame::new`.
    let text1 = format!("Source: {}", source);
    let text2 = format!("Font: {}", name);

    let mut label1 = Frame::new(20, 20, 560, 40, text1.as_str());
    label1.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    let mut label2 = Frame::new(20, 60, 560, 40, text2.as_str());
    label2.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    let mut label3 = Frame::new(20, 100, 560, 40, "中文 English 日本語 한국어");
    label3.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);

    let _btn = Button::new(20, 150, 120, 30, "Button");

    wind.end();
    wind.show();
    app.run().unwrap();
}