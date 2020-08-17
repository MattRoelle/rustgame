use crate::ui::*;
use crate::engine::*;
use crate::{input::GameInput, scene::Scene, sprite::Sprite};
use crate::{block, constants::*, tiles::Tilemap, fullscreen};
use sdl2::{pixels::Color, render::Canvas, video::Window};
use stretch::{
    geometry::{Rect, Size},
    style::{AlignItems, Dimension, FlexDirection, JustifyContent, Overflow, Style},
};
use super::assets::Assets;


#[derive(Debug, Copy, Clone)]
pub struct UIProps {
    count: i32,
}

struct GameSceneUI {
    props: UIProps,
    graph: UIGraph
}

impl GameSceneUI {
    pub fn new() -> Self {
        let initial_props = UIProps {
            count: 0
        };

        Self {
            graph: UIGraph::new(Self::render(initial_props)),
            props: initial_props
        }
    }

    pub fn increment(&mut self) {
        self.props.count += 1;
        self.props.count %= 8;
        self.set_props(self.props);
    }
}

impl UIComponent for GameSceneUI {
    type Props = UIProps;

    fn render(props: UIProps) -> ViewBuilder {
        let mut top_children = {
            let mut ret = Vec::new();
            for i in 0..props.count {
                let block = block!(120, 120, 120)
                    .clone()
                    .border_width(2)
                    .border_color(Color::RGB(255, 255, 255))
                    .bg_color(Color::RGB((i* 20) as u8, (i* 20) as u8, (i*20) as u8))
                    .clone();

                ret.push(block)
            }
            ret
        };

        let mut bottom_children = {
            let mut ret = Vec::new();
            for i in 0..(7 - props.count) {
                let block = block!(120, 120, 120).clone();
                ret.push(block)
            }
            ret
        };

        fullscreen!()
            .bg_color(Color::RGB(40, 40, 240))
            .flex_direction(FlexDirection::Column)
            .padding_pt_all(10.0)
            .children(&mut vec![
                block!(80, 80, 80)
                    .children(&mut top_children.iter_mut().collect()),
                block!(80, 80, 80)
                    .flex_direction(FlexDirection::Column)
                    .children(&mut bottom_children.iter_mut().collect()),
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
                },
                GameInput::Jump => {
                    self.ui.increment();
                },
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
