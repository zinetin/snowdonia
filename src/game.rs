use crate::game::entities::entity_trait::Entity;
use crate::game::state::GameState;
use crate::input::InputMap;
use crate::state::{AppState, AppStateScreen};
use crate::vars::{FRAME, MAX_ACCUMULATOR};
use macroquad::prelude::*;

pub mod entities;
pub mod state;

pub async fn game(menu_state: AppState, bindings: InputMap) {
    let mut entities: Vec<Box<dyn Entity>> = vec![];
    let mut accumulator: f32 = 0.0;

    let mut game_state = GameState::initialize();

    while menu_state.menu_scene == AppStateScreen::Game {
        let dt = get_frame_time();
        accumulator += dt;

        if accumulator >= MAX_ACCUMULATOR {
            game_state.paused = true;
            accumulator = 0.0;
        }

        if game_state.paused {
        } else {
            while accumulator >= FRAME {
                for entity in entities.iter_mut() {
                    entity.update(dt);
                }

                for entity in entities.iter() {
                    entity.draw();
                }
                accumulator -= FRAME
            }
        }

        next_frame().await;
    }
}
