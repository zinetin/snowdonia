use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};

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

#[derive(Default)]
pub struct ActionState {
    held: HashSet<Action>,
    pressed: HashSet<Action>,
    released: HashSet<Action>,
}

impl ActionState {
    pub fn held(&self, a: Action) -> bool {
        self.held.contains(&a)
    }
    pub fn pressed(&self, a: Action) -> bool {
        self.pressed.contains(&a)
    }
    pub fn released(&self, a: Action) -> bool {
        self.released.contains(&a)
    }
    pub fn axis(&self) -> Vec2 {
        // Add logic for gamepad at some point
        let x = self.held(Action::Right) as u32 as f32 - self.held(Action::Left) as u32 as f32;
        let y = self.held(Action::Down) as u32 as f32 - self.held(Action::Up) as u32 as f32;
        vec2(x, y)
    }
}
