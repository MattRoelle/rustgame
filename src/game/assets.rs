use bytebuffer::ByteBuffer;
use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator, Canvas},
    video::{Window, WindowContext}, ttf::{Font, Sdl2TtfContext},
};
use std::io::BufReader;
use crate::engine::text::FontAtlas;

extern crate sdl2;

pub struct Assets<'a> {
    pub white_rect: Texture<'a>,
    pub blue_rect: Texture<'a>,
    pub red_rect: Texture<'a>,
    pub green_rect: Texture<'a>,
    pub tilemap: Texture<'a>,
    pub test_level: tiled::Map,
    pub font: FontAtlas<'a>
}

pub fn init<'a>(
    canvas: &mut Box<Canvas<Window>>,
    texture_creator: &'a TextureCreator<WindowContext>,
    ttf_context: &'a Sdl2TtfContext
) -> Result<Assets<'a>, String> {
    Ok(Assets {
        white_rect: texture_creator.load_texture("./resources/white_rect.png")?,
        blue_rect: texture_creator.load_texture("./resources/blue_rect.png")?,
        red_rect: texture_creator.load_texture("./resources/red_rect.png")?,
        green_rect: texture_creator.load_texture("./resources/green_rect.png")?,
        tilemap: texture_creator.load_texture("./resources/tilemap.png")?,
        test_level: tiled::parse(BufReader::new(ByteBuffer::from_bytes(include_bytes!(
            "../../resources/test_level.tmx"
        )))).expect("Failed to load map"),
        font: FontAtlas::new(
            canvas,
            texture_creator,
            ttf_context,
            "./resources/VCR_OSD_MONO_1.001.ttf",
            128
        )
    })
}
