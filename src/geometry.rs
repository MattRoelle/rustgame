use std::fmt::Display;

type V = f32;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: V,
    pub y: V
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    fn distance_to(&self, v2: Vec2) -> V {
        return ((v2.y - self.y).powf(2.0) + (v2.x - self.x).powf(2.0)).sqrt()
    }

    fn new(x: V, y: V) -> Self {
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
    pub fn left(&self)  -> V  { self.pos.x }
    pub fn right(&self) -> V { self.pos.x + self.size.x }
    pub fn top(&self) -> V { self.pos.y }
    pub fn bottom(&self) -> V { self.pos.y + self.size.y }

    pub fn collides_with(&self, r2: SimpleRect) -> bool {
        !(
            self.left() > r2.right()
        ||  self.top() > r2.bottom()
        ||  self.left() < r2.right()
        ||  self.bottom() < r2.top()
        )
    }

    pub fn new(x: V, y: V, w: V, h: V) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size: Vec2::new(w, h)
        }
    }
}