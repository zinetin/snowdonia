pub struct GameState {
    pub paused: bool,
    pub debug: bool,
}

impl GameState {
    pub fn initialize() -> Self {
        Self {
            paused: false,
            debug: false,
        }
    }
}
