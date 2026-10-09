use crate::game::entities::entity_trait::Entity;
use crate::game::game_state::GameState;
use crate::game::level::Level;
use crate::input::{Action, InputMap};
use crate::state::{AppState, AppStateScreen};
use crate::vars::{FRAME, MAX_ACCUMULATOR};
use macroquad::prelude::*;

pub mod entities;
pub mod game_state;
pub mod level;

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

        // Poll the users inputs
        let inputs = bindings.poll();

        // Pauses the game on pause action
        game_state.toggle_pause(inputs.pressed(Action::Pause));

        // Opens the debug box on debug action
        if inputs.pressed(Action::Debug) {
            game_state.debug = !game_state.debug
        }

        level.timescale = 1.0;

        // Increase the accumulator by the delta time if the game is not paused
        if !game_state.paused {
            accumulator += dt;
        }

        // Check if the accumulator is higher than a certain maximum (Ie if there is a big lag
        // spike, this pauses the game and resets the accumulator before any physics is calculated
        // to try to avoid deaths due to sudden lag)
        if accumulator >= MAX_ACCUMULATOR {
            game_state.toggle_pause(true);
            accumulator = 0.0;
        }

        while accumulator >= FRAME {
            // If the game is paused, no physics calculation should happen.
            if game_state.paused {
                for entity in entities.iter_mut() {
                    entity.update_buffers(FRAME * level.timescale, &inputs);
                }
            }
            // Either updates the physics or decreases the freeze frames
            else {
                // If the game still has the freeze frames, it decreases the freeze frames
                if game_state.freeze_frames > 0.0 {
                    game_state.freeze_frames -= FRAME;
                    for entity in entities.iter_mut() {
                        entity.update_buffers(FRAME * level.timescale, &inputs);
                    }
                }
                // Otherwise it updates the physics
                else {
                    game_state.guarenteed_step = false;
                    for entity in entities.iter_mut() {
                        entity.update(
                            FRAME * level.timescale,
                            &inputs,
                            &level.screens[level.current_screen].tilemap,
                        );
                    }
                }
            }
            accumulator -= FRAME
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
