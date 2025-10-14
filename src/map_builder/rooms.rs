use crate::prelude::*;

use super::MapArchitect;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            enemy_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);

        // place player in the center of the first room
        mb.player_start = mb.rooms[0].center();

        // place the amulet in the furthest point
        mb.amulet_start = mb.find_most_distant();

        // 1 monster per room (except the first)
        for r in mb.rooms.iter().skip(1) {
            mb.enemy_spawns.push(r.center());
        }

        mb
    }
}
