use crate::geometry::{Vec2, SimpleRect};
use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

pub struct Sprite<'a> {
    tex: &'a Texture<'a>,
    sdl_rect: Option<Rect>,
    pub rect: SimpleRect,
}

impl<'a> Sprite<'a> {
    pub fn new(tex: &'a Texture<'a>, x: f32, y: f32, w: f32, h: f32) -> Self {
        let mut ret = Self {
            tex,
            sdl_rect: None,
            rect: SimpleRect::new(x, y, w, h),
        };

        ret.update_sdl_rect();

        return ret
    }

    pub fn pos(&self) -> Vec2 { self.rect.pos }
    pub fn size(&self) -> Vec2 { self.rect.size }

    pub fn set_size(&mut self, w: f32, h: f32) {
        self.rect.size.x = w;
        self.rect.size.y = h;
        self.update_sdl_rect();
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.rect.pos.x = x;
        self.rect.pos.y = y;
        self.update_sdl_rect();
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.rect.pos.x += dx;
        self.rect.pos.y += dy;
        self.update_sdl_rect();
    }

    pub fn update_sdl_rect(&mut self) {
        self.sdl_rect = Some(Rect::new(
            self.rect.pos.x as i32,
            self.rect.pos.y as i32,
            self.rect.size.x as u32,
            self.rect.size.y as u32,
        ));
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.copy(&self.tex, None, self.sdl_rect).expect("Error calling canvas.copy");
    }
}
