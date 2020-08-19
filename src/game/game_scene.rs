use super::assets::Assets;
use crate::ui::*;
use crate::ui::StyleAttr::*;
use crate::{constants::*, tiles::Tilemap};
use crate::{input::GameInput, scene::Scene, sprite::Sprite};
use sdl2::{pixels::Color, render::Canvas, video::Window};
use stretch::style::*;

#[derive(Debug, Copy, Clone)]
pub struct UIProps {
    count: i32,
}

static FULLSCREEN: &'static [StyleAttr] = &[
    WidthPx(SCREEN_WIDTH as f32),
    HeightPx(SCREEN_HEIGHT as f32)
];

static RED_BG: &'static [StyleAttr] = &[
    BgColorRGB(200, 100, 100)
];

static BLUE_BG: &'static [StyleAttr] = &[
    BgColorRGB(200, 100, 100)
];

struct GameSceneUI {
    props: UIProps,
    graph: UIGraph,
}

impl GameSceneUI {
    pub fn new() -> Self {
        let initial_props = UIProps { count: 0 };

        Self {
            graph: UIGraph::new(Self::render(initial_props)),
            props: initial_props,
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
        view()
            .class(FULLSCREEN)
            .style(BgColorRGB(200, 100, 100))
            .children(&mut vec![
                view()
                    .style(WidthPx(100.0))
                    .style(HeightPx(100.0))
                    .style(BgColorRGB(100, 100, 200))

            ])
            .clone()
    }

    fn set_ui_graph(&mut self, graph: UIGraph) {
        self.graph = graph;
    }

    fn set_props(&mut self, props: Self::Props) {
        self.set_ui_graph(UIGraph::new(Self::render(props).clone()))
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
                GameInput::Jump => {
                    self.ui.increment();
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
