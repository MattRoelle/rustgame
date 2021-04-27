use sdl2::{video::Window, render::Canvas};
use super::text::FontAtlas;

pub trait Drawable {
    fn visible(&self) -> bool { true }
    fn z_index(&self) -> i32 { 0 }
    fn draw<'a>(&mut self, canvas: &mut Canvas<Window>, font_atlas: &mut FontAtlas<'a>);
}

pub struct RenderList {
    pub items: Vec<Box<dyn Drawable>>
}

impl RenderList {
    pub fn new() -> Self {
        Self {
            items: vec![]
        }
    }

    pub fn add_item(&mut self, item: Box<dyn Drawable>) {
        self.items.push(item);
        self.sort();
    }

    pub fn remove_item(&mut self, item: Box<dyn Drawable>) {
        todo!();
    }

    pub fn sort(&mut self) {
        self.items.sort_by(|a, b| { a.z_index().cmp(&b.z_index()) });
    }

    pub fn draw<'a>(&mut self, canvas: &mut Canvas<Window>, font_atlas: &mut FontAtlas<'a>) {
        for i in self.items.iter_mut() {
            if i.visible() {
                i.draw(canvas, font_atlas);
            }
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}