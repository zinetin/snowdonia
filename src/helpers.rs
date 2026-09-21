pub fn approach_zero(vel: f32, amount: f32) -> f32 {
    vel.signum() * (vel.abs() - amount).max(0.0)
}
