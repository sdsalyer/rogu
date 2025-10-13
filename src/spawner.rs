pub use crate::prelude::*;

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

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    const MONSTER_FG: (u8, u8, u8) = WHITE;
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
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

/// Create a player entity at the given position
pub fn spawn_player(ecs: &mut World, pos: Point) {
    const PLAYER_HP: i32 = 10;
    const PLAYER_GLYPH: char = '@';
    const PLAYER_FG: (u8, u8, u8) = WHITE;
    const PLAYER_BG: (u8, u8, u8) = BLACK;
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(PLAYER_FG, PLAYER_BG),
            glyph: to_cp437(PLAYER_GLYPH),
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
