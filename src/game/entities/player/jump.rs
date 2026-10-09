use crate::game::entities::player::Player;
use crate::input::{Action, ActionState};
use crate::vars::{P_BUFFER_TIME, P_JUMP_ACC, P_JUMP_INIT_VEL, P_MAX_JUMP_TIME};

impl Player {
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
            self.x_vel *= 1.2
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
