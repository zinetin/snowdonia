pub mod game;
pub mod state;

#[macroquad::main("Snowdonia")]
async fn main() {
    let menu_state = state::AppState::init();
    game::game(menu_state).await;
}
