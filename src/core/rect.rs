#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
    pub fn cut_top(&mut self, amount: i32) -> Rect {
        let a = amount.min(self.h);
        let r = Rect::new(self.x, self.y, self.w, a);
        self.y += a;
        self.h -= a;
        r
    }
    pub fn cut_bottom(&mut self, amount: i32) -> Rect {
        let a = amount.min(self.h);
        let r = Rect::new(self.x, self.y + self.h - a, self.w, a);
        self.h -= a;
        r
    }
    pub fn cut_left(&mut self, amount: i32) -> Rect {
        let a = amount.min(self.w);
        let r = Rect::new(self.x, self.y, a, self.h);
        self.x += a;
        self.w -= a;
        r
    }
    pub fn cut_right(&mut self, amount: i32) -> Rect {
        let a = amount.min(self.w);
        let r = Rect::new(self.x + self.w - a, self.y, a, self.h);
        self.w -= a;
        r
    }
}
