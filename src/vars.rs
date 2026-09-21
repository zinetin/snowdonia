// Frame rate constants
pub const MAX_ACCUMULATOR: f32 = 0.25;
pub const FRAME: f32 = 1.0 / 60.0;

// The maximum number of subpixels an entity can move in a single step. To avoid tunneling
pub const MAX_STEPS: f32 = 15.0;

// Constants to determine 'pixel' size and Tile Size
pub const PIXEL: f32 = 4.0;
pub const TILE_SIZE: f32 = PIXEL * 8.0;
pub const DEBUG_FONT_SIZE: u16 = 20;

// Constants for the player
pub const P_HEIGHT: f32 = 3.0 * TILE_SIZE;
pub const P_WIDTH: f32 = 2.0 * TILE_SIZE;

pub const P_WALK_ACC: f32 = 4000.0;
pub const P_WALK_VEL: f32 = 500.0;

pub const P_GRAVITY: f32 = 4000.0;
pub const P_MAX_FALL: f32 = 800.0;
pub const P_MAX_FAST_FALL: f32 = 1600.0;

pub const P_JUMP_ACC: f32 = 3150.0;
pub const P_JUMP_INIT_VEL: f32 = 600.0;
pub const P_JUMP_GRAVITY_MULTIPER: f32 = 0.5;
pub const P_JUMP_GRAVITY_MULTIPLIER_LIMIT: f32 = 150.0;

pub const P_AIR_RESISTANCE: f32 = 500.0;
pub const P_FRICTION: f32 = 2500.0;

pub const P_COYOTE_TIME: f32 = 5.0 * FRAME;
pub const P_BUFFER_TIME: f32 = 5.0 * FRAME;
pub const P_MAX_JUMP_TIME: f32 = 15.0 * FRAME;
