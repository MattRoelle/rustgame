use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    fn distance_to(&self, v2: Vec2) -> f64 {
        return ((v2.y - self.y).powf(2.0) + (v2.x - self.x).powf(2.0)).sqrt()
    }

    fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SimpleRect {
    pub pos: Vec2,
    pub size: Vec2
}

impl Display for SimpleRect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.pos.x, self.pos.y, self.size.x, self.size.y)
    }
}

impl SimpleRect {
    pub fn left(&self)  -> f64  { self.pos.x }
    pub fn right(&self) -> f64 { self.pos.x + self.size.x }
    pub fn top(&self) -> f64 { self.pos.y }
    pub fn bottom(&self) -> f64 { self.pos.y + self.size.y }

    pub fn collides_with(&self, r2: SimpleRect) -> bool {
        !(
            self.left() > r2.right()
        ||  self.top() > r2.bottom()
        ||  self.left() < r2.right()
        ||  self.bottom() < r2.top()
        )
    }

    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size: Vec2::new(w, h)
        }
    }

    pub fn clamp(&mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        self.pos.x = self.pos.x.max(min_x).min(max_x - self.size.x);
        self.pos.y = self.pos.y.max(min_y).min(max_y - self.size.y);
    }
}