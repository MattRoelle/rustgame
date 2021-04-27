use crate::engine::{sprite::Sprite, game_context::GameObject};
use super::assets::Assets;

#[derive(Debug, Copy, Clone)]
pub struct PlayerProps {

}

pub struct Player<'a> {
    sprite: Sprite<'a>
}

impl<'a> Player<'a> {
    pub fn new(props: PlayerProps, assets: &'a Assets<'a>) -> Self {
        Self {
            sprite: Sprite::new(&assets.green_rect, 0.0 ,0.0, 32.0, 32.0)
        }
    }
}

impl<'a> GameObject for Player<'a> {
    fn tags(&self) -> Vec<String> {
        vec![]
    }

    fn update(&mut self) {
    }

    fn set_pos(&mut self, x: f32, y: f32) {
    }
}