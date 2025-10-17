use crate::prelude::*;

use super::MapArchitect;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            theme: super::themes::DungeonTheme::new(),
            rooms: Vec::new(),
            entity_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Floor);

        // place player in the center
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        // place the amulet in the furthest point
        mb.amulet_start = mb.find_most_distant();

        for _ in 0..50 {
            mb.entity_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ));
        }

        mb
    }
}
