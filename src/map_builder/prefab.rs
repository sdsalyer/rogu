use crate::prelude::*;

pub struct Vault {
    pub template: String,
    pub width: i32,
    pub height: i32,
}

impl Vault {
    pub fn new(template: &str) -> Self {
        let width = template.lines().max().unwrap().len();
        let height = template.lines().filter(|x| x.trim().len() != 0).count();
        Self {
            template: template.to_string(),
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
        }
    }

    pub fn try_apply(&self, mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        const MAX_ATTEMPTS: usize = 10;
        const UNREACHABLE: f32 = 2000.0;
        const NEAREST: f32 = 20.0;

        let mut placement = None;
        let dijkstra_map = mb.map.generate_dijkstra_map(&mb.player_start);

        let mut attempts = 0;
        while placement.is_none() && attempts < MAX_ATTEMPTS {
            let dimensions = Rect::with_size(
                rng.range(0, SCREEN_WIDTH - self.width),
                rng.range(0, SCREEN_HEIGHT - self.height),
                self.width,
                self.height,
            );

            let mut can_place = false;
            dimensions.for_each(|pt| {
                let idx = mb.map.point2d_to_index(pt);
                // TODO: BUG - index out of bounds: the len is 4000 but the index is 4032
                let dist = dijkstra_map.map[idx];
                if dist < UNREACHABLE && dist > NEAREST && mb.amulet_start != pt {
                    can_place = true;
                }
            });

            if can_place {
                placement = Some(Point::new(dimensions.x1, dimensions.y1));
                let points = dimensions.point_set();
                mb.entity_spawns.retain(|pt| !points.contains(pt));
            }

            attempts += 1;
        }

        if let Some(placement) = placement {
            let template: Vec<char> = self
                .template
                .chars()
                .filter(|a| *a != '\r' && *a != '\n')
                .collect();

            let mut i = 0;
            for ty in placement.y..placement.y + self.height {
                for tx in placement.x..placement.x + self.width {
                    let idx = map_idx(tx, ty);
                    let c = template[i];
                    match c {
                        'M' => {
                            mb.map.tiles[idx] = TileType::Floor;
                            mb.entity_spawns.push(Point::new(tx, ty));
                        }
                        '-' => mb.map.tiles[idx] = TileType::Floor,
                        '#' => mb.map.tiles[idx] = TileType::Wall,
                        _ => eprintln!("Unknown character in vault: {}", c),
                    }
                    i += 1;
                }
            }
        }
    }
}

pub fn try_apply_fortress(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    const FORTRESS: &str = "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
";

    let vault = Vault::new(FORTRESS);

    vault.try_apply(mb, rng);
}

/* TODO: for fully prefabricated levels
use super::MapArchitect;

pub struct VaultArchitect {}

impl MapArchitect for VaultArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            enemy_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Floor);

        // place player in the center
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        // place the amulet in the furthest point
        mb.amulet_start = mb.find_most_distant();

        for _ in 0..50 {
            mb.enemy_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ));
        }

        mb
    }
}
*/
