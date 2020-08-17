use crate::geometry::{SimpleRect, Vec2};
use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

pub struct Sprite<'a> {
    tex: &'a Texture<'a>,
    sdl_rect: Option<sdl2::rect::Rect>,
    pub rect: SimpleRect,
    pub angle: f64,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl<'a> Sprite<'a> {
    pub fn new(tex: &'a Texture<'a>, x: f64, y: f64, w: f64, h: f64) -> Self {
        let mut ret = Self {
            tex,
            sdl_rect: None,
            rect: SimpleRect::new(x, y, w, h),
            angle: 0.0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        ret.update_sdl_rect();

        return ret;
    }

    pub fn pos(&self) -> Vec2 {
        self.rect.pos
    }
    pub fn size(&self) -> Vec2 {
        self.rect.size
    }

    pub fn set_size(&mut self, w: f64, h: f64) {
        self.rect.size.x = w;
        self.rect.size.y = h;
        self.update_sdl_rect();
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.rect.pos.x = x;
        self.rect.pos.y = y;
        self.update_sdl_rect();
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.rect.pos.x += dx;
        self.rect.pos.y += dy;
        self.update_sdl_rect();
    }

    pub fn update_sdl_rect(&mut self) {
        self.sdl_rect = Some(sdl2::rect::Rect::new(
            self.rect.pos.x as i32,
            self.rect.pos.y as i32,
            self.rect.size.x as u32,
            self.rect.size.y as u32,
        ));
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas
            .copy_ex(
                &self.tex,
                None,
                self.sdl_rect,
                self.angle,
                None,
                self.flip_horizontal,
                self.flip_vertical,
            )
            .expect("Error calling canvas.copy_ex")
    }

    pub fn clamp(&mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        self.rect.clamp(min_x, min_y, max_x, max_y);
        self.update_sdl_rect();
    }
}
