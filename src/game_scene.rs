use crate::ui::*;
use crate::{asset_cache::Assets, input::GameInput, scene::Scene, sprite::Sprite};
use crate::{block, constants::*, tiles::Tilemap, fullscreen};
use sdl2::{pixels::Color, render::Canvas, video::Window};
use stretch::{
    geometry::{Rect, Size},
    style::{AlignItems, Dimension, FlexDirection, JustifyContent, Overflow, Style},
};


pub struct UIProps {
    count: i32,
}

struct GameSceneUI {
    graph: UIGraph
}

impl GameSceneUI {
    pub fn new() -> Self {
        Self {
            graph: UIGraph::new(Self::render(UIProps {
                count: 1
            }))
        }
    }
}

impl UIComponent<UIProps> for GameSceneUI {
    fn render(props: UIProps) -> ViewBuilder {
        fullscreen!()
            .bg_color(Color::RGB(40, 40, 40))
            .flex_direction(FlexDirection::Column)
            .padding_pt_all(10.0)
            .children(&mut vec![
                block!(80, 80, 80)
                    .children(&mut vec![block!(120, 120, 120), block!(120, 120, 120)]),
                block!(80, 80, 80),
            ])
            .clone()
    }

    fn set_ui_graph(&mut self, graph: UIGraph) {
        self.graph = graph;
    }
}

pub struct GameScene<'a> {
    assets: &'a Assets<'a>,
    player: Sprite<'a>,
    tilemap: Tilemap<'a>,
    ui: GameSceneUI,
    player_direction: f64,
    target_direction: f64,
    switched_t: u128,
}

impl<'a> GameScene<'a> {
    pub fn new(assets: &'a Assets) -> Self {
        Self {
            assets,
            player: Sprite::new(&assets.red_rect, 0.0, 0.0, 20.0, 20.0),
            player_direction: 1.0,
            target_direction: 1.0,
            switched_t: 0,
            tilemap: Tilemap::new(0.0, 0.0, &assets.test_level, &assets.tilemap),
            ui: GameSceneUI::new(),
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
        self.ui.graph.draw(canvas);
    }
}
