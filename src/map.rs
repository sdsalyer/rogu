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

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let dest = loc + delta;
        if self.in_bounds(dest) {
            if self.can_enter_tile(dest) {
                let idx = self.point2d_to_index(dest);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// bracket-lib pathfinding
impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        // Move cost could change on terrain or moving diagonal (i.e. 1.4)
        const MOVE_COST: f32 = 1.0;

        let mut exits = SmallVec::new();
        let loc = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(loc, Point::new(-1, 0)) {
            exits.push((idx, MOVE_COST))
        }

        if let Some(idx) = self.valid_exit(loc, Point::new(1, 0)) {
            exits.push((idx, MOVE_COST))
        }

        if let Some(idx) = self.valid_exit(loc, Point::new(0, 1)) {
            exits.push((idx, MOVE_COST))
        }

        if let Some(idx) = self.valid_exit(loc, Point::new(0, -1)) {
            exits.push((idx, MOVE_COST))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

/// Bracket utils for map to coordination translations
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
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
