use sdl2::{keyboard::Keycode};
use std::collections::HashMap;

pub enum GameInput {
    Move(f64, f64),
    Jump,
    None,
}

pub struct InputManager {
    inputs: Vec<GameInput>, 
    pub keyboard_states: HashMap<Keycode, bool>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            inputs: vec![],
            keyboard_states: HashMap::new()
        }
    }

    pub fn process_keydown(&mut self, keycode: Keycode) {
        self.keyboard_states.insert(keycode, true);
    }

    pub fn process_keyup(&mut self, keycode: Keycode) {
        self.keyboard_states.insert(keycode, false);

        match keycode {
            Keycode::Space => {
                self.inputs.push(GameInput::Jump)
            },
            _ => {}
        }
    }

    pub fn is_key_down(&self, keycode: Keycode) -> bool {
            self.keyboard_states.contains_key(&keycode)
        && *self.keyboard_states.get(&keycode).unwrap()
    }

    pub fn collect_game_inputs(&mut self) -> Vec<GameInput> {
        let mut dx: f64 = 0.0;
        let mut dy: f64 = 0.0;

        if self.is_key_down(Keycode::W) || self.is_key_down(Keycode::Up) { dy = -1.0; }
        if self.is_key_down(Keycode::A) || self.is_key_down(Keycode::Left) { dx = -1.0; }
        if self.is_key_down(Keycode::S) || self.is_key_down(Keycode::Down) { dy = 1.0; }
        if self.is_key_down(Keycode::D) || self.is_key_down(Keycode::Right) { dx = 1.0; }

        if dx.abs() > 0.0 || dy.abs() > 0.0 {
            let mag = (dy.powf(2.0) + dx.powf(2.0)).sqrt();
            let theta = dy.atan2(dx);
            self.inputs.push(GameInput::Move(theta.cos() * mag, theta.sin() * mag))
        }

        return self.inputs.drain(..).collect();
    }
}
