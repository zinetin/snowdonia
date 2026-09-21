pub mod game;
pub mod input;
pub mod state;
pub mod vars;

#[macroquad::main("Snowdonia")]
async fn main() {
    let menu_state = state::AppState::init();
    let bindings = input::InputMap::defaults();
    game::game(menu_state, bindings).await;
}
