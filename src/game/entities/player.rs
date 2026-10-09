use crate::game::level::tile::TileMap;
use crate::game::Entity;
use crate::game::GameState;
use crate::helpers::approach_zero;
use crate::input::{Action, ActionState};
use crate::vars::{
    DEBUG_FONT_SIZE, MAX_STEPS, PIXEL, P_AIR_RESISTANCE, P_BUFFER_TIME, P_COYOTE_TIME, P_FRICTION,
    P_GRAVITY, P_HEIGHT, P_JUMP_GRAVITY_MULTIPER, P_JUMP_GRAVITY_MULTIPLIER_LIMIT,
    P_MAX_CEILING_TIME, P_MAX_FALL, P_MAX_FAST_FALL, P_ROLL_HEIGHT, P_ROLL_OFFSET, P_WALK_ACC,
    P_WALK_VEL, P_WIDTH,
};
use macroquad::prelude::*;

pub mod draw_debug_box;
pub mod entity_impl;
pub mod jump;
pub mod walk;

#[derive(Default, Clone, Debug)]
pub enum WallSide {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Clone, Debug)]
pub struct Player {
    // Contains the players x and y coords, as well as its width and height
    pub r: Rect,

    // players x_vel, y_vel and gravity
    pub x_vel: f32,
    pub y_vel: f32,
    pub gravity: f32,

    // Whether the player is touching a ground or ceiling
    pub grounded: bool,
    pub ceilinged: bool,

    // Whether the player is jumping
    pub jumping: bool,

    // Whether the player is rolling, and whether it was rolling on the last physics frame
    pub was_rolling: bool,
    pub rolling: bool,

    // Whether there is a wall on either side of the player, and if so, where it is
    pub wall_side: WallSide,
    pub grabbing: bool,

    // Time to jump after falling off a platform.
    pub coyote_time: f32,

    // Jump stuff
    pub jump_buffer: f32,
    pub jump_time_left: f32,
    pub ceiling_time: f32,
}

impl Entity for Player {
    fn update(&mut self, dt: f32, inputs: &ActionState, tilemap: &TileMap) {
        // Checks if the player was rolling last frame
        self.was_rolling = self.rolling;

        // Puts the player into the roll state
        self.rolling = inputs.held(Action::Roll);

        // Moves the player appropriately when rolling and unrolling
        if self.rolling && !self.was_rolling {
            self.r.y += P_ROLL_OFFSET;
        }
        if !self.rolling && self.was_rolling {
            self.r.y -= P_ROLL_OFFSET;
        }

        // Changes the player height when rolling and unrolling
        self.r.h = if self.rolling {
            P_ROLL_HEIGHT
        } else {
            P_HEIGHT
        };

        // Decreases gravity if jumping and jump vel between acceptable limits
        if inputs.held(Action::Jump) && self.y_vel.abs() < P_JUMP_GRAVITY_MULTIPLIER_LIMIT {
            self.gravity = P_GRAVITY * P_JUMP_GRAVITY_MULTIPER;
        } else {
            self.gravity = P_GRAVITY;
        }

        // Apply Gravity
        if self.y_vel < P_MAX_FAST_FALL && inputs.held(Action::Down) {
            self.y_vel += self.gravity * dt;
        } else if self.y_vel < P_MAX_FALL {
            self.y_vel += self.gravity * dt;
        }

        // Coyote Time Calculations
        if self.grounded {
            self.coyote_time = P_COYOTE_TIME;
        } else if self.coyote_time > 0.0 {
            self.coyote_time -= dt;
        } else {
            self.coyote_time = 0.0;
        }

        // Calculate drag for x
        let drag_x = if self.grounded {
            P_FRICTION * dt
        } else {
            P_AIR_RESISTANCE * dt
        } * if self.rolling { 0.5 } else { 1.0 }
            * if inputs.axis().x != 0.0 { 0.5 } else { 1.0 };

        // Calculate drag for y
        let drag_y = if !self.grounded {
            P_AIR_RESISTANCE * dt
        } else {
            0.0
        };

        // Jump function - does all of the stuff to do with jumping
        self.jump(dt, inputs);

        // Apply drag for x and y (approach zero makes a v1 approach zero by an amount equal to v2)
        self.x_vel = approach_zero(self.x_vel, drag_x);
        self.y_vel = approach_zero(self.y_vel, drag_y);

        // If touching ceiling, increase ceiling time.
        if self.ceilinged {
            self.ceiling_time += dt;
        } else {
            self.ceiling_time = 0.0;
        }

        // If the player touches the ceiling for too long, lose the jump state
        if self.jumping && self.ceiling_time >= P_MAX_CEILING_TIME {
            self.jumping = false;
        }

        // Walk velocity calculation
        self.x_vel = Self::add_walk_vel(self.x_vel, dt, inputs);

        // All modifiers to x and y vel go above this

        //
        //
        //

        // change in x or y
        let dx = self.x_vel * dt * PIXEL;
        let dy = self.y_vel * dt * PIXEL;

        // Seperate the velocity into small steps to calculate to avoid tunneling
        // To do this, it finds whether dx or dy is larger. Then divides this by the maximum number
        // of pixels the player can move per step. Rounds up. Then makes sure this value is at
        // least 1
        let steps = (dx.abs().max(dy.abs()) / MAX_STEPS).ceil().max(1.0) as u32;

        // Calculates the change in x and y per step
        let mut ddx = dx / steps as f32;
        let mut ddy = dy / steps as f32;

        // Reset some variables
        self.grounded = false;
        self.ceilinged = false;

        // Apply x and y velocity
        for _ in 0..steps {
            // Call tilemap.move_x, which tries to move the rectangle by the change in x. If it
            // fails, it returns false, causing the players x_vel to be 0, and setting ddx to be 0
            if ddx != 0.0 && !tilemap.move_x(&mut self.r, ddx) {
                self.x_vel = 0.0;
                ddx = 0.0; // Stops the attempting to continue to move in the x dir
            }

            if ddy != 0.0 && !tilemap.move_y(&mut self.r, ddy) {
                if ddy > 0.0 {
                    self.grounded = true;
                }
                if ddy < 0.0 {
                    self.ceilinged = true;
                }
                self.y_vel = 0.0;
                ddy = 0.0;
            }
        }
    }

    fn draw(&self, game_state: &GameState) {
        if game_state.debug {
            self.draw_debug_box()
        }
        let x_pixel = (self.r.x / PIXEL).floor() * PIXEL;
        let y_pixel = (self.r.y / PIXEL).floor() * PIXEL;
        draw_rectangle(x_pixel, y_pixel, self.r.w, self.r.h, WHITE);
    }

    fn update_buffers(&mut self, dt: f32, inputs: &ActionState) {
        // Checks if the jump buffer is larger than 0, and if it is, decreases the jump buffer.
        // Otherwise it just sets the jump buffer to 0
        if self.jump_buffer > 0.0 {
            self.jump_buffer -= dt;
        } else if self.jump_buffer < 0.0 {
            self.jump_buffer = 0.0;
        }

        // If the player presses the jump input, set the jump buffer.
        if inputs.pressed(Action::Jump) {
            self.jump_buffer = P_BUFFER_TIME;
        }
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
            r: Rect::new(0.0, 0.0, P_WIDTH, P_HEIGHT),

            x_vel: 0.0,
            y_vel: 0.0,
            gravity: P_GRAVITY,

            grounded: false,
            ceilinged: false,

            jumping: false,
            was_rolling: false,
            rolling: false,

            wall_side: WallSide::None,
            grabbing: false,

            coyote_time: 0.0,

            jump_buffer: 0.0,
            jump_time_left: 0.0,
            ceiling_time: 0.0,
        }
    }
}
