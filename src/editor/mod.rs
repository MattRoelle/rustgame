use super::constants::*;
use crate::constants::*;
use crate::engine::{
    input::GameInput,
    ui::{view, UIComponent, ViewAttr::*, ViewBuilder},
};
use sdl2::{keyboard::Keycode, render::Canvas, video::Window};
use std::{collections::HashMap, path::Path};

const game_dir: &str = "~/personal/rust/game/";
const game_config_fn: &str = "gameconfig.json";

#[derive(Debug, Copy, Clone)]
pub struct EditorProps {
    entity_select_open: bool,
}

impl Default for EditorProps {
    fn default() -> Self {
        Self {
            entity_select_open: true
        }
    }
}

pub enum EditorActions {
    OpenEntitySelect,
    Up,
    Down
}

pub struct Editor {
    pub ui: UIComponent<EditorProps, EditorActions>,
}

fn entity_select() -> ViewBuilder {
    view().attr(FlexGrow(1.0)).attr(BgColorRGB(255, 0, 0))
}

impl Editor {
    pub fn new() -> Self {
        Self {
            ui: UIComponent::new(
                EditorProps::default(),
                |props, actions| match actions {
                    EditorActions::OpenEntitySelect => {
                        props.entity_select_open = !props.entity_select_open
                    },
                    EditorActions::Up => {
                    },
                    EditorActions::Down => {

                    }
                },
                |props| {
                    let mut root = view()
                        .attr(WidthPx(SCREEN_WIDTH as f32))
                        .attr(HeightPx(SCREEN_HEIGHT as f32));

                    if props.entity_select_open {
                        root.child(entity_select());
                    }

                    root
                },
            ),
        }
    }

    pub fn update(&mut self, inputs: Vec<GameInput>) {
        for input in inputs.iter() {
            match input {
                GameInput::Other(kc) => match kc {
                    Keycode::F1 => self.ui.dispatch(EditorActions::OpenEntitySelect),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
