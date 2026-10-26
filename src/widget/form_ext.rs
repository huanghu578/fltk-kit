// src/widget/form_ext.rs

use crate::core::validator::Validator;
use fltk::{
    enums::Color,
    input::Input,
    prelude::{InputExt as FltkInputExt, WidgetBase, WidgetExt},
};
use std::rc::Rc;

/// 扩展 Trait：用于绑定验证器
pub trait ValidatedInput {
    fn bind_validator<V: Validator + 'static>(&mut self, validator: V);
}

impl ValidatedInput for Input {
    fn bind_validator<V: Validator + 'static>(&mut self, validator: V) {
        let mut input = self.clone();
        let default_color = input.text_color();

        // 用 Rc 包裹，使得闭包可以多次克隆它
        let validator = Rc::new(validator);

        input.handle(move |i, ev| {
            if ev == fltk::enums::Event::KeyUp || ev == fltk::enums::Event::Unfocus {
                let mut i_clone = i.clone();
                // 每次触发按键事件，克隆一份 Rc 指针进延时闭包
                let v = Rc::clone(&validator);

                // 10ms 延时，等待 FLTK 将输入框内容同步完毕后再验证
                fltk::app::add_timeout3(0.01, move |_| {
                    let text = i_clone.value();
                    match v.validate(&text) {
                        Ok(_) => {
                            i_clone.set_text_color(default_color);
                            i_clone.set_tooltip("");
                        }
                        Err(err) => {
                            i_clone.set_text_color(Color::Red);
                            i_clone.set_tooltip(&format!("{:?}", err));
                        }
                    }
                });
            }
            false
        });
    }
}
