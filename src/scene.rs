use sdl2::{video::Window, render::Canvas};
use crate::{input::GameInput};

pub trait Scene {
    fn update(&mut self, inputs: Vec<GameInput>, dt: f32);
    fn render(&mut self, canvas: &mut Canvas<Window>);
}