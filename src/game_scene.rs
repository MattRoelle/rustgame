use crate::{asset_cache::Assets, scene::Scene, sprite::Sprite, input::GameInput};
use crate::constants::*;
use sdl2::{video::Window, render::Canvas};

pub struct GameScene<'a> {
    assets: &'a Assets<'a>,
    player: Sprite<'a>,
    player_direction: f64,
    target_direction: f64,
    switched_t: u128
}

impl<'a> GameScene<'a> {
    pub fn new(assets: &'a Assets) -> Self {
        Self {
            assets,
            player: Sprite::new(&assets.red_rect, 0.0, 0.0, 20.0, 20.0),
            player_direction: 1.0,
            target_direction: 1.0,
            switched_t: 0
        }
    }

    pub fn switch_direction(&mut self, t: u128, direction: f64) {
        if self.target_direction == direction {
            return;
        }

        self.switched_t = t;
        self.target_direction = direction;
    }
}

impl<'a> Scene for GameScene<'a> {
    fn update(&mut self, inputs: Vec<GameInput>, t: u128, dt: f64) {
        for input in inputs {
            match input {
                GameInput::Move(dx, dy) => {
                    self.player.translate(dx*dt * 10.0, dy*dt * 10.0);
                    if dx > 0.0 {
                        self.switch_direction(t, 1.0);
                    } else if dx < 0.0 {
                        self.switch_direction(t, -1.0);
                    }
                },
                _ => {}
            }
        }

        self.player.angle += 1.0 * dt;
        self.player.clamp(0.0, 0.0, SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.player.draw(canvas);
    }
}