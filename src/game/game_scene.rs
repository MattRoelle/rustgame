use super::assets::Assets;
use crate::constants::*;
use crate::ui::ViewAttr::*;
use crate::ui::*;
use crate::{define_class, input::GameInput, scene::Scene, engine::{physics::PhysicsWorld, tiles::Tilemap}};
use sdl2::{render::Canvas, video::Window};

pub struct GameScene<'a> {
    tilemap: Tilemap<'a>,
    physics_world: PhysicsWorld
}

impl<'a> GameScene<'a> {
    pub fn new(assets: &'a Assets<'a>) -> Self {
        Self {
            tilemap: Tilemap::new(0.0, 0.0, &assets.test_level, &assets.tilemap),
            physics_world: PhysicsWorld::new()
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
        self.tilemap.draw(canvas);
    }
}
