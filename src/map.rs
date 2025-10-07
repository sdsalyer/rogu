use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    /// The map tiles are stored in a single list, in x-order
    pub tiles: Vec<TileType>,
}

impl Map {
    /// Create a new map of all Floor tiles
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// render the map in its current state
    pub fn render(&self, ctx: &mut BTerm) {
        // y-first is faster since we are using row-first striding
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            } // end x loop
        } // end y loop
    }
}

/// get index of map tile given x, y coords (aka _striding_)
/// this is Y-first (or row-first) encoding
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

/// get (x, y) tuple of map tile given its index
/// i.e. the inverse of `map_idx()`
pub fn map_coords(idx: i32) -> (i32, i32) {
    (idx % SCREEN_WIDTH, idx / SCREEN_WIDTH)
}
