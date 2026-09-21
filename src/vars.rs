// Frame rate constants
pub const MAX_ACCUMULATOR: f32 = 0.25;
pub const FRAME: f32 = 1.0 / 60.0;

// Constants to determine 'pixel' size and Tile Size
pub const PIXEL: f32 = 4.0;
pub const TILE_SIZE: f32 = PIXEL * 8.0;

// Constants for the player
pub const P_HEIGHT: f32 = 3.0 * TILE_SIZE;
pub const P_WIDTH: f32 = 2.0 * TILE_SIZE;
pub const P_WALK_ACC: f32 = 20.0;
pub const P_WALK_VEL: f32 = 100.0;
pub const P_GRAVITY: f32 = 50.0;
pub const P_MAX_FALL: f32 = 100.0;
pub const P_MAX_FAST_FALL: f32 = 200.0;
