// Frame rate constants
pub const MAX_ACCUMULATOR: f32 = 0.25;
pub const FRAME: f32 = 1.0 / 60.0;

pub const PAUSE_FREEZE_TIME: f32 = 6.0 * FRAME;

// The maximum number of subpixels an entity can move in a single step. To avoid tunneling
pub const MAX_STEPS: f32 = 3.0 * PIXEL;

// Constants to determine 'pixel' size and Tile Size
pub const PIXEL: f32 = 6.0;
pub const TILE_SIZE: f32 = PIXEL * 8.0;
pub const DEBUG_FONT_SIZE: u16 = 20;

// Constants for the player
pub const P_HEIGHT: f32 = 1.5 * TILE_SIZE;
pub const P_ROLL_HEIGHT: f32 = P_HEIGHT / 3.0;
pub const P_ROLL_OFFSET: f32 = P_HEIGHT - P_ROLL_HEIGHT;
pub const P_WIDTH: f32 = TILE_SIZE;

pub const P_WALK_ACC: f32 = 600.0;
pub const P_WALK_VEL: f32 = 90.0;

pub const P_GRAVITY: f32 = 600.0;
pub const P_MAX_FALL: f32 = 120.0;
pub const P_MAX_FAST_FALL: f32 = 240.0;

pub const P_JUMP_ACC: f32 = 520.0;
pub const P_JUMP_INIT_VEL: f32 = 100.0;
pub const P_JUMP_GRAVITY_MULTIPER: f32 = 0.5;
pub const P_JUMP_GRAVITY_MULTIPLIER_LIMIT: f32 = 40.0;

pub const P_AIR_RESISTANCE: f32 = 100.0;
pub const P_FRICTION: f32 = 500.0;

pub const P_COYOTE_TIME: f32 = 5.0 * FRAME;
pub const P_BUFFER_TIME: f32 = 5.0 * FRAME;
pub const P_MAX_JUMP_TIME: f32 = 16.0 * FRAME;
pub const P_MAX_CEILING_TIME: f32 = 2.0 * FRAME;
