use crate::game::level::tile::TileMap;
use crate::game::Entity;
use crate::game::GameState;
use crate::helpers::approach_zero;
use crate::input::{Action, ActionState};
use crate::vars::P_JUMP_GRAVITY_MULTIPER;
use crate::vars::{
    DEBUG_FONT_SIZE, MAX_STEPS, PIXEL, P_AIR_RESISTANCE, P_BUFFER_TIME, P_COYOTE_TIME, P_FRICTION,
    P_GRAVITY, P_HEIGHT, P_JUMP_ACC, P_JUMP_GRAVITY_MULTIPLIER_LIMIT, P_JUMP_INIT_VEL, P_MAX_FALL,
    P_MAX_FAST_FALL, P_MAX_JUMP_TIME, P_WALK_ACC, P_WALK_VEL, P_WIDTH,
};
use macroquad::prelude::*;

#[derive(Default, Clone, Debug)]
pub enum WallSide {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub r: Rect,

    pub x_vel: f32,
    pub y_vel: f32,
    pub gravity: f32,

    pub grounded: bool,
    pub jumping: bool,
    pub rolling: bool,
    pub wall_side: WallSide,
    pub grabbing: bool,

    pub coyote_time: f32,

    pub jump_buffer: f32,
    pub jump_time_left: f32,
}

impl Entity for Player {
    fn update(&mut self, dt: f32, inputs: &ActionState, tilemap: &TileMap) {
        // Walk velocity calculation
        self.x_vel = Self::add_walk_vel(self.x_vel, dt, inputs);

        self.rolling = if inputs.held(Action::Roll) {
            true
        } else {
            false
        };

        if inputs.pressed(Action::Roll) {
            self.r.y += P_HEIGHT / 3.0 * 2.0;
        }
        if inputs.released(Action::Roll) {
            self.r.y -= P_HEIGHT / 3.0 * 2.0;
        }

        self.r.h = if self.rolling {
            P_HEIGHT / 3.0
        } else {
            P_HEIGHT
        };

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

        let drag_x = if self.grounded {
            P_FRICTION * dt
        } else {
            P_AIR_RESISTANCE * dt
        } * if self.rolling { 0.5 } else { 1.0 }
            * if inputs.axis().x != 0.0 { 0.5 } else { 1.0 };

        let drag_y = if !self.grounded {
            P_AIR_RESISTANCE * dt
        } else {
            0.0
        };

        self.jump(dt, inputs);

        self.x_vel = approach_zero(self.x_vel, drag_x);
        self.y_vel = approach_zero(self.y_vel, drag_y);

        // All modifiers to x and y vel go above this

        //
        //
        //

        // change in x or y
        let dx = self.x_vel * dt;
        let dy = self.y_vel * dt;

        // Seperate the velocity into small steps to calculate to avoid tunneling
        let steps = (dx.abs().max(dy.abs()) / MAX_STEPS).ceil().max(1.0) as u32;

        // Change in change in x or y
        let mut ddx = dx / steps as f32;
        let mut ddy = dy / steps as f32;

        self.grounded = false;

        for _ in 0..steps {
            if ddx != 0.0 && tilemap.move_x(&mut self.r, ddx) {
                self.x_vel = 0.0;
                ddx = 0.0; // Stops the attempting to continue to move in the x dir
            }

            if ddy != 0.0 && tilemap.move_y(&mut self.r, ddy) {
                if ddy > 0.0 {
                    self.grounded = true;
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
}

impl Player {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    fn draw_debug_box(&self) {
        // {:#.1?} = pretty-print, floats to 1 decimal place
        let text = format!("{:#.1?}", self);
        let lines: Vec<&str> = text.lines().collect();

        let font_size = DEBUG_FONT_SIZE as f32;
        let line_h = font_size * 1.2;
        let pad = 8.0;

        // Width of the widest line, so the box fits the text
        let text_w = lines
            .iter()
            .map(|l| measure_text(l, None, DEBUG_FONT_SIZE, 1.0).width)
            .fold(0.0, f32::max);

        let box_w = text_w + pad * 2.0;
        let box_h = lines.len() as f32 * line_h + pad * 2.0;
        let x = screen_width() - box_w - pad;
        let y = pad;

        draw_rectangle(x, y, box_w, box_h, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_rectangle_lines(x, y, box_w, box_h, 2.0, WHITE);

        for (i, line) in lines.iter().enumerate() {
            // draw_text's y is the text baseline, so add font_size to sit inside the box
            draw_text(
                line,
                x + pad,
                y + pad + i as f32 * line_h + font_size,
                font_size,
                WHITE,
            );
        }
    }

    pub fn add_walk_vel(curr_x_vel: f32, dt: f32, inputs: &ActionState) -> f32 {
        // Calculate dx due to walk input
        let walk_dx = inputs.axis().x * P_WALK_ACC * dt;
        // Check to see which is larger, current velocity, or maximum walk velocity.
        let limit = P_WALK_VEL.max(curr_x_vel.abs());
        // Calculate the new velocity
        let new_vel = curr_x_vel + walk_dx;
        // Set self.x_vel to the new velocity if the new velocity is less than whichever is larger
        // of current velocity or maximum walk velocity, or set it to the limit, if new vel is
        // larger than the limit
        new_vel.clamp(-limit, limit)
    }

    pub fn jump(&mut self, dt: f32, inputs: &ActionState) {
        if self.jump_buffer > 0.0 {
            self.jump_buffer -= dt;
        } else if self.jump_buffer < 0.0 {
            self.jump_buffer = 0.0;
        }

        if inputs.pressed(Action::Jump) {
            self.jump_buffer = P_BUFFER_TIME;
        }

        if self.jump_buffer > 0.0 && self.coyote_time > 0.0 {
            self.jumping = true;
            self.jump_time_left = P_MAX_JUMP_TIME;
            self.y_vel = -1.0 * P_JUMP_INIT_VEL;
            self.jump_buffer = 0.0;
            self.coyote_time = 0.0;
            self.x_vel += if self.x_vel != 0.0 {
                self.x_vel.signum() * 40.0
            } else {
                0.0
            }
        }

        if self.jumping && inputs.held(Action::Jump) && self.jump_time_left > 0.0 {
            self.y_vel -= P_JUMP_ACC * dt;
            self.jump_time_left -= dt;
        } else {
            self.jumping = false;
            self.jump_time_left = 0.0;
        }
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
            jumping: false,
            rolling: false,
            wall_side: WallSide::None,
            grabbing: false,
            coyote_time: 0.0,
            jump_buffer: 0.0,
            jump_time_left: 0.0,
        }
    }
}
