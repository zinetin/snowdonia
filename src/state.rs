// All of the stuff about the state of the application as a whole.
#[derive(PartialEq)]
pub enum AppStateScreen {
    Menu,
    Game,
}

pub struct AppState {
    pub menu_scene: AppStateScreen,
}

impl AppState {
    pub fn init() -> Self {
        Self {
            menu_scene: AppStateScreen::Game,
        }
    }
}
