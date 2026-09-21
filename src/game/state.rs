// Declares the state of the game
pub struct GameState {
    pub paused: bool,
    pub debug: bool,
}

// Initialize a default game state
impl GameState {
    pub fn initialize() -> Self {
        Self {
            paused: false,
            debug: false,
        }
    }
}
