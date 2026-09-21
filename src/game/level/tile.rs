use crate::vars::TILE_SIZE;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TileType {
    #[default]
    Empty,
    Stone,
    Debug,
}

#[derive(Default)]
pub struct TileMap {
    width: usize,
    height: usize,
    tiles: Vec<TileType>,
}

impl TileMap {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileType::Empty; width * height],
        }
    }

    // Note that tx and ty are measured in TILE_SIZE not in PIXEL or subpixels
    pub fn get(&self, tx: i32, ty: i32) -> TileType {
        if tx < 0 || ty < 0 || tx >= self.width as i32 || tx >= self.height as i32 {
            return TileType::Empty;
        }
        self.tiles[ty as usize * self.width + tx as usize]
    }

    // Kind is used instead of type since type is already defined as something in rust
    pub fn set(&mut self, tx: usize, ty: usize, kind: TileType) {
        if tx < self.width && ty < self.height {
            self.tiles[ty * self.width + tx] = kind;
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, kind: TileType) {
        for ty in y..y + height {
            for tx in x..x + width {
                self.set(tx, ty, kind)
            }
        }
    }

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
}
