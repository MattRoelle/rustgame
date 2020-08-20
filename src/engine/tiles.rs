use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};
use crate::geometry::Vec2;

pub struct Tilemap<'a> {
    map: tiled::Map,
    tileset: Tileset<'a>,
    sdl_rects: Vec<(sdl2::rect::Rect, sdl2::rect::Rect)>,
    pos: Vec2
}

pub struct Tileset<'a> {
    map_width: u32,
    map_height: u32,
    tile_width: u32,
    tile_height: u32,
    first_gid: u32, 
    texture: &'a Texture<'a>,
    texture_src_rects: Vec<sdl2::rect::Rect>
}

impl<'a> Tilemap<'a> {
    pub fn new(x: f64, y: f64, map: &'a tiled::Map, tileset_texture: &'a Texture<'a>) -> Self {
        let map_tileset = &map.tilesets[0];
        let img = map_tileset.images.first().expect("No associated image in the tileset");
        let tile_width = map_tileset.tile_width;
        let tile_height = map_tileset.tile_height;
        let map_width = img.width as u32 / map_tileset.tile_width;
        let map_height = img.height as u32 / map_tileset.tile_height;

        let mut texture_src_rects = Vec::new();

        for y in 0..map_height {
            for x in 0..map_width {
                texture_src_rects.push(sdl2::rect::Rect::new((x * tile_width) as i32, (y * tile_height) as i32, tile_width, tile_height))
            }
        }

        let tileset : Tileset<'a> = Tileset {
            map_width,
            map_height,
            tile_width,
            tile_height,
            first_gid: map_tileset.first_gid,
            texture: tileset_texture,
            texture_src_rects
        };

        Self {
            pos: Vec2::new(x, y),
            map: map.clone(),
            sdl_rects: Tilemap::get_sdl_rects(x, y, map.clone(), &tileset),
            tileset
        }
    }

    fn update_sdl_rects(&mut self) {
        self.sdl_rects = Tilemap::get_sdl_rects(self.pos.x, self.pos.y, self.map.clone(), &self.tileset);
    }

    fn get_sdl_rects(x: f64, y: f64, map: tiled::Map, tileset: &Tileset) -> Vec<(sdl2::rect::Rect, sdl2::rect::Rect)> {
        let mut sdl_rects = Vec::new();

        for layer in map.layers.iter() {
            for ty in 0..layer.tiles.len() {
                let row = &layer.tiles[ty];
                for tx in 0..row.len() {
                    let tile = row[tx];
                    let src_idx = (tile.gid - tileset.first_gid) as usize;
                    sdl_rects.push((
                        tileset.texture_src_rects[src_idx],
                        sdl2::rect::Rect::new(x as i32 + (tx as i32 * tileset.tile_width as i32), y as i32 + (ty as i32 * tileset.tile_height as i32), tileset.tile_width, tileset.tile_height)
                    ));
                }
            }
        }

        return sdl_rects;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for pair in self.sdl_rects.iter() {
            canvas.copy(&self.tileset.texture, pair.0, pair.1).expect("canvas.copy call failed")
        }
    }
}
