pub trait Entity {
    fn update(&mut self, dt: f32);
    fn draw(&self);
}
