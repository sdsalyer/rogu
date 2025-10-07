use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    /// A *copy* of the context's map for working on
    pub map: Map,

    /// Each room is a bracket-lib Rect
    pub rooms: Vec<Rect>,

    /// Player spawn point
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        // mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();

        mb
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
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
        // note that `t` is a reference, so it must be deref for
        // assignment
        // (this could just as easily been a regular for loop)
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}
