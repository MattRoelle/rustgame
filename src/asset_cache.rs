use sdl2::{image::LoadTexture, render::{Texture, TextureCreator}, video::WindowContext};

extern crate sdl2;

pub struct Assets<'a> {
    pub white_rect: Texture<'a>,
    pub blue_rect: Texture<'a>,
    pub red_rect: Texture<'a>,
    pub green_rect: Texture<'a>,
}

pub fn init<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Assets<'a>, String> {
    Ok(Assets {
        white_rect: texture_creator.load_texture("./resources/white_rect.png")?,
        blue_rect: texture_creator.load_texture("./resources/blue_rect.png")?,
        red_rect: texture_creator.load_texture("./resources/red_rect.png")?,
        green_rect: texture_creator.load_texture("./resources/green_rect.png")?,
    })
}