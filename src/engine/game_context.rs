pub trait GameObject {
    fn tags(&self) -> Vec<String>;
    fn update(&mut self);
}

type GameObjectRef = Box<dyn GameObject>;

pub struct GameContext {
    game_objects: Vec<Box<dyn GameObject>>
}

impl GameContext {
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
}