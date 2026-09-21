use crate::game::level::tile::TileMap;
use crate::game::GameState;
use crate::input::ActionState;

pub trait Entity {
    fn update(&mut self, dt: f32, inputs: &ActionState, tilemap: &TileMap);
    fn draw(&self, game_state: &GameState);
}
