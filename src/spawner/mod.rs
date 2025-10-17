mod template;

use crate::prelude::*;
use template::Templates;

/// Create the Amulet of Yala at the given point
pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    const AMULET_GLYPH: char = '|';
    const AMULET_FG: (u8, u8, u8) = WHITE;
    const AMULET_BG: (u8, u8, u8) = BLACK;
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(AMULET_FG, AMULET_BG),
            glyph: to_cp437(AMULET_GLYPH),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

/// Create a game level
pub fn spawn_level(
    ecs: &mut World,
    resources: &mut Resources,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, resources, rng, level, spawn_points);
}

/// Create a player entity at the given position
pub fn spawn_player(ecs: &mut World, pos: Point) {
    const PLAYER_HP: i32 = 10;
    const PLAYER_DMG: i32 = 1;
    const PLAYER_GLYPH: char = '@';
    const PLAYER_FG: (u8, u8, u8) = WHITE;
    const PLAYER_BG: (u8, u8, u8) = BLACK;
    const PLAYER_FOV: i32 = 8;
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(PLAYER_FG, PLAYER_BG),
            glyph: to_cp437(PLAYER_GLYPH),
        },
        Health {
            current: PLAYER_HP,
            max: PLAYER_HP,
        },
        Damage(PLAYER_DMG),
        FieldOfView::new(PLAYER_FOV),
    ));
}
