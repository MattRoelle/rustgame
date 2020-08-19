use super::assets::Assets;
use crate::constants::*;
use crate::ui::ViewAttr::*;
use crate::ui::*;
use crate::{define_class, input::GameInput, scene::Scene};
use sdl2::{render::Canvas, video::Window};

#[derive(Debug, Copy, Clone)]
pub struct UIProps {
    count: u32,
}

enum UIActions {
    Increment,
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
                UIProps { count: 0 },
                |props, action| match action {
                    UIActions::Increment => {
                        props.count += 1;
                        props.count %= 240;
                    }
                },
                |props| {
                    let color = props.count as u8;
                    view()
                        .class(FULLSCREEN)
                        .attr(BgColorRGB(color, color, color))
                        .attr(PaddingPx(20.0, 20.0, 20.0, 20.0))
                        .child(
                            view()
                                .attr(FlexGrow(1.0))
                                .attr(BgColorRGB(180, 100, 100))
                                .children(
                                    &mut ((0..(props.count))
                                        .map(|i| {
                                            view()
                                                .attr(FlexGrow(1.0))
                                                .attr(MarginPx(5.0, 5.0, 5.0, 5.0))
                                                .attr(BgColorRGB(240, 100, 100))
                                                .attr(FontSize(0.5))
                                                .text("This is a test. lorum ipsum dolor")
                                        })
                                        .collect()),
                                ),
                        )
                },
            ),
        }
    }
}

impl<'a> Scene for GameScene<'a> {
    fn update(&mut self, inputs: Vec<GameInput>, t: u128, dt: f64) {
        self.ui.dispatch(UIActions::Increment);
        // for input in inputs {
        //     match input {
        //         GameInput::Jump => {
        //             self.ui.dispatch(UIActions::Increment)
        //         }
        //         _ => {}
        //     }
        // }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.ui.draw(canvas, &mut self.assets.font);
    }
}
