use crate::{asset_cache::Assets, input::GameInput, scene::Scene, sprite::Sprite};
use crate::{
    constants::*,
    tiles::Tilemap,
};
use crate::ui::*;
use sdl2::{pixels::Color, render::Canvas, video::Window};
use stretch::{
    geometry::{Rect, Size},
    style::{AlignItems, Dimension, JustifyContent, Style, FlexDirection, Overflow},
};

pub struct GameScene<'a> {
    assets: &'a Assets<'a>,
    player: Sprite<'a>,
    tilemap: Tilemap<'a>,
    ui: UIGraph,
    player_direction: f64,
    target_direction: f64,
    switched_t: u128,
}

impl<'a> GameScene<'a> {
    fn build_ui() -> UIGraph {
        UIGraph::new(
            view()
                .width_px(SCREEN_WIDTH as f32)
                .height_px(SCREEN_HEIGHT as f32)
                .bg_color(Color::RGB(40, 40, 40))
                .flex_direction(FlexDirection::Column)
                .padding_pt_all(10.0)
                .children(&mut vec![
                    view().flex_grow(1.0).bg_color(Color::RGB(120, 120, 120)).margin_pt_all(10.0),
                    view().flex_grow(1.0).bg_color(Color::RGB(120, 200, 120)).margin_pt_all(10.0).children(&mut vec![
                        view().flex_grow(1.0).bg_color(Color::RGB(200, 120, 120)),
                        view().flex_grow(1.0).bg_color(Color::RGB(120, 120, 200)),
                    ]),
                    view().flex_grow(1.0).bg_color(Color::RGB(120, 120, 120)).margin_pt_all(10.0),
                ])
        )
    }

    pub fn new(assets: &'a Assets) -> Self {
        Self {
            assets,
            player: Sprite::new(&assets.red_rect, 0.0, 0.0, 20.0, 20.0),
            player_direction: 1.0,
            target_direction: 1.0,
            switched_t: 0,
            tilemap: Tilemap::new(0.0, 0.0, &assets.test_level, &assets.tilemap),
            ui: GameScene::build_ui(),
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
                    self.player.translate(dx * dt * 10.0, dy * dt * 10.0);
                    if dx > 0.0 {
                        self.switch_direction(t, 1.0);
                    } else if dx < 0.0 {
                        self.switch_direction(t, -1.0);
                    }
                }
                _ => {}
            }
        }

        self.player.angle += 1.0 * dt;
        self.player
            .clamp(0.0, 0.0, SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        // self.tilemap.draw(canvas);
        // self.player.draw(canvas);
        self.ui.draw(canvas);
    }
}
