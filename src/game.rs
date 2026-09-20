use crate::game::entities::entity_trait::Entity;
use crate::state::{AppState, AppStateScreen};
use macroquad::prelude::*;

pub mod entities;

pub async fn game(menu_state: AppState) {
    let mut entities: Vec<Box<dyn Entity>> = vec![];

    while menu_state.menu_scene == AppStateScreen::Game {
        let dt = get_frame_time();

        for entity in entities.iter_mut() {
            entity.update(dt);
        }

        for entity in entities.iter() {
            entity.draw();
        }

        next_frame().await;
    }
}
