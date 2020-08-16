use crate::{asset_cache::Assets, scene::Scene, sprite::Sprite, input::GameInput};
use crate::constants::*;
use sdl2::{video::Window, render::Canvas};

pub struct GameScene<'a> {
    assets: &'a Assets<'a>,
    player: Sprite<'a>
}

impl<'a> GameScene<'a> {
    pub fn new(assets: &'a Assets) -> Self {
        Self {
            assets,
            player: Sprite::new(&assets.red_rect, 0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
        }
    }
}

impl<'a> Scene for GameScene<'a> {
    fn update(&mut self, inputs: Vec<GameInput>, dt: f32) {
        for input in inputs {
            match input {
                GameInput::Move(dx, dy) => {
                    self.player.translate(dx*dt, dy*dt);
                },
                _ => {}
            }
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.player.draw(canvas);
    }
}