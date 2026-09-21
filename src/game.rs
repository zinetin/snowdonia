use crate::game::entities::entity_trait::Entity;
use crate::game::level::Level;
use crate::game::state::GameState;
use crate::input::{Action, InputMap};
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

        // Poll the users inputs
        let inputs = bindings.poll();

        // Check if the accumulator is higher than a certain maximum (Ie if there is a big lag
        // spike, this pauses the game and resets the accumulator before any physics is calculated
        // to try to avoid deaths due to sudden lag)
        if accumulator >= MAX_ACCUMULATOR {
            game_state.paused = true;
            accumulator = 0.0;
        }

        if inputs.pressed(Action::Pause) {
            game_state.paused = !game_state.paused
        }

        if inputs.pressed(Action::Debug) {
            game_state.debug = !game_state.debug
        }

        level.timescale = 0.5;

        // Checks if the game is paused, and if the game is not paused, it updates the physics,
        // then draws the game.
        if game_state.paused {
        } else {
            while accumulator >= FRAME {
                for entity in entities.iter_mut() {
                    entity.update(
                        FRAME * level.timescale,
                        &inputs,
                        &level.screens[level.current_screen].tilemap,
                    );
                }

                accumulator -= FRAME
            }
        }

        // The level gets drawn regardless of whether the game is paused or not
        clear_background(BLACK);

        draw_level(&entities, &level, &game_state);

        next_frame().await;
    }
}

// Function to draw the level to tidy things up in the main game loop
pub fn draw_level(entities: &Vec<Box<dyn Entity>>, level: &Level, game_state: &GameState) {
    for entity in entities.iter() {
        entity.draw(game_state);
    }
    level.draw();
}
