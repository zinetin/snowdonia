use crate::game::entities::entity_trait::Entity;
use crate::game::level::Level;
use crate::game::state::GameState;
use crate::input::InputMap;
use crate::state::{AppState, AppStateScreen};
use crate::vars::{FRAME, MAX_ACCUMULATOR};
use macroquad::prelude::*;

pub mod entities;
pub mod level;
pub mod state;

pub async fn game(menu_state: AppState, bindings: InputMap) {
    // Load the level (Just a debug level right now)
    let mut level: Level = Level::debug();

    // Load the default entity states from the level data
    let mut entities: Vec<Box<dyn Entity>> = vec![Box::new(level.init_player_state.clone())];

    // Declare the accumulator for the game loop
    let mut accumulator: f32 = 0.0;

    // Declare the game state
    let mut game_state = GameState::initialize();

    while menu_state.menu_scene == AppStateScreen::Game {
        // Get the delta time
        let dt = get_frame_time();
        // Increase the accumulator by the delta time
        accumulator += dt;

        // Check if the accumulator is higher than a certain maximum (Ie if there is a big lag
        // spike, this pauses the game and resets the accumulator before any physics is calculated
        // to try to avoid deaths due to sudden lag)
        if accumulator >= MAX_ACCUMULATOR {
            game_state.paused = true;
            accumulator = 0.0;
        }

        // Checks if the game is paused, and if the game is not paused, it updates the physics,
        // then draws the game.
        if game_state.paused {
        } else {
            while accumulator >= FRAME {
                let inputs = bindings.poll();

                for entity in entities.iter_mut() {
                    entity.update(FRAME, &inputs);
                }

                accumulator -= FRAME
            }
        }

        // The level gets drawn regardless of whether the game is paused or not
        clear_background(BLACK);
        draw_level(&entities, &level);

        next_frame().await;
    }
}

// Function to draw the level to tidy things up in the main game loop
pub fn draw_level(entities: &Vec<Box<dyn Entity>>, level: &Level) {
    for entity in entities.iter() {
        entity.draw();
    }
    level.draw();
}
