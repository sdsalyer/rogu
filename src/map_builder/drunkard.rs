use crate::prelude::*;

use super::MapArchitect;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;
const STAGGER_DISTANCE: usize = 400;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            theme: super::themes::DungeonTheme::new(),
            rooms: Vec::new(),
            entity_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        // Fill with walls
        mb.fill(TileType::Wall);

        // first miner starts digging
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);

        // additional miners dig until % floor tiles reached
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            self.drunkard(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );

            // generate a dijkstra map
            let dijkstra_map = mb.map.generate_dijkstra_map(&center);

            // use the map to change any unreachable tiles to walls
            // TODO: Not sure the reasoning for using 2000 (tiles from center?)
            const UNREACHABLE: &f32 = &2000.0;
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist > UNREACHABLE)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }

        mb.entity_spawns = mb.spawn_enemies(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            // Place a miner on a floor tile
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            // stagger...
            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }

            // stop if we run off the map
            if !map.in_bounds(drunkard_pos) {
                break;
            }

            // accumulate our mined distance and stop when done
            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
