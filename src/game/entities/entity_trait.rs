use crate::input::ActionState;

pub trait Entity {
    fn update(&mut self, dt: f32, inputs: &ActionState);
    fn draw(&self);
}
