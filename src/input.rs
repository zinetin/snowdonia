use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};

// Create an enum so that key input, mouse input and gamepad input can all be interacted by the
// same functions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Input {
    Key(KeyCode),
    Mse(MouseButton),
}

// Make key and mouse and (eventually) gamepad input all be interacted by the same function
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

// Create the Actions that the inputs will match to
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

// Struct for the bindings hashmap
pub struct InputMap {
    input_map: HashMap<Input, Action>,
}

impl InputMap {
    // Create the default bindings
    pub fn defaults() -> Self {
        let mut input_map: HashMap<Input, Action> = HashMap::new();
        input_map.insert(Input::Key(KeyCode::E), Action::Up);
        input_map.insert(Input::Key(KeyCode::S), Action::Left);
        input_map.insert(Input::Key(KeyCode::D), Action::Down);
        input_map.insert(Input::Key(KeyCode::F), Action::Right);
        input_map.insert(Input::Key(KeyCode::Space), Action::Jump);
        input_map.insert(Input::Key(KeyCode::J), Action::Grab);
        input_map.insert(Input::Key(KeyCode::K), Action::Dash);
        input_map.insert(Input::Key(KeyCode::L), Action::Roll);
        input_map.insert(Input::Key(KeyCode::GraveAccent), Action::Debug);
        input_map.insert(Input::Key(KeyCode::Escape), Action::Pause);

        Self { input_map }
    }

    // Poll the input to see the active actions
    pub fn poll(&self) -> ActionState {
        let mut state = ActionState::default();
        for (input, action) in &self.input_map {
            if input.is_down() {
                state.held.insert(*action);
            }
            if input.is_pressed() {
                state.pressed.insert(*action);
            }
            if input.is_released() {
                state.released.insert(*action);
            }
        }
        state
    }
}

// Struct so that you don't have to poll the input for every single entity reading inputs
#[derive(Default)]
pub struct ActionState {
    held: HashSet<Action>,
    pressed: HashSet<Action>,
    released: HashSet<Action>,
}

// Implimentation of checking if a key is help, pressed or released
// // Implimentation of checking if a key is help, pressed or released
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
