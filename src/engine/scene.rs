use sdl2::{video::Window, render::Canvas};
use crate::{input::GameInput};

pub trait Scene {
    fn update(&mut self, inputs: Vec<GameInput>, t: u128, dt: f64);
    fn render(&mut self, canvas: &mut Canvas<Window>);
}