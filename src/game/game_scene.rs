use super::assets::Assets;
use crate::ui::*;
use crate::ui::StyleAttr::*;
use crate::{constants::*};
use crate::{input::GameInput, scene::Scene, define_class};
use sdl2::{render::Canvas, video::Window};

#[derive(Debug, Copy, Clone)]
pub struct UIProps {
    color: u8,
    count: u32,
}

enum UIActions {
    Increment,
}

define_class!(FULLSCREEN, [
    WidthPx(SCREEN_WIDTH as f32),
    HeightPx(SCREEN_HEIGHT as f32)
]);

pub struct GameScene<'a> {
    assets: &'a Assets<'a>,
    ui: UIComponent<UIProps, UIActions>,
}

impl<'a> GameScene<'a> {
    pub fn new(assets: &'a Assets) -> Self {
        Self {
            assets,
            ui: UIComponent::new(
                UIProps { color: 0, count: 0 },
                |props, action| {
                    match action {
                        UIActions::Increment => {
                            props.color += 1;
                            props.color %= 240;
                            props.count += 1;
                            props.count %= 20;
                        }
                    }
                },
                |props| {
                view()
                    .class(FULLSCREEN)
                    .style(BgColorRGB(props.color, props.color, props.color))
                    .style(PaddingPx(20.0, 20.0, 20.0, 20.0))
                    .child(
                        view()
                            .style(FlexGrow(1.0))
                            .style(BgColorRGB(200, 100, 100))
                            // .children(
                            //     &mut ((0..props.count).map(|i| {
                            //         view()
                            //             .style(FlexGrow(1.0))
                            //             .style(BgColorRGB(100, 200, 100))
                            //             .clone()
                            //     })
                            // )
                    )
                    .clone()
            }),
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
        self.ui.draw(canvas);
    }
}
