use crate::vars::TILE_SIZE;
use macroquad::prelude::*;

// The different tile types
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TileType {
    #[default]
    Empty,
    Stone,
    Debug,
}

// The struct that determines a tile map
#[derive(Default)]
pub struct TileMap {
    width: usize,  // Width of level / screen
    height: usize, // height
    tiles: Vec<TileType>, // Vec of tiles, 2d array is not needed, since we have the width of the
                   // level
}

// Tilemap impls
impl TileMap {
    // Initializes an empty tilemap
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileType::Empty; width * height],
        }
    }

    // This allows you to get the data of a specific tile
    // Note that tx and ty are measured in TILE_SIZE not in PIXEL or subpixels
    pub fn get(&self, tx: i32, ty: i32) -> TileType {
        if tx < 0 || ty < 0 || tx >= self.width as i32 || tx >= self.height as i32 {
            return TileType::Empty;
        }
        self.tiles[ty as usize * self.width + tx as usize]
    }

    // This allows you to set a specific tile to a different tiletype
    // Kind is used instead of type since type is already defined as something in rust
    pub fn set(&mut self, tx: usize, ty: usize, kind: TileType) {
        if tx < self.width && ty < self.height {
            self.tiles[ty * self.width + tx] = kind;
        }
    }

    // Set a rectangle, this exists to draw the debug map
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, kind: TileType) {
        for ty in y..y + height {
            for tx in x..x + width {
                self.set(tx, ty, kind)
            }
        }
    }

    // Draw the tiles to the screen
    pub fn draw(&self) {
        for ty in 0..self.height {
            for tx in 0..self.width {
                let kind = self.tiles[ty * self.width + tx];

                match kind {
                    TileType::Empty => continue,
                    TileType::Debug => draw_rectangle(
                        tx as f32 * TILE_SIZE,
                        ty as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        WHITE,
                    ),
                    _ => println!("Not yet add a texture for this tile"),
                }
            }
        }
    }

    // Very simple helper that just checks if the tile is not empty
    // Exists in case I make other tiles that are empty, but have a different texture / properties
    pub fn is_solid(&self, tx: i32, ty: i32) -> bool {
        self.get(tx, ty) != TileType::Empty
    }

    // Calculates the range of tile coords that the player's hitbox lies in
    pub fn tile_range(&self, r: Rect) -> (i32, i32, i32, i32) {
        // (x0, y0, x1, y1)
        let eps = 0.001;
        (
            (r.x / TILE_SIZE).floor() as i32,
            (r.y / TILE_SIZE).floor() as i32,
            ((r.x + r.w - eps) / TILE_SIZE).floor() as i32,
            ((r.y + r.h - eps) / TILE_SIZE).floor() as i32,
        )
    }

    pub fn move_x(&self, r: &mut Rect, dx: f32) -> bool {
        r.x += dx;
        let (x0, y0, x1, y1) = self.tile_range(*r);
        for ty in y0..=y1 {
            for tx in x0..=x1 {
                if self.is_solid(tx, ty) {
                    if dx > 0.0 {
                        r.x = tx as f32 * TILE_SIZE - r.w
                    } else if dx < 0.0 {
                        r.x = (tx + 1) as f32 * TILE_SIZE;
                    }
                    return true;
                }
            }
        }
        false
    }

    pub fn move_y(&self, r: &mut Rect, dy: f32) -> bool {
        r.y += dy;
        let (x0, y0, x1, y1) = self.tile_range(*r);
        for ty in y0..=y1 {
            for tx in x0..=x1 {
                if self.is_solid(tx, ty) {
                    if dy > 0.0 {
                        r.y = ty as f32 * TILE_SIZE - r.h
                    } else if dy < 0.0 {
                        r.y = (ty + 1) as f32 * TILE_SIZE;
                    }
                    return true;
                }
            }
        }
        false
    }
}
