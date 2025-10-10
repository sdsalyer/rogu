pub use crate::prelude::*;

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    const MONSTER_FG: (u8, u8, u8) = ORANGE;
    const MONSTER_BG: (u8, u8, u8) = BLACK;

    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(MONSTER_FG, MONSTER_BG),
            glyph,
        },
        MovingRandomly,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
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

// (hp, name, glyph)
fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

// (hp, name, glyph)
fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
