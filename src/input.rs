use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Input {
    Key(KeyCode),
    Mse(MouseButton),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Jump,
    Dash,
    Roll,
    Grab,
    Pause,
    Debug,
}

impl Input {
    pub fn is_down(&self) -> bool {
        match *self {
            Input::Key(k) => is_key_down(k),
            Input::Mse(b) => is_mouse_button_down(b),
        }
    }

    pub fn is_pressed(&self) -> bool {
        match *self {
            Input::Key(k) => is_key_pressed(k),
            Input::Mse(b) => is_mouse_button_pressed(b),
        }
    }

    pub fn is_released(&self) -> bool {
        match *self {
            Input::Key(k) => is_key_released(k),
            Input::Mse(b) => is_mouse_button_released(b),
        }
    }
}

pub struct InputMap {
    input_map: HashMap<Input, Action>,
}

impl InputMap {
    pub fn defaults() -> Self {
        let mut input_map: HashMap<Input, Action> = HashMap::new();
        input_map.insert(Input::Key(KeyCode::E), Action::Up);
        input_map.insert(Input::Key(KeyCode::S), Action::Left);
        input_map.insert(Input::Key(KeyCode::D), Action::Down);
        input_map.insert(Input::Key(KeyCode::F), Action::Right);
        input_map.insert(Input::Key(KeyCode::GraveAccent), Action::Debug);

        Self { input_map }
    }
}
