use super::assets::Assets;
use crate::constants::*;
use crate::ui::ViewAttr::*;
use crate::ui::*;
use crate::{define_class, input::GameInput, scene::Scene, engine::{physics::PhysicsWorld, tiles::Tilemap, rendering::RenderList}};
use sdl2::{render::Canvas, video::Window};

pub struct GameScene<'a> {
    tilemap: Tilemap<'a>,
    physics_world: PhysicsWorld,
    render_list: RenderList,
    assets: &'a mut Assets<'a>
}

impl<'a> GameScene<'a> {
    pub fn new(assets: &'a mut Assets<'a>) -> Self {
        Self {
            tilemap: Tilemap::new(0.0, 0.0, &assets.test_level, &assets.tilemap),
            physics_world: PhysicsWorld::new(),
            render_list: RenderList::new(),
            // assets
        }
    }
}

impl<'a> Scene for GameScene<'a> {
    fn update(&mut self, inputs: Vec<GameInput>, t: u128, dt: f64) {
        for input in inputs {
            match input {
                _ => {}
            }
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.render_list.draw(canvas, &mut self.assets.font);
    }
}
