//! Safe wrapper around FLTK's system clipboard.

use fltk::prelude::*;
use fltk::input::Input;

pub struct Clipboard;

impl Clipboard {
    /// 复制文本到系统剪贴板
    pub fn set_text(text: &str) {
        fltk::app::copy(text);
    }

    /// 从系统剪贴板获取文本。
    /// 在 FLTK 中，读取剪贴板需要绑定到一个具体的控件上。
    /// 我们在这里创建一个临时的隐藏输入框来安全地“接住”剪贴板的文本。
    pub fn get_text() -> Option<String> {
        // 创建一个 0x0 的隐藏输入框，不显示在界面上
        let mut temp_input = Input::new(0, 0, 0, 0, "");
        
        // 让这个隐藏的输入框去请求粘贴操作
        fltk::app::paste(&mut temp_input);
        
        // 获取粘贴后的内容
        let text = temp_input.value();
        
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}