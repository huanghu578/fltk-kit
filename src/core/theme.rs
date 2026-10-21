//! Shared UI constants.
use fltk::prelude::*;
use fltk::enums::{Color, FrameType};

impl Theme {
    /// Recursively applies a dark theme to a group of widgets.
    pub fn apply_dark<T: GroupExt>(root: &mut T) {
        root.set_color(Color::from_rgb(40, 40, 40));
        root.set_selection_color(Color::from_rgb(60, 120, 200));

        // 递归遍历并修改子控件样式
        for i in 0..root.children() {
            if let Some(mut child) = root.child(i) {
                child.set_color(Color::from_rgb(40, 40, 40));
                child.set_label_color(Color::White);
                // 如果是输入框之类的，可以设置文字颜色
                // child.set_text_color(Color::White); 
                child.set_frame(FrameType::FlatBox);
            }
        }
    }
}
pub struct Theme;

impl Theme {
    pub const PAD_SM: i32 = 4;
    pub const PAD_MD: i32 = 8;
    pub const PAD_LG: i32 = 16;
    pub const WIN_W: i32 = 800;
    pub const WIN_H: i32 = 600;
}