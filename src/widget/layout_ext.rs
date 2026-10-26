//! Extension trait to apply `Rect` to widgets.
use fltk::prelude::*;
use crate::core::rect::Rect;

pub trait RectExt {
    /// 完全应用 Rect 的坐标和尺寸
    fn apply_rect(&mut self, r: Rect);
    
    /// 仅应用尺寸，保持原有坐标不变
    fn apply_size(&mut self, r: Rect);
}

impl<T: WidgetExt> RectExt for T {
    fn apply_rect(&mut self, r: Rect) {
        self.resize(r.x, r.y, r.w, r.h);
    }

    fn apply_size(&mut self, r: Rect) {
        self.resize(self.x(), self.y(), r.w, r.h);
    }
}