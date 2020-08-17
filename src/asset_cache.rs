use bytebuffer::ByteBuffer;
use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::io::BufReader;

extern crate sdl2;

pub struct Assets<'a> {
    pub white_rect: Texture<'a>,
    pub blue_rect: Texture<'a>,
    pub red_rect: Texture<'a>,
    pub green_rect: Texture<'a>,
    pub tilemap: Texture<'a>,
    pub test_level: tiled::Map,
}

pub fn init<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Assets<'a>, String> {
    Ok(Assets {
        white_rect: texture_creator.load_texture("./resources/white_rect.png")?,
        blue_rect: texture_creator.load_texture("./resources/blue_rect.png")?,
        red_rect: texture_creator.load_texture("./resources/red_rect.png")?,
        green_rect: texture_creator.load_texture("./resources/green_rect.png")?,
        tilemap: texture_creator.load_texture("./resources/tilemap.png")?,
        test_level: tiled::parse(BufReader::new(ByteBuffer::from_bytes(include_bytes!(
            "../resources/test_level.tmx"
        )))).expect("Failed to load map"),
    })
}
