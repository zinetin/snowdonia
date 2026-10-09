use crate::vars::PAUSE_FREEZE_TIME;

// Declares the state of the game
pub struct GameState {
    // Is the game paused
    pub paused: bool,
    // Is the game in debug mode
    pub debug: bool,
    // Will the game force a frame to happen even if the player tries to pause
    pub guarenteed_step: bool,

    // Time when the game is frozen, but the pause menu is not up, used when the player leaves a
    // pause, or when interacting with certain objects.
    pub freeze_frames: f32,
}

// Initialize a default game state
impl GameState {
    pub fn initialize() -> Self {
        Self {
            paused: false,
            debug: false,
            freeze_frames: PAUSE_FREEZE_TIME,
            guarenteed_step: false,
        }
    }

    pub fn toggle_pause(&mut self, pause_is_pressed: bool) {
        // If the player presses (or holds) the pause button, and the player is allowed to pause,
        // then the game pauses
        if pause_is_pressed && !self.guarenteed_step && !self.paused {
            self.paused = true;
            self.freeze_frames = PAUSE_FREEZE_TIME;
            self.guarenteed_step = true;
        }
        // If the player pressed (or holds the pause button), then the game unpauses
        else if self.paused && pause_is_pressed {
            self.paused = false;
        }
    }
}
