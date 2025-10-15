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
    const ENEMY_FG: (u8, u8, u8) = WHITE;
    const ENEMY_BG: (u8, u8, u8) = BLACK;
    const ENEMY_FOV: i32 = 6;

    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(ENEMY_FG, ENEMY_BG),
            glyph,
        },
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(ENEMY_FOV),
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_enemy(ecs, rng, pos),
    }
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    const ITEM_GLYPH: char = '!';
    const ITEM_FG: (u8, u8, u8) = WHITE;
    const ITEM_BG: (u8, u8, u8) = BLACK;
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(ITEM_FG, ITEM_BG),
            glyph: to_cp437(ITEM_GLYPH),
        },
        Name("Healing Potion".to_string()),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    const ITEM_GLYPH: char = '{';
    const ITEM_FG: (u8, u8, u8) = WHITE;
    const ITEM_BG: (u8, u8, u8) = BLACK;
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(ITEM_FG, ITEM_BG),
            glyph: to_cp437(ITEM_GLYPH),
        },
        Name("Dungeon Map".to_string()),
        ProvidesDungeonMap {},
    ));
}

/// Create a player entity at the given position
pub fn spawn_player(ecs: &mut World, pos: Point) {
    const PLAYER_HP: i32 = 10;
    const PLAYER_GLYPH: char = '@';
    const PLAYER_FG: (u8, u8, u8) = WHITE;
    const PLAYER_BG: (u8, u8, u8) = BLACK;
    const PLAYER_FOV: i32 = 8;
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
        FieldOfView::new(PLAYER_FOV),
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
