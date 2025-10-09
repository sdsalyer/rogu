use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
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

    /* Now in systems/map_render.rs...
    /// render the map in its current state
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        // y-first is faster since we are using row-first striding
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                (50, 50, 50),
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                (30, 5, 25),
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                } // end if in bounds
            } // end x loop
        } // end y loop
    }
    */

    /// check whether point is walkable
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /// check whether point is on the screen
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// get map tile index or None for given point
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(map_idx(point.x, point.y))
        } else {
            None
        }
    }
}

/// get index of map tile given x, y coords (aka _striding_)
/// this is Y-first (or row-first) encoding
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

/*
/// get (x, y) tuple of map tile given its index
/// i.e. the inverse of `map_idx()`
pub fn map_coords(idx: i32) -> (i32, i32) {
    (idx % SCREEN_WIDTH, idx / SCREEN_WIDTH)
}
*/
