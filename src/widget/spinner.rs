//! A simple loading spinner widget drawn with custom vector graphics.

use fltk::{app, draw, enums::*, prelude::*, widget::Widget};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Spinner {
    widget: Widget,
    angle: Rc<RefCell<f64>>,
}

impl Spinner {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut widget = Widget::new(x, y, w, h, "");
        let angle = Rc::new(RefCell::new(0.0));
        let angle_draw = angle.clone();
        
        // 绘制逻辑
        widget.draw(move |w| {
            let a = *angle_draw.borrow();
            
            // 画一个灰色的底圆
            draw::set_draw_color(Color::from_rgb(220, 220, 220)); 
            draw::set_line_style(draw::LineStyle::Solid, 3);
            let padding = 10;
            draw::draw_arc(w.x() + padding, w.y() + padding, w.w() - padding*2, w.h() - padding*2, 0.0, 360.0);
            
            // 画蓝色的转圈部分
            draw::set_draw_color(Color::from_rgb(50, 150, 250)); 
            draw::draw_arc(w.x() + padding, w.y() + padding, w.w() - padding*2, w.h() - padding*2, a, a + 90.0);
        });

        let s = Self { widget, angle };
        s.start_animation();
        s
    }

    fn start_animation(&self) {
        let angle_timer = self.angle.clone();
        let mut w_clone = self.widget.clone();
        
        // 启动定时器
        app::add_timeout3(0.05, move |handle| {
            if w_clone.was_deleted() {
                return;
            }

            // 无论是否可见，我们都在后台自增角度
            let mut a = angle_timer.borrow_mut();
            *a += 15.0;
            if *a >= 360.0 {
                *a = 0.0;
            }
            
            // 只有可见时才触发重绘以节省 CPU
            if w_clone.visible() {
                w_clone.redraw();
            }

            // 循环定时器
            app::repeat_timeout3(0.05, handle);
        });
    }

    pub fn show(&mut self) {
        self.widget.show();
    }

    pub fn hide(&mut self) {
        self.widget.hide();
    }
}

impl std::ops::Deref for Spinner {
    type Target = Widget;
    fn deref(&self) -> &Self::Target { &self.widget }
}

impl std::ops::DerefMut for Spinner {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.widget }
}