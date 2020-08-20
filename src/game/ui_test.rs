use super::assets::Assets;
use crate::constants::*;
use crate::ui::ViewAttr::*;
use crate::ui::*;
use crate::{define_class, input::GameInput, scene::Scene};
use sdl2::{render::Canvas, video::Window};

#[derive(Debug, Copy, Clone)]
pub struct UIProps {
    selected_idx: i8,
}

enum UIActions {
    MoveCursor(i8),
}

define_class!(
    FULLSCREEN,
    [WidthPx(SCREEN_WIDTH as f32), HeightPx(SCREEN_HEIGHT as f32)]
);

pub struct GameScene<'a> {
    assets: Assets<'a>,
    ui: UIComponent<UIProps, UIActions>,
}

impl<'a> GameScene<'a> {
    pub fn new(assets: Assets<'a>) -> Self {
        Self {
            assets,
            ui: UIComponent::new(
                UIProps { selected_idx: 0 },
                |props, action| match action {
                    UIActions::MoveCursor(direction) => {
                        props.selected_idx = (props.selected_idx + direction).max(0).min(3);
                    }
                },
                |props| {
                    let menu_options = vec![
                        "Attack",
                        "Defend",
                        "Inventory",
                        "Flee",
                    ];

                    fn menuitem(text: &str, selected: bool) -> ViewBuilder {
                        view()
                            .attr(MarginPx(10.0, 10.0, 10.0, 10.0))
                            .attr_if(BgColorRGB(180, 180, 180), selected)
                            .attr_if(BgColorRGB(120, 120, 120), !selected)
                            .attr(PaddingPx(20.0, 20.0, 20.0, 20.0))
                            .child(
                                view()
                                    .attr(FlexGrow(1.0))
                                    .attr(FontSize(0.5))
                                    .attr(HeightPx(64.0))
                                    .text(text),
                            )
                    }

                    view()
                        .class(FULLSCREEN)
                        .attr(BgColorRGB(0, 0, 0))
                        .attr(PaddingPx(20.0, 20.0, 20.0, 20.0))
                        .children(&mut vec![
                            view() // Left column
                                .attr(FlexGrow(0.4))
                                .attr(FlexDirection(stretch::style::FlexDirection::Column))
                                .attr(BgColorRGB(100, 100, 100))
                                .attr(MarginPx(10.0, 10.0, 10.0, 10.0))
                                .children(
                                    &mut ((0..(menu_options.len()))
                                        .map(|i| {
                                            menuitem(menu_options[i], props.selected_idx == i as i8)
                                        })
                                        .collect()),
                                ),
                            view() // Center view
                                .attr(FlexGrow(1.0))
                                .attr(FlexDirection(stretch::style::FlexDirection::Column))
                                .attr(BgColorRGB(100, 100, 100))
                                .attr(MarginPx(10.0, 10.0, 10.0, 10.0)),
                            view() // Right Column
                                .attr(FlexGrow(0.4))
                                .attr(MarginPx(10.0, 10.0, 10.0, 10.0))
                                .attr(FlexDirection(stretch::style::FlexDirection::Column))
                                .attr(BgColorRGB(100, 100, 100))
                        ])
                },
            ),
        }
    }
}

impl<'a> Scene for GameScene<'a> {
    fn update(&mut self, inputs: Vec<GameInput>, t: u128, dt: f64) {
        for input in inputs {
            match input {
                GameInput::Up => self.ui.dispatch(UIActions::MoveCursor(-1)),
                GameInput::Down => self.ui.dispatch(UIActions::MoveCursor(1)),
                _ => {}
            }
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.ui.draw(canvas, &mut self.assets.font);
    }
}
