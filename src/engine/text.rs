use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, TextureAccess, TextureCreator, Texture},
    ttf::{Font, Sdl2TtfContext},
    video::{Window, WindowContext},
};
use std::collections::HashMap;

const ASCII_START: char = 32u8 as char;
const ASCII_END: char = 127u8 as char;
const NUM_GLYPHS: u8 = 127 - 32;

pub struct TextAtlas<'a> {
    font: Font<'a, 'static>,
    texture_position_map: HashMap<char, Rect>,
    pub atlas_texture: Texture<'a>
}

impl<'a> TextAtlas<'a> {
    pub fn new<'b>(
        canvas: &mut Box<Canvas<Window>>,
        texture_creator: &'a TextureCreator<WindowContext>,
        ttf_context: &'a Sdl2TtfContext,
        font_path: &str,
        base_font_size: u16,
    ) -> Self {
        let font = ttf_context.load_font(font_path, base_font_size).unwrap();
        let mut atlas_texture = texture_creator
            .create_texture_target(
                PixelFormatEnum::RGB24,
                (base_font_size * NUM_GLYPHS as u16) as u32,
                base_font_size as u32,
            )
            .unwrap();

        let mut texture_position_map: HashMap<char, Rect> = HashMap::new();
        let mut glyph_x = 0u32;

        canvas.with_texture_canvas(&mut atlas_texture, |atlas_texture_canvas| {
            atlas_texture_canvas.clear();

            for ascii_code in ASCII_START..ASCII_END {
                let char_surf = font
                    .render_char(ascii_code.into())
                    .blended(Color::RGB(255, 255, 255))
                    .unwrap();

                let char_texture = texture_creator
                    .create_texture_from_surface(&char_surf)
                    .unwrap();

                let char_rect = char_surf.rect();
                let target_rect = Rect::new(
                    char_rect.x() + glyph_x as i32,
                    char_rect.y(),
                    char_rect.width(),
                    char_rect.height(),
                );

                // dbg!(ascii_code as char);
                // dbg!(target_rect);

                texture_position_map.insert(ascii_code, target_rect);
                atlas_texture_canvas.copy(&char_texture, None, target_rect).unwrap();

                glyph_x += char_surf.rect().width();
            }
        }).unwrap();

        Self {
            font,
            texture_position_map,
            atlas_texture
        }
    }

    pub fn draw_char(&mut self, canvas: &mut Canvas<Window>, c: char, x: i32, y: i32, size: f32) -> Rect {
        let src_rect = *self.texture_position_map.get(&c).expect("Invalid character. Must be an ASCII char between 32 and 127");

        let width = (src_rect.width() as f32 * size).round() as u32;
        let height = (src_rect.height() as f32 * size).round() as u32;

        let target_rect = Rect::new(x, y, width, height);
        
        canvas.copy(
            &self.atlas_texture,
            Some(src_rect),
            Some(target_rect)
        ).unwrap();

        target_rect
    }

    pub fn draw_str(&mut self, canvas: &mut Canvas<Window>, s: &str, x: i32, y: i32, width: u32, height: u32, size: f32, line_height: f32) {
        let mut cursor_x = 0;
        let mut cursor_y = 0;

        for c in s.chars() {
            let glyph_rect = self.draw_char(canvas, c, cursor_x, cursor_y, size);
            cursor_x += glyph_rect.width() as i32;
            if cursor_x > width as i32 {
                cursor_x = 0;
                cursor_y += (glyph_rect.height() as f32 * line_height) as i32;
            }
        }
    }
}
