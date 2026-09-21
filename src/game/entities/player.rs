use crate::game::Entity;
use crate::input::ActionState;
use crate::vars::{P_GRAVITY, P_HEIGHT, P_WIDTH};
use macroquad::prelude::*;

pub mod walking;

#[derive(Default, Clone)]
pub enum WallSide {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Clone)]
pub struct Player {
    pub height: f32,
    pub width: f32,

    pub x: f32,
    pub y: f32,
    pub x_vel: f32,
    pub y_vel: f32,

    pub grounded: bool,
    pub jumping: bool,
    pub rolling: bool,
    pub wall_side: WallSide,
    pub grabbing: bool,

    pub coyote_time: f32,

    pub jump_buffer: f32,
}

impl Entity for Player {
    fn update(&mut self, dt: f32, inputs: &ActionState) {
        self.x_vel = walking::add_walk_vel(self.x_vel, dt, inputs);

        if !self.grounded {
            self.y_vel += P_GRAVITY * dt;
        }

        self.x += self.x_vel;
        self.y += self.y_vel;
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
    }
}

impl Player {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            height: P_HEIGHT,
            width: P_WIDTH,
            x: 0.0,
            y: 0.0,
            x_vel: 0.0,
            y_vel: 0.0,
            grounded: false,
            jumping: false,
            rolling: false,
            wall_side: WallSide::None,
            grabbing: false,
            coyote_time: 0.0,
            jump_buffer: 0.0,
        }
    }
}
