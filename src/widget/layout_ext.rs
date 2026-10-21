//! Extension trait to apply `Rect` to widgets.
use fltk::prelude::*;
use crate::core::rect::Rect;

pub trait RectExt {
    fn apply_rect(&mut self, r: Rect);
}

impl<T: WidgetExt> RectExt for T {
    fn apply_rect(&mut self, r: Rect) {
        self.resize(r.x, r.y, r.w, r.h);
    }
}