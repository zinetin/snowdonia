use crate::game::entities::{entity_trait::Entity, player::Player};
use crate::game::level::tile::{TileMap, TileType};

pub mod tile;

#[derive(Default)]
pub struct Level {
    pub init_player_state: Player,
    pub screens: Vec<Screen>,
    pub current_screen: usize,
}

#[derive(Default)]
pub struct Screen {
    pub tilemap: TileMap,
    //    pub init_entities: Vec<Box<dyn Entity>>,
}

impl Level {
    pub fn debug() -> Self {
        let mut tilemap = TileMap::empty(48, 22);

        tilemap.fill_rect(0, 13, 48, 2, TileType::Debug);
        tilemap.fill_rect(5, 10, 4, 2, TileType::Debug);
        tilemap.fill_rect(22, 7, 5, 2, TileType::Debug);

        Self {
            init_player_state: Player {
                ..Player::default()
            },
            screens: vec![Screen { tilemap }],
            ..Level::default()
        }
    }

    pub fn draw(&self) {
        self.screens[self.current_screen].tilemap.draw();
    }
}
