use crate::{game::{assets::Assets, player::Player}, game_object_types::GameObjectType};
use super::rendering::RenderList;

pub trait GameObject {
    // type TProps;
    // fn init(&mut self, props: Self::TProps);
    fn tags(&self) -> Vec<String>;
    fn update(&mut self);
    fn set_pos(&mut self, x: f32, y: f32);
}

type GameObjectRef = Box<dyn GameObject>;

pub struct GameContext<'a> {
    game_objects: Vec<Box<dyn GameObject>>,
    assets: Assets<'a>,
    render_list: RenderList
}

pub struct Level {
    game_objects: Vec<GameObjectConfig>
}

pub struct GameObjectConfig {
    game_object_type: GameObjectType,
    x: f32,
    y: f32,
}

impl<'a> GameContext<'a> {
    pub fn new(assets: Assets<'a>) -> Self {
        Self {
            game_objects: vec![],
            assets,
            render_list: RenderList::new()
        }
    }

    pub fn get_objects_by_tag(&mut self, tag: String) -> Vec<&mut GameObjectRef> {
        let mut ret = vec![];

        for go in self.game_objects.iter_mut() {
            let tags = go.tags();
            if tags.contains(&tag) {
                ret.push(go);
            }
        }

        return ret;
    }

    pub fn spawn(&mut self, object: GameObjectRef) {
        self.game_objects.push(object);
    }

    pub fn load_level(&mut self, level: &Level) {
        self.game_objects.clear();
        self.render_list.clear();

        for config in level.game_objects.iter() {
            match config.game_object_type {
                GameObjectType::Player(props) => {
                    self.game_objects.push(Box::new(Player::new(props, &self.assets)));
                }
            }
        }
    }
}