mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;

use crate::prelude::*;
use automata::CellularAutomataArchitect;
use drunkard::DrunkardsWalkArchitect;
use empty::EmptyArchitect;
use rooms::RoomsArchitect;
use prefab::*;

pub trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    /// A *copy* of the context's map for working on
    pub map: Map,

    /// Each room is a bracket-lib Rect
    pub rooms: Vec<Rect>,

    /// Enemy spawn points
    pub enemy_spawns: Vec<Point>,

    /// Player spawn point
    pub player_start: Point,

    /// Amulet spawn point
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        // let mut architect = EmptyArchitect {};
        // let mut architect = RoomsArchitect {};
        // let mut architect = CellularAutomataArchitect {};
        // let mut architect = DrunkardsWalkArchitect {};
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            2 => Box::new(CellularAutomataArchitect {}),
            // TODO: This would never be reached, right?
            _ => Box::new(EmptyArchitect {}),
        };

        let mut mb = architect.new(rng);
        try_apply_fortress(&mut mb, rng);
        mb
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// create paths between rooms
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        // TODO: Can this be done without clone() ?
        let mut rooms = self.rooms.clone();

        // sort rooms by their center point so we're more likely to
        // connect adjacent rooms
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // skip the first room and connect subsequent rooms back to the previous
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let curr = room.center();

            // randomly tunnel by rows or cols first
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, curr.x, prev.y);
                self.apply_vertical_tunnel(prev.y, curr.y, curr.x);
            } else {
                self.apply_vertical_tunnel(prev.y, curr.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, curr.x, curr.y);
            }
        }
    }

    /// Creates a number of randomly sized rooms that don't overlap
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        const NUM_ROOMS: usize = 20;
        while self.rooms.len() < NUM_ROOMS {
            // New room with random x, y, width, height
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // Check if this new room overlaps any existing rooms
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // Add the new room if no overlap
            if !overlap {
                // iterate each point of the rect
                room.for_each(|point| {
                    if point.x > 0
                        && point.x < SCREEN_WIDTH
                        && point.y > 0
                        && point.y < SCREEN_HEIGHT
                    {
                        let idx = map_idx(point.x, point.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        } // end while rooms
    }

    /// Make every tile the same as the given type
    fn fill(&mut self, tile: TileType) {
        // note that `t` is a reference, so it must be deref for assignment
        // (this could just as easily been a regular for loop)
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        const UNREACHABLE: &f32 = &f32::MAX;
        let dijkstra_map = &self.map.generate_dijkstra_map(&self.player_start);
        self.map.index_to_point2d(
            // find the index of the furthest room
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    fn spawn_enemies(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_ENEMIES: usize = 50;

        // Find floor tiles > 10 units away from `start`
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        // Randomly select a tile as a spawn point
        let mut spawns = Vec::new();
        for _ in 0..NUM_ENEMIES {
            let target_idx = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_idx].clone());
            // remove spawn candidate from future iterations
            spawnable_tiles.remove(target_idx);
        }

        spawns
    }
}
