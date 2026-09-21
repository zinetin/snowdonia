use crate::input::ActionState;
use crate::vars::{P_WALK_ACC, P_WALK_VEL};

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
