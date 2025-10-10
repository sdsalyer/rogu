pub use crate::prelude::*;

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    const MONSTER_FG: (u8, u8, u8) = ORANGE;
    const MONSTER_BG: (u8, u8, u8) = BLACK;
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(MONSTER_FG, MONSTER_BG),
            // TODO: More on mosnsters later
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'), // Ettin
                1 => to_cp437('O'), // Ogre
                2 => to_cp437('o'), // orc
                3 => to_cp437('g'), // goblin
                _ => to_cp437('M'), // any ol monster
            },
        },
        MovingRandomly,
    ));
}

/// Create a player entity at the given position
pub fn spawn_player(ecs: &mut World, pos: Point) {
    const PLAYER_HP: i32 = 20;
    const PLAYER_FG: (u8, u8, u8) = PURPLE;
    const PLAYER_BG: (u8, u8, u8) = BLACK;
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(PLAYER_FG, PLAYER_BG),
            glyph: to_cp437('@'),
        },
        Health {
            current: PLAYER_HP,
            max: PLAYER_HP,
        },
    ));
}
